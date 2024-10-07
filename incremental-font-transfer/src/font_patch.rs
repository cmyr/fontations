//! Reads and applies font patches to font binaries.
//!
//! Font patch formats are defined as part of the incremental font transfer specification:
//! <https://w3c.github.io/IFT/Overview.html#font-patch-formats>
//!
//! Two main types of font patches are implemented:
//! 1. Table Keyed Patch - these patches contain a per table brotli binary patch to be applied
//!    to the input font.
//! 2. Glyph Keyed - these patches contain blobs of data associated with combinations of
//!    glyph id + table. The patch inserts these blobs into the table at the location for
//!    the corresponding glyph id.

use crate::patchmap::PatchEncoding;

use read_fonts::tables::ift::{CompatibilityId, TablePatchFlags};
use read_fonts::tables::ift::{TableKeyedPatch, TablePatch};
use read_fonts::types::Tag;
use read_fonts::{FontData, FontRef};
use read_fonts::{FontRead, ReadError};
use shared_brotli_patch_decoder::{shared_brotli_decode, DecodeError};
use std::collections::BTreeSet;
use write_fonts::FontBuilder;

/// A trait for types to which an incremental font transfer patch can be applied.
///
/// See: <https://w3c.github.io/IFT/Overview.html#font-patch-formats> for details on the format of patches.
pub trait IncrementalFontPatchBase {
    /// Apply an incremental font patch (<https://w3c.github.io/IFT/Overview.html#font-patch-formats>)
    ///
    /// Applies the patch to this base. In the base the patch is associated with the supplied
    /// expected_compatibility_id and has the specified encoding.
    ///
    /// Returns the byte data for the new font produced as a result of the patch application.
    fn apply_patch(
        &self,
        expected_compatibility_id: &CompatibilityId,
        patch_data: &[u8],
        encoding: PatchEncoding,
    ) -> Result<Vec<u8>, ReadError>;
}

trait IncrementalFontPatch {
    /// Applies this patch to the given font.
    ///
    /// In the font this patch is associated with the supplied compatibility_id.
    ///
    /// Returns the byte data for the new font produced as a result of the patch application.
    fn apply(
        &self,
        font: &FontRef,
        compatibility_id: &CompatibilityId,
    ) -> Result<Vec<u8>, ReadError>;
}

impl IncrementalFontPatchBase for FontRef<'_> {
    fn apply_patch(
        &self,
        compatibility_id: &CompatibilityId,
        patch_data: &[u8],
        encoding: PatchEncoding,
    ) -> Result<Vec<u8>, ReadError> {
        if self.table_data(Tag::new(b"IFT ")).is_none()
            && self.table_data(Tag::new(b"IFTX")).is_none()
        {
            // This base is not an incremental font, which is an error.
            // See: https://w3c.github.io/IFT/Overview.html#apply-table-keyed
            return Err(ReadError::ValidationError);
        }

        // TODO(garretrieger): Return a custom error type instead of ReadError?
        let patch = match encoding {
            PatchEncoding::TableKeyed { .. } => TableKeyedPatch::read(FontData::new(patch_data))?,
            PatchEncoding::GlyphKeyed => {
                todo!()
            }
        };

        patch.apply(self, compatibility_id)
    }
}

impl IncrementalFontPatchBase for &[u8] {
    fn apply_patch(
        &self,
        compatibility_id: &CompatibilityId,
        patch_data: &[u8],
        encoding: PatchEncoding,
    ) -> Result<Vec<u8>, ReadError> {
        let font_ref = FontRef::new(self)?;
        font_ref.apply_patch(compatibility_id, patch_data, encoding)
    }
}

impl IncrementalFontPatch for TableKeyedPatch<'_> {
    fn apply(
        &self,
        font: &FontRef,
        compatibility_id: &CompatibilityId,
    ) -> Result<Vec<u8>, ReadError> {
        if self.compatibility_id() != *compatibility_id {
            return Err(ReadError::ValidationError);
        }

        if self.format() != Tag::new(b"iftk") {
            return Err(ReadError::ValidationError);
        }

        const STREAM_START: u32 = 9;
        let mut font_builder = FontBuilder::new();
        let mut processed_tables = BTreeSet::<Tag>::new();
        // TODO(garretrieger): enforce a max combined size of all decoded tables? say something in the spec about this?
        for i in 0..self.patches_count() {
            let i = i as usize;
            let next = i + 1;

            let table_patch = self.patches().get(i)?;
            let (Some(offset), Some(next_offset)) =
                (self.patch_offsets().get(i), self.patch_offsets().get(next))
            else {
                return Err(ReadError::MalformedData("Missing patch offset."));
            };

            let offset = offset.get().to_u32();
            let next_offset = next_offset.get().to_u32();
            let Some(stream_length) = next_offset
                .checked_sub(offset)
                .and_then(|v| v.checked_sub(STREAM_START))
            // brotli stream starts at the (u32 tag + u8 flags + u32 length) = 9th byte
            else {
                // TODO(garretrieger): update spec to clarify this case is an error.
                return Err(ReadError::MalformedData(
                    "Patch offsets are not in sorted order.",
                ));
            };

            if stream_length as usize > table_patch.brotli_stream().len() {
                return Err(ReadError::OutOfBounds);
            }

            let tag = table_patch.tag();
            if !processed_tables.insert(tag) {
                // Table has already been processed.
                continue;
            }

            if table_patch.flags().contains(TablePatchFlags::DROP_TABLE) {
                // TODO(garretrieger): spec needs to be clarified on what happens when DROP_TABLE and REPLACE_TABLE are
                // both set.

                // Table will not be copied, skip any further processing.
                continue;
            }

            let replacement = table_patch.flags().contains(TablePatchFlags::REPLACE_TABLE);
            let new_table = apply_table_patch(font, table_patch, stream_length, replacement)?;
            font_builder.add_raw(tag, new_table);
        }

        font.table_directory
            .table_records()
            .iter()
            .map(|r| r.tag())
            .filter(|tag| !processed_tables.contains(tag))
            .filter_map(|tag| Some((tag, font.table_data(tag)?)))
            .for_each(|(tag, data)| {
                font_builder.add_raw(tag, data);
            });

        Ok(font_builder.build())
    }
}

fn apply_table_patch(
    font: &FontRef,
    table_patch: TablePatch,
    stream_length: u32,
    replacement: bool,
) -> Result<Vec<u8>, ReadError> {
    // TODO(garretrieger): spec needs to be clarified what happens when replacement = false, but the source table
    // does not exist in the font.
    let stream_length = stream_length as usize;
    let base_data = font.table_data(table_patch.tag());
    let stream = if table_patch.brotli_stream().len() >= stream_length {
        &table_patch.brotli_stream()[..stream_length]
    } else {
        // TODO(garretrieger): update spec to clarify this case is an error.
        return Err(ReadError::OutOfBounds);
    };
    let r = match (base_data, replacement) {
        (Some(base_data), false) => shared_brotli_decode(
            stream,
            Some(base_data.as_bytes()),
            table_patch.max_uncompressed_length() as usize,
        ),
        (None, false) => return Err(ReadError::ValidationError),
        _ => shared_brotli_decode(stream, None, table_patch.max_uncompressed_length() as usize),
    };

    r.map_err(|decode_error| match decode_error {
        DecodeError::InitFailure => ReadError::MalformedData("Failure to init brotli encoder."),
        DecodeError::InvalidStream => ReadError::MalformedData("Malformed brotli stream."),
        DecodeError::InvalidDictionary => ReadError::MalformedData("Malformed dictionary."),
        DecodeError::MaxSizeExceeded => ReadError::OutOfBounds,
        DecodeError::ExcessInputData => {
            ReadError::MalformedData("Input brotli stream has excess bytes.")
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use font_test_data::ift::{noop_table_keyed_patch, table_keyed_patch};
    use read_fonts::FontRef;
    use read_fonts::ReadError;
    use write_fonts::FontBuilder;

    const IFT_TABLE: &[u8] = "IFT PATCH MAP".as_bytes();
    const TABLE_1_FINAL_STATE: &[u8] = "hijkabcdeflmnohijkabcdeflmno\n".as_bytes();
    const TABLE_2_FINAL_STATE: &[u8] = "foobarbaz foobarbaz foobarbaz\n".as_bytes();
    const TABLE_3_FINAL_STATE: &[u8] = "foobaz\n".as_bytes();
    const TABLE_4_FINAL_STATE: &[u8] = "unchanged\n".as_bytes();

    fn compat_id() -> CompatibilityId {
        CompatibilityId::from_u32s([1, 2, 3, 4])
    }

    fn test_font() -> Vec<u8> {
        let mut font_builder = FontBuilder::new();
        font_builder.add_raw(Tag::new(b"IFT "), IFT_TABLE);
        font_builder.add_raw(Tag::new(b"tab1"), "abcdef\n".as_bytes());
        font_builder.add_raw(Tag::new(b"tab2"), "foobar\n".as_bytes());
        font_builder.add_raw(Tag::new(b"tab3"), "foobaz\n".as_bytes());
        font_builder.add_raw(Tag::new(b"tab4"), "unchanged\n".as_bytes());
        font_builder.build()
    }

    #[test]
    fn table_keyed_patch_test() {
        let r = test_font().as_slice().apply_patch(
            &&compat_id(),
            table_keyed_patch().as_slice(),
            PatchEncoding::TableKeyed {
                fully_invalidating: false,
            },
        );

        let font = r.unwrap();
        let font = FontRef::new(&font).unwrap();

        assert_eq!(font.table_directory.num_tables(), 4);

        assert_eq!(
            font.table_data(Tag::new(b"IFT ")).unwrap().as_bytes(),
            IFT_TABLE
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab1")).unwrap().as_bytes(),
            TABLE_1_FINAL_STATE
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab2")).unwrap().as_bytes(),
            TABLE_2_FINAL_STATE
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab4")).unwrap().as_bytes(),
            TABLE_4_FINAL_STATE
        );
    }

    #[test]
    fn noop_table_keyed_patch_test() {
        let r = test_font().as_slice().apply_patch(
            &compat_id(),
            noop_table_keyed_patch().as_slice(),
            PatchEncoding::TableKeyed {
                fully_invalidating: false,
            },
        );

        let font = r.unwrap();
        let font = FontRef::new(&font).unwrap();
        let expected_font = test_font();
        let expected_font = FontRef::new(&expected_font).unwrap();

        assert_eq!(
            font.table_directory.num_tables(),
            expected_font.table_directory.num_tables()
        );

        for t in expected_font.table_directory.table_records() {
            let data = font.table_data(t.tag()).unwrap();
            let expected_data = expected_font.table_data(t.tag()).unwrap();
            assert_eq!(data.as_bytes(), expected_data.as_bytes());
        }
    }

    #[test]
    fn table_keyed_patch_compat_id_mismatch() {
        assert_eq!(
            Err(ReadError::ValidationError),
            test_font().as_slice().apply_patch(
                &CompatibilityId::from_u32s([1, 2, 2, 4]),
                table_keyed_patch().as_slice(),
                PatchEncoding::TableKeyed {
                    fully_invalidating: false,
                },
            )
        );
    }

    #[test]
    fn table_keyed_patch_bad_top_level_tag() {
        let mut patch = table_keyed_patch();
        patch.write_at("tag", Tag::new(b"ifgk"));
        let patch = patch.as_slice();

        assert_eq!(
            Err(ReadError::ValidationError),
            test_font().as_slice().apply_patch(
                &compat_id(),
                patch,
                PatchEncoding::TableKeyed {
                    fully_invalidating: false,
                },
            )
        );
    }

    #[test]
    fn table_keyed_ignore_duplicates() {
        // add a duplicate entry requesting dropping of tab2,
        // should be ignored.
        let mut patch = table_keyed_patch();
        patch.write_at("patch[2]", Tag::new(b"tab2"));
        let patch = patch.as_slice();

        let r = test_font().as_slice().apply_patch(
            &compat_id(),
            patch,
            PatchEncoding::TableKeyed {
                fully_invalidating: false,
            },
        );

        let font = r.unwrap();
        let font = FontRef::new(&font).unwrap();

        assert_eq!(font.table_directory.num_tables(), 5);

        assert_eq!(
            font.table_data(Tag::new(b"IFT ")).unwrap().as_bytes(),
            IFT_TABLE
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab1")).unwrap().as_bytes(),
            TABLE_1_FINAL_STATE
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab2")).unwrap().as_bytes(),
            TABLE_2_FINAL_STATE
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab3")).unwrap().as_bytes(),
            TABLE_3_FINAL_STATE
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab4")).unwrap().as_bytes(),
            TABLE_4_FINAL_STATE
        );
    }

    #[test]
    fn table_keyed_patch_unsorted_offsets() {
        // reorder the offsets
        let mut patch = table_keyed_patch();

        let offset = patch.offset_for("patch[1]") as u32;
        patch.write_at("patch_off[0]", offset);

        let offset = patch.offset_for("patch[0]") as u32;
        patch.write_at("patch_off[1]", offset);

        let patch = patch.as_slice();

        assert_eq!(
            Err(ReadError::MalformedData(
                "Patch offsets are not in sorted order."
            )),
            test_font().as_slice().apply_patch(
                &compat_id(),
                patch,
                PatchEncoding::TableKeyed {
                    fully_invalidating: false,
                },
            )
        );
    }

    #[test]
    fn table_keyed_patch_out_of_bounds_offsets() {
        // set last offset out of bounds
        let mut patch = table_keyed_patch();
        let offset = (patch.offset_for("end") + 5) as u32;
        patch.write_at("patch_off[3]", offset);

        let patch = patch.as_slice();

        assert_eq!(
            Err(ReadError::OutOfBounds),
            test_font().as_slice().apply_patch(
                &compat_id(),
                patch,
                PatchEncoding::TableKeyed {
                    fully_invalidating: false,
                },
            )
        );
    }

    #[test]
    fn table_keyed_patch_drop_and_replace() {
        let mut patch = table_keyed_patch();
        patch.write_at("flags[2]", 3u8);
        let patch = patch.as_slice();

        // When DROP and REPLACE are both set DROP takes priority.
        let r = test_font().as_slice().apply_patch(
            &compat_id(),
            patch,
            PatchEncoding::TableKeyed {
                fully_invalidating: false,
            },
        );

        let font = r.unwrap();
        let font = FontRef::new(&font).unwrap();

        assert_eq!(font.table_directory.num_tables(), 4);

        assert_eq!(
            font.table_data(Tag::new(b"IFT ")).unwrap().as_bytes(),
            IFT_TABLE
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab1")).unwrap().as_bytes(),
            TABLE_1_FINAL_STATE
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab2")).unwrap().as_bytes(),
            TABLE_2_FINAL_STATE
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab4")).unwrap().as_bytes(),
            TABLE_4_FINAL_STATE
        );
    }

    #[test]
    fn table_keyed_patch_missing_table() {
        let mut patch = table_keyed_patch();
        patch.write_at("flags[1]", 0u8);
        patch.write_at("patch[1]", Tag::new(b"tab5"));
        let patch = patch.as_slice();

        assert_eq!(
            Err(ReadError::ValidationError),
            test_font().as_slice().apply_patch(
                &compat_id(),
                patch,
                PatchEncoding::TableKeyed {
                    fully_invalidating: false,
                },
            )
        );
    }

    #[test]
    fn table_keyed_replace_missing_table() {
        let mut patch = table_keyed_patch();
        patch.write_at("patch[1]", Tag::new(b"tab5"));
        let patch = patch.as_slice();

        let r = test_font().as_slice().apply_patch(
            &compat_id(),
            patch,
            PatchEncoding::TableKeyed {
                fully_invalidating: false,
            },
        );

        let font = r.unwrap();
        let font = FontRef::new(&font).unwrap();

        assert_eq!(font.table_directory.num_tables(), 5);

        assert_eq!(
            font.table_data(Tag::new(b"IFT ")).unwrap().as_bytes(),
            IFT_TABLE
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab1")).unwrap().as_bytes(),
            TABLE_1_FINAL_STATE
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab2")).unwrap().as_bytes(),
            "foobar\n".as_bytes()
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab4")).unwrap().as_bytes(),
            TABLE_4_FINAL_STATE
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab5")).unwrap().as_bytes(),
            TABLE_2_FINAL_STATE
        );
    }

    #[test]
    fn table_keyed_drop_missing_table() {
        let mut patch = table_keyed_patch();
        patch.write_at("patch[2]", Tag::new(b"tab5"));
        let patch = patch.as_slice();

        let r = test_font().as_slice().apply_patch(
            &compat_id(),
            patch,
            PatchEncoding::TableKeyed {
                fully_invalidating: false,
            },
        );

        let font = r.unwrap();
        let font = FontRef::new(&font).unwrap();

        assert_eq!(font.table_directory.num_tables(), 5);

        assert_eq!(
            font.table_data(Tag::new(b"IFT ")).unwrap().as_bytes(),
            IFT_TABLE
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab1")).unwrap().as_bytes(),
            TABLE_1_FINAL_STATE
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab2")).unwrap().as_bytes(),
            TABLE_2_FINAL_STATE,
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab3")).unwrap().as_bytes(),
            TABLE_3_FINAL_STATE
        );
        assert_eq!(
            font.table_data(Tag::new(b"tab4")).unwrap().as_bytes(),
            TABLE_4_FINAL_STATE
        );
    }

    #[test]
    fn table_keyed_patch_uncompressed_len_too_small() {
        let mut patch = table_keyed_patch();
        patch.write_at("decompressed_len[0]", 28u32);
        let patch = patch.as_slice();

        assert_eq!(
            Err(ReadError::OutOfBounds),
            test_font().as_slice().apply_patch(
                &compat_id(),
                patch,
                PatchEncoding::TableKeyed {
                    fully_invalidating: false,
                },
            )
        );
    }
}
