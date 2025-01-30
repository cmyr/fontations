//! impl subset() for CBLC
use crate::offset::SerializeSubset;
use crate::serialize::{OffsetWhence, SerializeErrorFlags, Serializer};
use crate::{Plan, Subset, SubsetError, SubsetTable};
use std::cmp::Ordering;
use skrifa::raw::collections::IntSet;
use write_fonts::types::FixedSize;
use write_fonts::{
    read::{
        tables::{
            bitmap::{BitmapSize, IndexSubtable, IndexSubtable1, IndexSubtable3, IndexSubtableList, IndexSubtableRecord},
            cbdt::Cbdt,
            cblc::Cblc,
        },
        FontData, FontRef, TableProvider, TopLevelTable,
    },
    types::{GlyphId, Offset32},
    FontBuilder,
};

// reference: subset() for CBLC in harfbuzz
// <https://github.com/harfbuzz/harfbuzz/blob/6d8035a99c279e32183ad063f0de201ef1b2f05c/src/OT/Color/CBDT/CBDT.hh#L995>
impl Subset for Cblc<'_> {
    fn subset(
        &self,
        plan: &Plan,
        font: &FontRef,
        s: &mut Serializer,
        builder: &mut FontBuilder,
    ) -> Result<(), SubsetError> {
        let cbdt = font
            .cbdt()
            .or(Err(SubsetError::SubsetTableError(Cbdt::TAG)))?;

        s.embed(self.major_version())
            .map_err(|_| SubsetError::SubsetTableError(Cblc::TAG))?;
        s.embed(self.minor_version())
            .map_err(|_| SubsetError::SubsetTableError(Cblc::TAG))?;

        let mut num_sizes: u32 = 0;
        let num_sizes_pos = s
            .embed(num_sizes)
            .map_err(|_| SubsetError::SubsetTableError(Cblc::TAG))?;

        let bitmapsize_records = self.bitmap_sizes();
        let bitmapsize_bytes = self
            .offset_data()
            .as_bytes()
            .get(self.shape().bitmap_sizes_byte_range())
            .unwrap();
        let mut cbdt_out = Vec::with_capacity(cbdt.offset_data().len());

        for (idx, bitmap_size_table) in bitmapsize_records.iter().enumerate() {
            let old_len = cbdt_out.len();
            let snap = s.snapshot();

            let start = idx * BitmapSize::RAW_BYTE_LEN;
            let src_bytes = bitmapsize_bytes
                .get(start..start + BitmapSize::RAW_BYTE_LEN)
                .unwrap();
            if bitmap_size_table
                .subset(plan, s, &(&cbdt, &mut cbdt_out, src_bytes))
                .is_err()
            {
                s.revert_snapshot(snap);
                cbdt_out.truncate(old_len);
                continue;
            };
            num_sizes += 1;
        }

        if num_sizes == 0 || cbdt_out.is_empty() {
            return Err(SubsetError::SubsetTableError(Cblc::TAG));
        }

        s.copy_assign(num_sizes_pos, num_sizes);
        builder.add_raw(Cbdt::TAG, cbdt_out);
        Ok(())
    }
}

struct CblcBitmapSizeSubsetContext<'a> {
    cbdt_out: &'a mut Vec<u8>,
}

impl<'a> SubsetTable<'a> for BitmapSize {
    type ArgsForSubset = (&'a Cbdt<'a>, &'a mut Vec<u8>, &'a [u8]);
    fn subset(
        &self,
        plan: &Plan,
        s: &mut Serializer,
        args: &(&Cbdt, &mut Vec<u8>, &[u8]),
    ) -> Result<(), SerializeErrorFlags> {
        let cbdt = args.0;
        let src_bytes = args.2;
        let offset_pos = s.embed_bytes(src_bytes)?;

        let Ok(index_subtable_list) = self.index_subtable_list(cbdt.offset_data()) else {
            return Err(SerializeErrorFlags::SERIALIZE_ERROR_READ_ERROR);
        };

        Offset32::serialize_subset(&index_subtable_list, s, plan, args, offset_pos)?;
        // update some values
        s.copy_assign(pos, obj);
        Ok(())
    }
}

impl<'a> SubsetTable<'_> for IndexSubtableList<'a> {
    type ArgsForSubset = ();
    fn subset(
        &self,
        plan: &Plan,
        s: &mut Serializer,
        args: &Self::ArgsForSubset,
    ) -> Result<(), SerializeErrorFlags> {
        let records = self.index_subtable_records();
        for record in records.iter() {
            record.subset()?;
        }
        Ok(())
    }
}

impl SubsetTable<'_> for IndexSubtableRecord {
    type ArgsForSubset = ();
    fn subset(
            &self,
            plan: &Plan,
            s: &mut Serializer,
            args: &Self::ArgsForSubset,
        ) -> Result<(), SerializeErrorFlags> {
        let gid_min = self.first_glyph_index();
        let gid_max = self.last_glyph_index();

        let mut retained_glyphs = IntSet::empty();
        retained_glyphs.insert_range(GlyphId::from(gid_min)..=GlyphId::from(gid_max));
        retained_glyphs.intersect(&plan.glyphset);

        if retained_glyphs.is_empty() {
            return Err(SerializeErrorFlags::SERIALIZE_ERROR_EMPTY);
        }

        let snap = s.snapshot();
        s.embed(retained_glyphs.first().unwrap().to_u32() as u16)?;
        s.embed(retained_glyphs.last().unwrap().to_u32() as u16)?;
        //update gid_min and gid_max in bitmapsize

        let offset_pos = s.embed(0_u32)?;
        let Ok(t) = self.index_subtable(list_font_data) else {
            return Err(SerializeErrorFlags::SERIALIZE_ERROR_READ_ERROR);
        };

        if let Err(e) = Offset32::serialize_subset(&t, s, plan, args, offset_pos) {
            s.revert_snapshot(snap);
            return Err(e);
        }
        Ok(())
        
    }
}

impl SubsetTable<'_> for IndexSubtable {
    type ArgsForSubset = ();
    fn subset(
            &self,
            plan: &Plan,
            s: &mut Serializer,
            args: &Self::ArgsForSubset,
        ) -> Result<(), SerializeErrorFlags> {
            match self {
                Self::Format1(item) => item.subset(),
                Self::Format2(item) => item.subset(),
                Self::Format3(item) => item.subset(),
                Self::Format4(item) => item.subset(),
                Self::Format5(item) => item.subset(),
            }
    }
}

impl<'a> SubsetTable<'_> for IndexSubtable1<'a> {
    //pass in image_offset and cbdt_out, and retained glyph_set, original first glyph_index
    type ArgsForSubset = (u32, &'a IntSet<GlyphId>, GlyphId, &'a mut GlyphId, &'a mut GlyphId);
    fn subset(
            &self,
            plan: &Plan,
            s: &mut Serializer,
            args: &Self::ArgsForSubset,
        ) -> Result<(), SerializeErrorFlags> {
        let init_len = s.length();
        s.embed(self.index_format())?;
        s.embed(self.image_format())?;

        // pass in image offset
        let image_offset = args.0;
        s.embed(image_offset)?;

        size += s.length() - init_len;

        let glyphs = args.1;
        let src_first_glyph_index = args.2.to_u32() as usize;
        let src_offsets = self.sbit_offsets();

        // find the last glyph that has image_data
        let end_glyph = None;
        for gid in glyphs.iter().rev() {
            let idx = gid.to_u32() as usize - src_first_glyph_index;
            let offset_start = src_offsets[idx].get(); 
            let offset_end = src_offsets[idx+1].get();

            if offset_end > offset_start {
                end_glyph = Some(gid);
                break;
            }
        }

        if end_glyph.is_none() {
            // revert snapshot
            // revert CBDT_out
            return Err(SerializeErrorFlags::SERIALIZE_ERROR_EMPTY);
        }

        let mut start_glyph = None;
        for gid in glyphs.iter() {
            let idx = gid.to_u32() as usize - src_first_glyph_index;
            let offset_start = src_offsets[idx].get(); 
            let offset_end = src_offsets[idx+1].get();

            if offset_end <= offset_start {
                if start_glyph.is_none() {
                    continue;
                } else {
                    s.embed(offset)?;
                }
            } else {
                if start_glyph.is_none() {
                    start_glyph = Some(gid);
                }
                //copy glyph image data into cbdt_out
                offset += len;
                s.embed(offset)?;
            }

            if gid == end_glyph.unwrap() {
                break;
            }
        }
        Ok(())
    }
}

impl<'a> SubsetTable<'_> for IndexSubtable3<'a> {
    //pass in image_offset and cbdt_out, and retained glyph_set, original first glyph_index
    type ArgsForSubset = (u32, &'a IntSet<GlyphId>, GlyphId, &'a mut GlyphId, &'a mut GlyphId);
    fn subset(
            &self,
            plan: &Plan,
            s: &mut Serializer,
            args: &Self::ArgsForSubset,
        ) -> Result<(), SerializeErrorFlags> {
        let init_len = s.length();
        s.embed(self.index_format())?;
        s.embed(self.image_format())?;

        // pass in image offset
        let image_offset = args.0;
        s.embed(image_offset)?;

        size += s.length() - init_len;

        let glyphs = args.1;
        let src_first_glyph_index = args.2.to_u32() as usize;
        let src_offsets = self.sbit_offsets();

        // find the last glyph that has image_data
        let end_glyph = None;
        for gid in glyphs.iter().rev() {
            let idx = gid.to_u32() as usize - src_first_glyph_index;
            let offset_start = src_offsets[idx].get(); 
            let offset_end = src_offsets[idx+1].get();

            if offset_end > offset_start {
                end_glyph = Some(gid);
                break;
            }
        }

        if end_glyph.is_none() {
            // revert snapshot
            // revert CBDT_out
            return Err(SerializeErrorFlags::SERIALIZE_ERROR_EMPTY);
        }

        let mut start_glyph = None;
        for gid in glyphs.iter() {
            let idx = gid.to_u32() as usize - src_first_glyph_index;
            let offset_start = src_offsets[idx].get(); 
            let offset_end = src_offsets[idx+1].get();

            if offset_end <= offset_start {
                if start_glyph.is_none() {
                    continue;
                } else {
                    s.embed(offset)?;
                }
            } else {
                if start_glyph.is_none() {
                    start_glyph = Some(gid);
                }
                //copy glyph image data into cbdt_out
                offset += len;
                s.embed(offset)?;
            }

            if gid == end_glyph.unwrap() {
                break;
            }
        }

        //pad for format3
        Ok(())
    }
}

fn build_lookup(records: &[IndexSubtableRecord], subtable_list_fontdata: FontData, plan: &Plan) {
    let start_glyph_is_set = false;
    let num_records = records.len();
    let num_search = 32 - (num_records as u32).leading_zeros();
    if num_records > plan.new_to_old_gid_list.len() * num_search as usize {
        for (new_gid, old_gid) in plan.new_to_old_gid_list.iter() {
            let Ok(idx) = records.binary_search_by(|r| {
                if r.first_glyph_index() > *old_gid {
                    Ordering::Greater
                } else if r.last_glyph_index() < *old_gid {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }) else {
                continue;
            };

            let record = records[idx];
            let Ok(index_subtable) = record.index_subtable(subtable_list_fontdata) else {
                continue;
            };

            let idx = old_gid.to_u32() - record.first_glyph_index().to_u32();
            if !has_image_data(&index_subtable, idx as usize) {
                continue;
            }

        }
    } else {
        for record in records.iter() {
            let Ok(index_subtable) = record.index_subtable(subtable_list_fontdata) else {
                continue;
            };
            let start_gid = record.first_glyph_index().to_u32();
            let last_gid = record.last_glyph_index();
            if start_gid == 0 {
                for old_gid in plan.glyphset.iter() {
                    if old_gid > last_gid {
                        break;
                    }
        
                    let idx = old_gid.to_u32() - start_gid;
                    if !has_image_data(&index_subtable, idx as usize) {
                        continue;
                    }
                }
            } else {
                let it = plan.glyphset.iter_after(GlyphId::from(start_gid - 1));
                for old_gid in it {
                    if old_gid > last_gid {
                        break;
                    }
                    let idx = old_gid.to_u32() - start_gid;
                    if !has_image_data(&index_subtable, idx as usize) {
                        continue;
                    }
                }
            }
        }
    }
}

fn has_image_data(table: &IndexSubtable, idx: usize) -> bool {
    if let IndexSubtable::Format1(item) = table {
        let sbit_offsets = item.sbit_offsets();
        let Some(offset_start) = sbit_offsets.get(idx) else {
            return false;
        };

        let Some(offset_end) = sbit_offsets.get(idx+1) else {
            return false;
        };

        offset_end > offset_start
    } else if let IndexSubtable::Format3(item) = table {
        let sbit_offsets = item.sbit_offsets();
        let Some(offset_start) = sbit_offsets.get(idx) else {
            return false;
        };

        let Some(offset_end) = sbit_offsets.get(idx+1) else {
            return false;
        };
        offset_end > offset_start
    } else {
        false
    }
}
