// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IFT {}

impl IFT {
    /// Construct a new `IFT`
    pub fn new() -> Self {
        Self {}
    }
}

impl Validate for IFT {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl TopLevelTable for IFT {
    const TAG: Tag = Tag::new(b"IFT ");
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IFTX {}

impl IFTX {
    /// Construct a new `IFTX`
    pub fn new() -> Self {
        Self {}
    }
}

impl Validate for IFTX {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl TopLevelTable for IFTX {
    const TAG: Tag = Tag::new(b"IFTX");
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Ift {
    Format1(PatchMapFormat1),
    Format2(PatchMapFormat2),
}

impl Ift {
    /// Construct a new `PatchMapFormat1` subtable
    #[allow(clippy::too_many_arguments)]
    pub fn format_1(
        compatibility_id: Vec<u32>,
        max_entry_index: u16,
        glyph_count: u32,
        glyph_map: GlyphMap,
        feature_map: Option<FeatureMap>,
        applied_entries_bitmap: Vec<u8>,
        uri_template_length: u16,
        uri_template: Vec<u8>,
        patch_encoding: u8,
    ) -> Self {
        Self::Format1(PatchMapFormat1::new(
            compatibility_id,
            max_entry_index,
            glyph_count,
            glyph_map,
            feature_map,
            applied_entries_bitmap,
            uri_template_length,
            uri_template,
            patch_encoding,
        ))
    }

    /// Construct a new `PatchMapFormat2` subtable
    pub fn format_2(todo: u32) -> Self {
        Self::Format2(PatchMapFormat2::new(todo))
    }
}

impl Default for Ift {
    fn default() -> Self {
        Self::Format1(Default::default())
    }
}

impl FontWrite for Ift {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Format1(item) => item.write_into(writer),
            Self::Format2(item) => item.write_into(writer),
        }
    }
    fn table_type(&self) -> TableType {
        match self {
            Self::Format1(item) => item.table_type(),
            Self::Format2(item) => item.table_type(),
        }
    }
}

impl Validate for Ift {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        match self {
            Self::Format1(item) => item.validate_impl(ctx),
            Self::Format2(item) => item.validate_impl(ctx),
        }
    }
}

impl FromObjRef<read_fonts::tables::ift::Ift<'_>> for Ift {
    fn from_obj_ref(obj: &read_fonts::tables::ift::Ift, _: FontData) -> Self {
        use read_fonts::tables::ift::Ift as ObjRefType;
        match obj {
            ObjRefType::Format1(item) => Ift::Format1(item.to_owned_table()),
            ObjRefType::Format2(item) => Ift::Format2(item.to_owned_table()),
        }
    }
}

impl FromTableRef<read_fonts::tables::ift::Ift<'_>> for Ift {}

impl<'a> FontRead<'a> for Ift {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::ift::Ift as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

impl From<PatchMapFormat1> for Ift {
    fn from(src: PatchMapFormat1) -> Ift {
        Ift::Format1(src)
    }
}

impl From<PatchMapFormat2> for Ift {
    fn from(src: PatchMapFormat2) -> Ift {
        Ift::Format2(src)
    }
}

/// [Patch Map Format Format 1](https://w3c.github.io/IFT/Overview.html#patch-map-format-1)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatchMapFormat1 {
    /// Unique ID that identifies compatible patches.
    pub compatibility_id: Vec<u32>,
    /// Number of entries and glyphs that are mapped.
    pub max_entry_index: u16,
    pub glyph_count: u32,
    /// Sub table that maps glyph ids to entry indices.
    pub glyph_map: OffsetMarker<GlyphMap, WIDTH_32>,
    /// Sub table that maps feature and glyph ids to entry indices.
    pub feature_map: NullableOffsetMarker<FeatureMap, WIDTH_32>,
    pub applied_entries_bitmap: Vec<u8>,
    pub uri_template_length: u16,
    pub uri_template: Vec<u8>,
    /// Patch format number for patches referenced by this mapping.
    pub patch_encoding: u8,
}

impl PatchMapFormat1 {
    /// Construct a new `PatchMapFormat1`
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        compatibility_id: Vec<u32>,
        max_entry_index: u16,
        glyph_count: u32,
        glyph_map: GlyphMap,
        feature_map: Option<FeatureMap>,
        applied_entries_bitmap: Vec<u8>,
        uri_template_length: u16,
        uri_template: Vec<u8>,
        patch_encoding: u8,
    ) -> Self {
        Self {
            compatibility_id: compatibility_id.into_iter().map(Into::into).collect(),
            max_entry_index,
            glyph_count,
            glyph_map: glyph_map.into(),
            feature_map: feature_map.into(),
            applied_entries_bitmap: applied_entries_bitmap.into_iter().map(Into::into).collect(),
            uri_template_length,
            uri_template: uri_template.into_iter().map(Into::into).collect(),
            patch_encoding,
        }
    }
}

impl FontWrite for PatchMapFormat1 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u8).write_into(writer);
        (0 as u32).write_into(writer);
        self.compatibility_id.write_into(writer);
        self.max_entry_index.write_into(writer);
        self.glyph_count.write_into(writer);
        self.glyph_map.write_into(writer);
        self.feature_map.write_into(writer);
        self.applied_entries_bitmap.write_into(writer);
        self.uri_template_length.write_into(writer);
        self.uri_template.write_into(writer);
        self.patch_encoding.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("PatchMapFormat1")
    }
}

impl Validate for PatchMapFormat1 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("PatchMapFormat1", |ctx| {
            ctx.in_field("glyph_map", |ctx| {
                self.glyph_map.validate_impl(ctx);
            });
            ctx.in_field("feature_map", |ctx| {
                self.feature_map.validate_impl(ctx);
            });
            ctx.in_field("uri_template", |ctx| {
                if self.uri_template.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::ift::PatchMapFormat1<'a>> for PatchMapFormat1 {
    fn from_obj_ref(obj: &read_fonts::tables::ift::PatchMapFormat1<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        PatchMapFormat1 {
            compatibility_id: obj.compatibility_id().to_owned_obj(offset_data),
            max_entry_index: obj.max_entry_index(),
            glyph_count: obj.glyph_count(),
            glyph_map: obj.glyph_map().to_owned_table(),
            feature_map: obj.feature_map().to_owned_table(),
            applied_entries_bitmap: obj.applied_entries_bitmap().to_owned_obj(offset_data),
            uri_template_length: obj.uri_template_length(),
            uri_template: obj.uri_template().to_owned_obj(offset_data),
            patch_encoding: obj.patch_encoding(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::ift::PatchMapFormat1<'a>> for PatchMapFormat1 {}

impl<'a> FontRead<'a> for PatchMapFormat1 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::ift::PatchMapFormat1 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GlyphMap {
    pub first_mapped_glyph: u16,
}

impl GlyphMap {
    /// Construct a new `GlyphMap`
    pub fn new(first_mapped_glyph: u16) -> Self {
        Self { first_mapped_glyph }
    }
}

impl FontWrite for GlyphMap {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.first_mapped_glyph.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("GlyphMap")
    }
}

impl Validate for GlyphMap {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::ift::GlyphMap<'a>> for GlyphMap {
    fn from_obj_ref(obj: &read_fonts::tables::ift::GlyphMap<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        GlyphMap {
            first_mapped_glyph: obj.first_mapped_glyph(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::ift::GlyphMap<'a>> for GlyphMap {}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeatureMap {
    pub feature_count: u16,
    pub entry_map_data: Vec<u8>,
}

impl FeatureMap {
    /// Construct a new `FeatureMap`
    pub fn new(feature_count: u16, entry_map_data: Vec<u8>) -> Self {
        Self {
            feature_count,
            entry_map_data: entry_map_data.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for FeatureMap {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.feature_count.write_into(writer);
        self.entry_map_data.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("FeatureMap")
    }
}

impl Validate for FeatureMap {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::ift::FeatureMap<'a>> for FeatureMap {
    fn from_obj_ref(obj: &read_fonts::tables::ift::FeatureMap<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        FeatureMap {
            feature_count: obj.feature_count(),
            entry_map_data: obj.entry_map_data().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::ift::FeatureMap<'a>> for FeatureMap {}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeatureRecord {
    pub feature_tag: Tag,
}

impl FeatureRecord {
    /// Construct a new `FeatureRecord`
    pub fn new(feature_tag: Tag) -> Self {
        Self { feature_tag }
    }
}

impl FontWrite for FeatureRecord {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.feature_tag.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("FeatureRecord")
    }
}

impl Validate for FeatureRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::ift::FeatureRecord> for FeatureRecord {
    fn from_obj_ref(obj: &read_fonts::tables::ift::FeatureRecord, offset_data: FontData) -> Self {
        FeatureRecord {
            feature_tag: obj.feature_tag(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntryMapRecord {}

impl EntryMapRecord {
    /// Construct a new `EntryMapRecord`
    pub fn new() -> Self {
        Self {}
    }
}

impl FontWrite for EntryMapRecord {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {}
    fn table_type(&self) -> TableType {
        TableType::Named("EntryMapRecord")
    }
}

impl Validate for EntryMapRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::ift::EntryMapRecord> for EntryMapRecord {
    fn from_obj_ref(obj: &read_fonts::tables::ift::EntryMapRecord, offset_data: FontData) -> Self {
        EntryMapRecord {}
    }
}

/// [Patch Map Format Format 2](https://w3c.github.io/IFT/Overview.html#patch-map-format-1)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatchMapFormat2 {
    pub todo: u32,
}

impl PatchMapFormat2 {
    /// Construct a new `PatchMapFormat2`
    pub fn new(todo: u32) -> Self {
        Self { todo }
    }
}

impl FontWrite for PatchMapFormat2 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (2 as u8).write_into(writer);
        self.todo.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("PatchMapFormat2")
    }
}

impl Validate for PatchMapFormat2 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::ift::PatchMapFormat2<'a>> for PatchMapFormat2 {
    fn from_obj_ref(obj: &read_fonts::tables::ift::PatchMapFormat2<'a>, _: FontData) -> Self {
        PatchMapFormat2 { todo: obj.todo() }
    }
}

impl<'a> FromTableRef<read_fonts::tables::ift::PatchMapFormat2<'a>> for PatchMapFormat2 {}

impl<'a> FontRead<'a> for PatchMapFormat2 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::ift::PatchMapFormat2 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}
