// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [MVAR (Metrics Variations)](https://docs.microsoft.com/en-us/typography/opentype/spec/mvar) table
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mvar {
    /// Major version number of the horizontal metrics variations table — set to 1.
    /// Minor version number of the horizontal metrics variations table — set to 0.
    pub version: MajorMinor,
    /// The size in bytes of each value record — must be greater than zero.
    pub value_record_size: u16,
    /// The number of value records — may be zero.
    pub value_record_count: u16,
    /// Offset in bytes from the start of this table to the item variation store table. If valueRecordCount is zero, set to zero; if valueRecordCount is greater than zero, must be greater than zero.
    pub item_variation_store: NullableOffsetMarker<ItemVariationStore>,
    /// Array of value records that identify target items and the associated delta-set index for each. The valueTag records must be in binary order of their valueTag field.
    pub value_records: Vec<ValueRecord>,
}

impl FontWrite for Mvar {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.version.write_into(writer);
        (0 as u16).write_into(writer);
        self.value_record_size.write_into(writer);
        self.value_record_count.write_into(writer);
        self.item_variation_store.write_into(writer);
        self.value_records.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::TopLevel(Mvar::TAG)
    }
}

impl Validate for Mvar {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Mvar", |ctx| {
            ctx.in_field("item_variation_store", |ctx| {
                self.item_variation_store.validate_impl(ctx);
            });
            ctx.in_field("value_records", |ctx| {
                if self.value_records.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.value_records.validate_impl(ctx);
            });
        })
    }
}

impl TopLevelTable for Mvar {
    const TAG: Tag = Tag::new(b"MVAR");
}

impl<'a> FromObjRef<read_fonts::tables::mvar::Mvar<'a>> for Mvar {
    fn from_obj_ref(obj: &read_fonts::tables::mvar::Mvar<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        Mvar {
            version: obj.version(),
            value_record_size: obj.value_record_size(),
            value_record_count: obj.value_record_count(),
            item_variation_store: obj.item_variation_store().to_owned_table(),
            value_records: obj.value_records().to_owned_obj(offset_data),
        }
    }
}

#[allow(clippy::needless_lifetimes)]
impl<'a> FromTableRef<read_fonts::tables::mvar::Mvar<'a>> for Mvar {}

impl<'a> FontRead<'a> for Mvar {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::mvar::Mvar as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [ValueRecord](https://learn.microsoft.com/en-us/typography/opentype/spec/mvar#table-formats) metrics variation record
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValueRecord {
    /// Four-byte tag identifying a font-wide measure.
    pub value_tag: Tag,
    /// A delta-set outer index — used to select an item variation data subtable within the item variation store.
    pub delta_set_outer_index: u16,
    /// A delta-set inner index — used to select a delta-set row within an item variation data subtable.
    pub delta_set_inner_index: u16,
}

impl ValueRecord {
    /// Construct a new `ValueRecord`
    pub fn new(value_tag: Tag, delta_set_outer_index: u16, delta_set_inner_index: u16) -> Self {
        Self {
            value_tag,
            delta_set_outer_index,
            delta_set_inner_index,
        }
    }
}

impl FontWrite for ValueRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.value_tag.write_into(writer);
        self.delta_set_outer_index.write_into(writer);
        self.delta_set_inner_index.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("ValueRecord")
    }
}

impl Validate for ValueRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::mvar::ValueRecord> for ValueRecord {
    fn from_obj_ref(obj: &read_fonts::tables::mvar::ValueRecord, _: FontData) -> Self {
        ValueRecord {
            value_tag: obj.value_tag(),
            delta_set_outer_index: obj.delta_set_outer_index(),
            delta_set_inner_index: obj.delta_set_inner_index(),
        }
    }
}
