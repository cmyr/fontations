//! impl subset() for gvar table
use std::mem::size_of;

use crate::{estimate_subset_table_size, Plan, Subset, SubsetError, SubsetFlags};

use write_fonts::{
    read::{tables::gvar::Gvar, types::GlyphId, FontRef, TopLevelTable},
    FontBuilder,
};

const FIXED_HEADER_SIZE: u32 = 20;
// reference: subset() for gvar table in harfbuzz
// https://github.com/harfbuzz/harfbuzz/blob/63d09dbefcf7ad9f794ca96445d37b6d8c3c9124/src/hb-ot-var-gvar-table.hh#L411
impl Subset for Gvar<'_> {
    fn subset(
        &self,
        plan: &Plan,
        font: &FontRef,
        builder: &mut FontBuilder,
    ) -> Result<(), SubsetError> {
        let cap = estimate_subset_table_size(font, Gvar::TAG, plan);
        let mut out: Vec<u8> = Vec::with_capacity(cap);

        //table header: from version to sharedTuplesOffset
        out.extend_from_slice(self.offset_data().as_bytes().get(0..12).unwrap());

        // glyphCount
        let num_glyphs = plan.num_output_glyphs.min(0xFFFF) as u16;
        out.extend_from_slice(&num_glyphs.to_be_bytes());

        let subset_data_size: u32 = plan
            .new_to_old_gid_list
            .iter()
            .filter_map(|x| {
                if x.0 == GlyphId::NOTDEF
                    && !plan
                        .subset_flags
                        .contains(SubsetFlags::SUBSET_FLAGS_NOTDEF_OUTLINE)
                {
                    return None;
                }
                self.data_for_gid(x.0).ok().map(|data| data.len() as u32)
            })
            .sum();

        // According to the spec: If the short format (Offset16) is used for offsets, the value stored is the offset divided by 2.
        // So the maximum subset data size that could use short format should be 2 * 0xFFFFu, which is 0x1FFFE
        let long_offset = if subset_data_size > 0x1FFFE_u32 {
            1_u16
        } else {
            0_u16
        };
        // flags
        out.extend_from_slice(&long_offset.to_be_bytes());

        let out = if long_offset > 0 {
            subset_with_offset_type::<u32>(self, plan, num_glyphs, out)
        } else {
            subset_with_offset_type::<u16>(self, plan, num_glyphs, out)
        }?;

        builder.add_raw(Gvar::TAG, out);
        Ok(())
    }
}

fn subset_with_offset_type<OffsetType: GvarOffset>(
    gvar: &Gvar<'_>,
    plan: &Plan,
    num_glyphs: u16,
    mut out: Vec<u8>,
) -> Result<Vec<u8>, SubsetError> {
    // calculate sharedTuplesOffset
    // shared tuples array follow the GlyphVariationData offsets array at the end of the 'gvar' header.
    let off_size = size_of::<OffsetType>();

    let glyph_var_data_offset_array_size = (num_glyphs as u32 + 1) * off_size as u32;
    let shared_tuples_offset =
        if gvar.shared_tuple_count() == 0 || gvar.shared_tuples_offset().is_null() {
            0_u32
        } else {
            FIXED_HEADER_SIZE + glyph_var_data_offset_array_size
        };

    //update sharedTuplesOffset, which is of Offset32 type and byte position in gvar is 8..12
    out.get_mut(8..12)
        .unwrap()
        .copy_from_slice(&shared_tuples_offset.to_be_bytes());

    // calculate glyphVariationDataArrayOffset: put the glyphVariationData at last in the table
    let shared_tuples_size = 2 * gvar.axis_count() * gvar.shared_tuple_count();
    let glyph_var_data_offset =
        FIXED_HEADER_SIZE + glyph_var_data_offset_array_size + shared_tuples_size as u32;
    out.extend_from_slice(&glyph_var_data_offset.to_be_bytes());

    //pre-allocate glyphVariationDataOffsets array
    let mut start_idx = out.len();
    let new_len = start_idx + (num_glyphs as usize + 1) * off_size;
    out.resize(new_len, 0);

    // shared tuples array
    if shared_tuples_offset > 0 {
        let offset = gvar.shared_tuples_offset().to_u32() as usize;
        let shared_tuples_data = gvar
            .offset_data()
            .as_bytes()
            .get(offset..offset + shared_tuples_size as usize)
            .unwrap();
        out.extend_from_slice(shared_tuples_data);
    }

    // GlyphVariationData table array, also update glyphVariationDataOffsets
    start_idx += off_size;

    let mut glyph_offset = 0_u32;
    let mut last = 0;
    for (new_gid, old_gid) in plan.new_to_old_gid_list.iter().filter(|x| {
        x.0 != GlyphId::NOTDEF
            || plan
                .subset_flags
                .contains(SubsetFlags::SUBSET_FLAGS_NOTDEF_OUTLINE)
    }) {
        let last_gid = last;
        for _ in last_gid..new_gid.to_u32() {
            copy_offset_value(
                &mut out,
                &mut start_idx,
                OffsetType::stored_value(glyph_offset),
            );
            last += 1;
        }

        if let Ok(glyph_var_data) = gvar.data_for_gid(*old_gid) {
            out.extend_from_slice(glyph_var_data.as_bytes());
            glyph_offset += glyph_var_data.len() as u32;
        };

        copy_offset_value(
            &mut out,
            &mut start_idx,
            OffsetType::stored_value(glyph_offset),
        );

        last += 1;
    }

    for _ in last..plan.num_output_glyphs as u32 {
        copy_offset_value(
            &mut out,
            &mut start_idx,
            OffsetType::stored_value(glyph_offset),
        );
    }

    Ok(out)
}

trait GvarOffset {
    fn stored_value(val: u32) -> Self;
    fn copy_be_bytes(&self, f: impl FnOnce(&[u8]));
}

impl GvarOffset for u16 {
    fn stored_value(val: u32) -> u16 {
        (val / 2) as u16
    }

    fn copy_be_bytes(&self, f: impl FnOnce(&[u8])) {
        f(&self.to_be_bytes())
    }
}

impl GvarOffset for u32 {
    fn stored_value(val: u32) -> u32 {
        val
    }

    fn copy_be_bytes(&self, f: impl FnOnce(&[u8])) {
        f(&self.to_be_bytes())
    }
}

fn copy_offset_value<T: GvarOffset>(out: &mut [u8], idx: &mut usize, glyph_offset: T) {
    let offset_size = size_of::<T>();
    let f = |x: &_| {
        out.get_mut(*idx..*idx + offset_size)
            .unwrap()
            .copy_from_slice(x)
    };

    glyph_offset.copy_be_bytes(f);

    *idx += offset_size;
}
