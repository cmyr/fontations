// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

pub use read_fonts::tables::gdef::GlyphClassDef;

/// [GDEF](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#gdef-header) 1.0
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gdef {
    /// Offset to class definition table for glyph type, from beginning
    /// of GDEF header (may be NULL)
    pub glyph_class_def: NullableOffsetMarker<ClassDef>,
    /// Offset to attachment point list table, from beginning of GDEF
    /// header (may be NULL)
    pub attach_list: NullableOffsetMarker<AttachList>,
    /// Offset to ligature caret list table, from beginning of GDEF
    /// header (may be NULL)
    pub lig_caret_list: NullableOffsetMarker<LigCaretList>,
    /// Offset to class definition table for mark attachment type, from
    /// beginning of GDEF header (may be NULL)
    pub mark_attach_class_def: NullableOffsetMarker<ClassDef>,
    /// Offset to the table of mark glyph set definitions, from
    /// beginning of GDEF header (may be NULL)
    pub mark_glyph_sets_def: NullableOffsetMarker<MarkGlyphSets>,
    /// Offset to the Item Variation Store table, from beginning of
    /// GDEF header (may be NULL)
    pub item_var_store: NullableOffsetMarker<ItemVariationStore, WIDTH_32>,
}

impl Gdef {
    /// Construct a new `Gdef`
    pub fn new(
        glyph_class_def: Option<ClassDef>,
        attach_list: Option<AttachList>,
        lig_caret_list: Option<LigCaretList>,
        mark_attach_class_def: Option<ClassDef>,
    ) -> Self {
        Self {
            glyph_class_def: glyph_class_def.into(),
            attach_list: attach_list.into(),
            lig_caret_list: lig_caret_list.into(),
            mark_attach_class_def: mark_attach_class_def.into(),
            ..Default::default()
        }
    }
}

impl FontWrite for Gdef {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        let version = self.compute_version() as MajorMinor;
        version.write_into(writer);
        self.glyph_class_def.write_into(writer);
        self.attach_list.write_into(writer);
        self.lig_caret_list.write_into(writer);
        self.mark_attach_class_def.write_into(writer);
        version
            .compatible((1u16, 2u16))
            .then(|| self.mark_glyph_sets_def.write_into(writer));
        version
            .compatible((1u16, 3u16))
            .then(|| self.item_var_store.write_into(writer));
    }
    fn table_type(&self) -> TableType {
        TableType::TopLevel(Gdef::TAG)
    }
}

impl Validate for Gdef {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Gdef", |ctx| {
            ctx.in_field("glyph_class_def", |ctx| {
                self.glyph_class_def.validate_impl(ctx);
            });
            ctx.in_field("attach_list", |ctx| {
                self.attach_list.validate_impl(ctx);
            });
            ctx.in_field("lig_caret_list", |ctx| {
                self.lig_caret_list.validate_impl(ctx);
            });
            ctx.in_field("mark_attach_class_def", |ctx| {
                self.mark_attach_class_def.validate_impl(ctx);
            });
            ctx.in_field("mark_glyph_sets_def", |ctx| {
                self.mark_glyph_sets_def.validate_impl(ctx);
            });
            ctx.in_field("item_var_store", |ctx| {
                self.item_var_store.validate_impl(ctx);
            });
        })
    }
}

impl TopLevelTable for Gdef {
    const TAG: Tag = Tag::new(b"GDEF");
}

impl<'a> FromObjRef<read_fonts::tables::gdef::Gdef<'a>> for Gdef {
    fn from_obj_ref(obj: &read_fonts::tables::gdef::Gdef<'a>, _: FontData) -> Self {
        Gdef {
            glyph_class_def: obj.glyph_class_def().to_owned_table(),
            attach_list: obj.attach_list().to_owned_table(),
            lig_caret_list: obj.lig_caret_list().to_owned_table(),
            mark_attach_class_def: obj.mark_attach_class_def().to_owned_table(),
            mark_glyph_sets_def: obj.mark_glyph_sets_def().to_owned_table(),
            item_var_store: obj.item_var_store().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gdef::Gdef<'a>> for Gdef {}

impl<'a> FontRead<'a> for Gdef {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gdef::Gdef as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

impl FontWrite for GlyphClassDef {
    fn write_into(&self, writer: &mut TableWriter) {
        let val = *self as u16;
        writer.write_slice(&val.to_be_bytes())
    }
}

/// [Attachment Point List Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#attachment-point-list-table)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttachList {
    /// Offset to Coverage table - from beginning of AttachList table
    pub coverage: OffsetMarker<CoverageTable>,
    /// Array of offsets to AttachPoint tables-from beginning of
    /// AttachList table-in Coverage Index order
    pub attach_points: Vec<OffsetMarker<AttachPoint>>,
}

impl AttachList {
    /// Construct a new `AttachList`
    pub fn new(coverage: CoverageTable, attach_points: Vec<AttachPoint>) -> Self {
        Self {
            coverage: coverage.into(),
            attach_points: attach_points.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for AttachList {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.coverage.write_into(writer);
        (array_len(&self.attach_points).unwrap() as u16).write_into(writer);
        self.attach_points.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("AttachList")
    }
}

impl Validate for AttachList {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("AttachList", |ctx| {
            ctx.in_field("coverage", |ctx| {
                self.coverage.validate_impl(ctx);
            });
            ctx.in_field("attach_points", |ctx| {
                if self.attach_points.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.attach_points.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::gdef::AttachList<'a>> for AttachList {
    fn from_obj_ref(obj: &read_fonts::tables::gdef::AttachList<'a>, _: FontData) -> Self {
        AttachList {
            coverage: obj.coverage().to_owned_table(),
            attach_points: obj.attach_points().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gdef::AttachList<'a>> for AttachList {}

impl<'a> FontRead<'a> for AttachList {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gdef::AttachList as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// Part of [AttachList]
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttachPoint {
    /// Array of contour point indices -in increasing numerical order
    pub point_indices: Vec<u16>,
}

impl AttachPoint {
    /// Construct a new `AttachPoint`
    pub fn new(point_indices: Vec<u16>) -> Self {
        Self {
            point_indices: point_indices.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for AttachPoint {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.point_indices).unwrap() as u16).write_into(writer);
        self.point_indices.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("AttachPoint")
    }
}

impl Validate for AttachPoint {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("AttachPoint", |ctx| {
            ctx.in_field("point_indices", |ctx| {
                if self.point_indices.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::gdef::AttachPoint<'a>> for AttachPoint {
    fn from_obj_ref(obj: &read_fonts::tables::gdef::AttachPoint<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        AttachPoint {
            point_indices: obj.point_indices().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gdef::AttachPoint<'a>> for AttachPoint {}

impl<'a> FontRead<'a> for AttachPoint {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gdef::AttachPoint as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [Ligature Caret List Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#ligature-caret-list-table)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LigCaretList {
    /// Offset to Coverage table - from beginning of LigCaretList table
    pub coverage: OffsetMarker<CoverageTable>,
    /// Array of offsets to LigGlyph tables, from beginning of
    /// LigCaretList table —in Coverage Index order
    pub lig_glyphs: Vec<OffsetMarker<LigGlyph>>,
}

impl LigCaretList {
    /// Construct a new `LigCaretList`
    pub fn new(coverage: CoverageTable, lig_glyphs: Vec<LigGlyph>) -> Self {
        Self {
            coverage: coverage.into(),
            lig_glyphs: lig_glyphs.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for LigCaretList {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.coverage.write_into(writer);
        (array_len(&self.lig_glyphs).unwrap() as u16).write_into(writer);
        self.lig_glyphs.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("LigCaretList")
    }
}

impl Validate for LigCaretList {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("LigCaretList", |ctx| {
            ctx.in_field("coverage", |ctx| {
                self.coverage.validate_impl(ctx);
            });
            ctx.in_field("lig_glyphs", |ctx| {
                if self.lig_glyphs.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.lig_glyphs.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::gdef::LigCaretList<'a>> for LigCaretList {
    fn from_obj_ref(obj: &read_fonts::tables::gdef::LigCaretList<'a>, _: FontData) -> Self {
        LigCaretList {
            coverage: obj.coverage().to_owned_table(),
            lig_glyphs: obj.lig_glyphs().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gdef::LigCaretList<'a>> for LigCaretList {}

impl<'a> FontRead<'a> for LigCaretList {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gdef::LigCaretList as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [Ligature Glyph Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#ligature-glyph-table)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LigGlyph {
    /// Array of offsets to CaretValue tables, from beginning of
    /// LigGlyph table — in increasing coordinate order
    pub caret_values: Vec<OffsetMarker<CaretValue>>,
}

impl LigGlyph {
    /// Construct a new `LigGlyph`
    pub fn new(caret_values: Vec<CaretValue>) -> Self {
        Self {
            caret_values: caret_values.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for LigGlyph {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.caret_values).unwrap() as u16).write_into(writer);
        self.caret_values.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("LigGlyph")
    }
}

impl Validate for LigGlyph {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("LigGlyph", |ctx| {
            ctx.in_field("caret_values", |ctx| {
                if self.caret_values.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.caret_values.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::gdef::LigGlyph<'a>> for LigGlyph {
    fn from_obj_ref(obj: &read_fonts::tables::gdef::LigGlyph<'a>, _: FontData) -> Self {
        LigGlyph {
            caret_values: obj.caret_values().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gdef::LigGlyph<'a>> for LigGlyph {}

impl<'a> FontRead<'a> for LigGlyph {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gdef::LigGlyph as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [Caret Value Tables](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caret-value-tables)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CaretValue {
    Format1(CaretValueFormat1),
    Format2(CaretValueFormat2),
    Format3(CaretValueFormat3),
}

impl CaretValue {
    /// Construct a new `CaretValueFormat1` subtable
    pub fn format_1(coordinate: i16) -> Self {
        Self::Format1(CaretValueFormat1::new(coordinate))
    }

    /// Construct a new `CaretValueFormat2` subtable
    pub fn format_2(caret_value_point_index: u16) -> Self {
        Self::Format2(CaretValueFormat2::new(caret_value_point_index))
    }

    /// Construct a new `CaretValueFormat3` subtable
    pub fn format_3(coordinate: i16, device: DeviceOrVariationIndex) -> Self {
        Self::Format3(CaretValueFormat3::new(coordinate, device))
    }
}

impl Default for CaretValue {
    fn default() -> Self {
        Self::Format1(Default::default())
    }
}

impl FontWrite for CaretValue {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Format1(item) => item.write_into(writer),
            Self::Format2(item) => item.write_into(writer),
            Self::Format3(item) => item.write_into(writer),
        }
    }
    fn table_type(&self) -> TableType {
        match self {
            Self::Format1(item) => item.table_type(),
            Self::Format2(item) => item.table_type(),
            Self::Format3(item) => item.table_type(),
        }
    }
}

impl Validate for CaretValue {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        match self {
            Self::Format1(item) => item.validate_impl(ctx),
            Self::Format2(item) => item.validate_impl(ctx),
            Self::Format3(item) => item.validate_impl(ctx),
        }
    }
}

impl FromObjRef<read_fonts::tables::gdef::CaretValue<'_>> for CaretValue {
    fn from_obj_ref(obj: &read_fonts::tables::gdef::CaretValue, _: FontData) -> Self {
        use read_fonts::tables::gdef::CaretValue as ObjRefType;
        match obj {
            ObjRefType::Format1(item) => CaretValue::Format1(item.to_owned_table()),
            ObjRefType::Format2(item) => CaretValue::Format2(item.to_owned_table()),
            ObjRefType::Format3(item) => CaretValue::Format3(item.to_owned_table()),
        }
    }
}

impl FromTableRef<read_fonts::tables::gdef::CaretValue<'_>> for CaretValue {}

impl<'a> FontRead<'a> for CaretValue {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gdef::CaretValue as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

impl From<CaretValueFormat1> for CaretValue {
    fn from(src: CaretValueFormat1) -> CaretValue {
        CaretValue::Format1(src)
    }
}

impl From<CaretValueFormat2> for CaretValue {
    fn from(src: CaretValueFormat2) -> CaretValue {
        CaretValue::Format2(src)
    }
}

impl From<CaretValueFormat3> for CaretValue {
    fn from(src: CaretValueFormat3) -> CaretValue {
        CaretValue::Format3(src)
    }
}

/// [CaretValue Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caretvalue-format-1)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaretValueFormat1 {
    /// X or Y value, in design units
    pub coordinate: i16,
}

impl CaretValueFormat1 {
    /// Construct a new `CaretValueFormat1`
    pub fn new(coordinate: i16) -> Self {
        Self { coordinate }
    }
}

impl FontWrite for CaretValueFormat1 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        self.coordinate.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("CaretValueFormat1")
    }
}

impl Validate for CaretValueFormat1 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::gdef::CaretValueFormat1<'a>> for CaretValueFormat1 {
    fn from_obj_ref(obj: &read_fonts::tables::gdef::CaretValueFormat1<'a>, _: FontData) -> Self {
        CaretValueFormat1 {
            coordinate: obj.coordinate(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gdef::CaretValueFormat1<'a>> for CaretValueFormat1 {}

impl<'a> FontRead<'a> for CaretValueFormat1 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gdef::CaretValueFormat1 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// [CaretValue Format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caretvalue-format-2)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaretValueFormat2 {
    /// Contour point index on glyph
    pub caret_value_point_index: u16,
}

impl CaretValueFormat2 {
    /// Construct a new `CaretValueFormat2`
    pub fn new(caret_value_point_index: u16) -> Self {
        Self {
            caret_value_point_index,
        }
    }
}

impl FontWrite for CaretValueFormat2 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (2 as u16).write_into(writer);
        self.caret_value_point_index.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("CaretValueFormat2")
    }
}

impl Validate for CaretValueFormat2 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::gdef::CaretValueFormat2<'a>> for CaretValueFormat2 {
    fn from_obj_ref(obj: &read_fonts::tables::gdef::CaretValueFormat2<'a>, _: FontData) -> Self {
        CaretValueFormat2 {
            caret_value_point_index: obj.caret_value_point_index(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gdef::CaretValueFormat2<'a>> for CaretValueFormat2 {}

impl<'a> FontRead<'a> for CaretValueFormat2 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gdef::CaretValueFormat2 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// [CaretValue Format 3](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caretvalue-format-3)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaretValueFormat3 {
    /// X or Y value, in design units
    pub coordinate: i16,
    /// Offset to Device table (non-variable font) / Variation Index
    /// table (variable font) for X or Y value-from beginning of
    /// CaretValue table
    pub device: OffsetMarker<DeviceOrVariationIndex>,
}

impl CaretValueFormat3 {
    /// Construct a new `CaretValueFormat3`
    pub fn new(coordinate: i16, device: DeviceOrVariationIndex) -> Self {
        Self {
            coordinate,
            device: device.into(),
        }
    }
}

impl FontWrite for CaretValueFormat3 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (3 as u16).write_into(writer);
        self.coordinate.write_into(writer);
        self.device.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("CaretValueFormat3")
    }
}

impl Validate for CaretValueFormat3 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("CaretValueFormat3", |ctx| {
            ctx.in_field("device", |ctx| {
                self.device.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::gdef::CaretValueFormat3<'a>> for CaretValueFormat3 {
    fn from_obj_ref(obj: &read_fonts::tables::gdef::CaretValueFormat3<'a>, _: FontData) -> Self {
        CaretValueFormat3 {
            coordinate: obj.coordinate(),
            device: obj.device().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gdef::CaretValueFormat3<'a>> for CaretValueFormat3 {}

impl<'a> FontRead<'a> for CaretValueFormat3 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gdef::CaretValueFormat3 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// [Mark Glyph Sets Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#mark-glyph-sets-table)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarkGlyphSets {
    /// Array of offsets to mark glyph set coverage tables, from the
    /// start of the MarkGlyphSets table.
    pub coverages: Vec<OffsetMarker<CoverageTable, WIDTH_32>>,
}

impl MarkGlyphSets {
    /// Construct a new `MarkGlyphSets`
    pub fn new(coverages: Vec<CoverageTable>) -> Self {
        Self {
            coverages: coverages.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for MarkGlyphSets {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        (array_len(&self.coverages).unwrap() as u16).write_into(writer);
        self.coverages.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("MarkGlyphSets")
    }
}

impl Validate for MarkGlyphSets {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("MarkGlyphSets", |ctx| {
            ctx.in_field("coverages", |ctx| {
                if self.coverages.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.coverages.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::gdef::MarkGlyphSets<'a>> for MarkGlyphSets {
    fn from_obj_ref(obj: &read_fonts::tables::gdef::MarkGlyphSets<'a>, _: FontData) -> Self {
        MarkGlyphSets {
            coverages: obj.coverages().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gdef::MarkGlyphSets<'a>> for MarkGlyphSets {}

impl<'a> FontRead<'a> for MarkGlyphSets {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::gdef::MarkGlyphSets as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}
