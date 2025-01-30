//! impl subset() for CBLC
use crate::serialize::{OffsetWhence, SerializeErrorFlags, Serializer};
use crate::{Plan, Subset, SubsetError, SubsetTable};
use write_fonts::types::FixedSize;
use write_fonts::{
    read::{
        tables::{bitmap::BitmapSize, cbdt::Cbdt, cblc::Cblc},
        FontRef, TableProvider, TopLevelTable,
    },
    types::Offset32,
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
        let src_bytes = args.2;
        let offset_pos = s.embed_bytes(src_bytes)?;

        s.push()?;
        match subset_index_subtable_list(s, plan) {
            Ok(()) => {
                let Some(obj_idx) = s.pop_pack(true) else {
                    return Err(s.error());
                };
                s.add_link(
                    offset_pos..offset_pos + Offset32::RAW_BYTE_LEN,
                    obj_idx,
                    OffsetWhence::Head,
                    0,
                    false,
                );
            }
            Err(e) => {
                s.pop_discard();
                return Err(e);
            }
        }

        //update some values
        s.copy_assign(pos, obj);
        Ok(())
    }
}

fn subset_index_subtable_list(s: &mut Serializer, plan: &Plan) -> Result<(), SerializeErrorFlags> {
    let mut start_glyph = false;
    for (new_gid, old_gid) in plan.new_to_old_gid_list.iter() {
        let record = find_table();
    }

    Ok(())
}
