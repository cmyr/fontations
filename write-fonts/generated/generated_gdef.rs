// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// [GDEF](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#gdef-header) 1.0
#[derive(Clone, Debug)]
pub struct Gdef {
    /// Offset to class definition table for glyph type, from beginning
    /// of GDEF header (may be NULL)
    pub glyph_class_def_offset: NullableOffsetMarker<ClassDef>,
    /// Offset to attachment point list table, from beginning of GDEF
    /// header (may be NULL)
    pub attach_list_offset: NullableOffsetMarker<AttachList>,
    /// Offset to ligature caret list table, from beginning of GDEF
    /// header (may be NULL)
    pub lig_caret_list_offset: NullableOffsetMarker<LigCaretList>,
    /// Offset to class definition table for mark attachment type, from
    /// beginning of GDEF header (may be NULL)
    pub mark_attach_class_def_offset: NullableOffsetMarker<ClassDef>,
    /// Offset to the table of mark glyph set definitions, from
    /// beginning of GDEF header (may be NULL)
    pub mark_glyph_sets_def_offset: NullableOffsetMarker<MarkGlyphSets>,
    /// Offset to the Item Variation Store table, from beginning of
    /// GDEF header (may be NULL)
    pub item_var_store_offset: NullableOffsetMarker<ClassDef, WIDTH_32>,
}

impl FontWrite for Gdef {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        let version = self.compute_version() as MajorMinor;
        version.write_into(writer);
        self.glyph_class_def_offset.write_into(writer);
        self.attach_list_offset.write_into(writer);
        self.lig_caret_list_offset.write_into(writer);
        self.mark_attach_class_def_offset.write_into(writer);
        version
            .compatible(MajorMinor::VERSION_1_2)
            .then(|| self.mark_glyph_sets_def_offset.write_into(writer));
        version
            .compatible(MajorMinor::VERSION_1_3)
            .then(|| self.item_var_store_offset.write_into(writer));
    }
}

impl Validate for Gdef {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Gdef", |ctx| {
            ctx.in_field("glyph_class_def_offset", |ctx| {
                self.glyph_class_def_offset.validate_impl(ctx);
            });
            ctx.in_field("attach_list_offset", |ctx| {
                self.attach_list_offset.validate_impl(ctx);
            });
            ctx.in_field("lig_caret_list_offset", |ctx| {
                self.lig_caret_list_offset.validate_impl(ctx);
            });
            ctx.in_field("mark_attach_class_def_offset", |ctx| {
                self.mark_attach_class_def_offset.validate_impl(ctx);
            });
            ctx.in_field("mark_glyph_sets_def_offset", |ctx| {
                self.mark_glyph_sets_def_offset.validate_impl(ctx);
            });
            ctx.in_field("item_var_store_offset", |ctx| {
                self.item_var_store_offset.validate_impl(ctx);
            });
        })
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromObjRef<read_fonts::layout::gdef::Gdef<'a>> for Gdef {
    fn from_obj_ref(obj: &read_fonts::layout::gdef::Gdef<'a>, _: FontData) -> Self {
        Gdef {
            glyph_class_def_offset: obj.glyph_class_def().into(),
            attach_list_offset: obj.attach_list().into(),
            lig_caret_list_offset: obj.lig_caret_list().into(),
            mark_attach_class_def_offset: obj.mark_attach_class_def().into(),
            mark_glyph_sets_def_offset: obj.mark_glyph_sets_def().into(),
            item_var_store_offset: obj.item_var_store().into(),
        }
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromTableRef<read_fonts::layout::gdef::Gdef<'a>> for Gdef {}

#[cfg(feature = "parsing")]
impl<'a> FontRead<'a> for Gdef {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::layout::gdef::Gdef as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// Used in the [Glyph Class Definition Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#glyph-class-definition-table)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GlyphClassDef {
    Base = 1,
    Ligature = 2,
    Mark = 3,
    Component = 4,
}

impl FontWrite for GlyphClassDef {
    fn write_into(&self, writer: &mut TableWriter) {
        let val: u16 = match self {
            Self::Base => 1,
            Self::Ligature => 2,
            Self::Mark => 3,
            Self::Component => 4,
        };
        writer.write_slice(&val.to_be_bytes())
    }
}

/// [Attachment Point List Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#attachment-point-list-table)
#[derive(Clone, Debug)]
pub struct AttachList {
    /// Offset to Coverage table - from beginning of AttachList table
    pub coverage_offset: OffsetMarker<CoverageTable>,
    /// Array of offsets to AttachPoint tables-from beginning of
    /// AttachList table-in Coverage Index order
    pub attach_point_offsets: Vec<OffsetMarker<AttachPoint>>,
}

impl FontWrite for AttachList {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.coverage_offset.write_into(writer);
        (array_len(&self.attach_point_offsets).unwrap() as u16).write_into(writer);
        self.attach_point_offsets.write_into(writer);
    }
}

impl Validate for AttachList {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("AttachList", |ctx| {
            ctx.in_field("coverage_offset", |ctx| {
                self.coverage_offset.validate_impl(ctx);
            });
            ctx.in_field("attach_point_offsets", |ctx| {
                if self.attach_point_offsets.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
                self.attach_point_offsets.validate_impl(ctx);
            });
        })
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromObjRef<read_fonts::layout::gdef::AttachList<'a>> for AttachList {
    fn from_obj_ref(obj: &read_fonts::layout::gdef::AttachList<'a>, _: FontData) -> Self {
        AttachList {
            coverage_offset: obj.coverage().into(),
            attach_point_offsets: obj.attach_points().map(|x| x.into()).collect(),
        }
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromTableRef<read_fonts::layout::gdef::AttachList<'a>> for AttachList {}

#[cfg(feature = "parsing")]
impl<'a> FontRead<'a> for AttachList {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::layout::gdef::AttachList as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// Part of [AttachList]
#[derive(Clone, Debug)]
pub struct AttachPoint {
    /// Array of contour point indices -in increasing numerical order
    pub point_indices: Vec<u16>,
}

impl FontWrite for AttachPoint {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.point_indices).unwrap() as u16).write_into(writer);
        self.point_indices.write_into(writer);
    }
}

impl Validate for AttachPoint {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("AttachPoint", |ctx| {
            ctx.in_field("point_indices", |ctx| {
                if self.point_indices.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
            });
        })
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromObjRef<read_fonts::layout::gdef::AttachPoint<'a>> for AttachPoint {
    fn from_obj_ref(obj: &read_fonts::layout::gdef::AttachPoint<'a>, _: FontData) -> Self {
        AttachPoint {
            point_indices: obj.point_indices().iter().map(|x| x.get()).collect(),
        }
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromTableRef<read_fonts::layout::gdef::AttachPoint<'a>> for AttachPoint {}

#[cfg(feature = "parsing")]
impl<'a> FontRead<'a> for AttachPoint {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::layout::gdef::AttachPoint as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [Ligature Caret List Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#ligature-caret-list-table)
#[derive(Clone, Debug)]
pub struct LigCaretList {
    /// Offset to Coverage table - from beginning of LigCaretList table
    pub coverage_offset: OffsetMarker<CoverageTable>,
    /// Array of offsets to LigGlyph tables, from beginning of
    /// LigCaretList table —in Coverage Index order
    pub lig_glyph_offsets: Vec<OffsetMarker<LigGlyph>>,
}

impl FontWrite for LigCaretList {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.coverage_offset.write_into(writer);
        (array_len(&self.lig_glyph_offsets).unwrap() as u16).write_into(writer);
        self.lig_glyph_offsets.write_into(writer);
    }
}

impl Validate for LigCaretList {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("LigCaretList", |ctx| {
            ctx.in_field("coverage_offset", |ctx| {
                self.coverage_offset.validate_impl(ctx);
            });
            ctx.in_field("lig_glyph_offsets", |ctx| {
                if self.lig_glyph_offsets.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
                self.lig_glyph_offsets.validate_impl(ctx);
            });
        })
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromObjRef<read_fonts::layout::gdef::LigCaretList<'a>> for LigCaretList {
    fn from_obj_ref(obj: &read_fonts::layout::gdef::LigCaretList<'a>, _: FontData) -> Self {
        LigCaretList {
            coverage_offset: obj.coverage().into(),
            lig_glyph_offsets: obj.lig_glyphs().map(|x| x.into()).collect(),
        }
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromTableRef<read_fonts::layout::gdef::LigCaretList<'a>> for LigCaretList {}

#[cfg(feature = "parsing")]
impl<'a> FontRead<'a> for LigCaretList {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::layout::gdef::LigCaretList as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [Ligature Glyph Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#ligature-glyph-table)
#[derive(Clone, Debug)]
pub struct LigGlyph {
    /// Array of offsets to CaretValue tables, from beginning of
    /// LigGlyph table — in increasing coordinate order
    pub caret_value_offsets: Vec<OffsetMarker<CaretValue>>,
}

impl FontWrite for LigGlyph {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.caret_value_offsets).unwrap() as u16).write_into(writer);
        self.caret_value_offsets.write_into(writer);
    }
}

impl Validate for LigGlyph {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("LigGlyph", |ctx| {
            ctx.in_field("caret_value_offsets", |ctx| {
                if self.caret_value_offsets.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
                self.caret_value_offsets.validate_impl(ctx);
            });
        })
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromObjRef<read_fonts::layout::gdef::LigGlyph<'a>> for LigGlyph {
    fn from_obj_ref(obj: &read_fonts::layout::gdef::LigGlyph<'a>, _: FontData) -> Self {
        LigGlyph {
            caret_value_offsets: obj.caret_values().map(|x| x.into()).collect(),
        }
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromTableRef<read_fonts::layout::gdef::LigGlyph<'a>> for LigGlyph {}

#[cfg(feature = "parsing")]
impl<'a> FontRead<'a> for LigGlyph {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::layout::gdef::LigGlyph as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [Caret Value Tables](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caret-value-tables)
#[derive(Clone, Debug)]
pub enum CaretValue {
    Format1(CaretValueFormat1),
    Format2(CaretValueFormat2),
    Format3(CaretValueFormat3),
}

impl FontWrite for CaretValue {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Format1(item) => item.write_into(writer),
            Self::Format2(item) => item.write_into(writer),
            Self::Format3(item) => item.write_into(writer),
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

#[cfg(feature = "parsing")]
impl FromObjRef<read_fonts::layout::gdef::CaretValue<'_>> for CaretValue {
    fn from_obj_ref(obj: &read_fonts::layout::gdef::CaretValue, _: FontData) -> Self {
        use read_fonts::layout::gdef::CaretValue as ObjRefType;
        match obj {
            ObjRefType::Format1(item) => CaretValue::Format1(item.to_owned_table()),
            ObjRefType::Format2(item) => CaretValue::Format2(item.to_owned_table()),
            ObjRefType::Format3(item) => CaretValue::Format3(item.to_owned_table()),
        }
    }
}

#[cfg(feature = "parsing")]
impl FromTableRef<read_fonts::layout::gdef::CaretValue<'_>> for CaretValue {}

#[cfg(feature = "parsing")]
impl<'a> FontRead<'a> for CaretValue {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::layout::gdef::CaretValue as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [CaretValue Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caretvalue-format-1)
#[derive(Clone, Debug)]
pub struct CaretValueFormat1 {
    /// X or Y value, in design units
    pub coordinate: i16,
}

impl FontWrite for CaretValueFormat1 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        self.coordinate.write_into(writer);
    }
}

impl Validate for CaretValueFormat1 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

#[cfg(feature = "parsing")]
impl<'a> FromObjRef<read_fonts::layout::gdef::CaretValueFormat1<'a>> for CaretValueFormat1 {
    fn from_obj_ref(obj: &read_fonts::layout::gdef::CaretValueFormat1<'a>, _: FontData) -> Self {
        CaretValueFormat1 {
            coordinate: obj.coordinate(),
        }
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromTableRef<read_fonts::layout::gdef::CaretValueFormat1<'a>> for CaretValueFormat1 {}

#[cfg(feature = "parsing")]
impl<'a> FontRead<'a> for CaretValueFormat1 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::layout::gdef::CaretValueFormat1 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// [CaretValue Format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caretvalue-format-2)
#[derive(Clone, Debug)]
pub struct CaretValueFormat2 {
    /// Contour point index on glyph
    pub caret_value_point_index: u16,
}

impl FontWrite for CaretValueFormat2 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (2 as u16).write_into(writer);
        self.caret_value_point_index.write_into(writer);
    }
}

impl Validate for CaretValueFormat2 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

#[cfg(feature = "parsing")]
impl<'a> FromObjRef<read_fonts::layout::gdef::CaretValueFormat2<'a>> for CaretValueFormat2 {
    fn from_obj_ref(obj: &read_fonts::layout::gdef::CaretValueFormat2<'a>, _: FontData) -> Self {
        CaretValueFormat2 {
            caret_value_point_index: obj.caret_value_point_index(),
        }
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromTableRef<read_fonts::layout::gdef::CaretValueFormat2<'a>> for CaretValueFormat2 {}

#[cfg(feature = "parsing")]
impl<'a> FontRead<'a> for CaretValueFormat2 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::layout::gdef::CaretValueFormat2 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// [CaretValue Format 3](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#caretvalue-format-3)
#[derive(Clone, Debug)]
pub struct CaretValueFormat3 {
    /// X or Y value, in design units
    pub coordinate: i16,
    /// Offset to Device table (non-variable font) / Variation Index
    /// table (variable font) for X or Y value-from beginning of
    /// CaretValue table
    pub device_offset: OffsetMarker<Device>,
}

impl FontWrite for CaretValueFormat3 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (3 as u16).write_into(writer);
        self.coordinate.write_into(writer);
        self.device_offset.write_into(writer);
    }
}

impl Validate for CaretValueFormat3 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("CaretValueFormat3", |ctx| {
            ctx.in_field("device_offset", |ctx| {
                self.device_offset.validate_impl(ctx);
            });
        })
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromObjRef<read_fonts::layout::gdef::CaretValueFormat3<'a>> for CaretValueFormat3 {
    fn from_obj_ref(obj: &read_fonts::layout::gdef::CaretValueFormat3<'a>, _: FontData) -> Self {
        CaretValueFormat3 {
            coordinate: obj.coordinate(),
            device_offset: obj.device().into(),
        }
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromTableRef<read_fonts::layout::gdef::CaretValueFormat3<'a>> for CaretValueFormat3 {}

#[cfg(feature = "parsing")]
impl<'a> FontRead<'a> for CaretValueFormat3 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::layout::gdef::CaretValueFormat3 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// [Mark Glyph Sets Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gdef#mark-glyph-sets-table)
#[derive(Clone, Debug)]
pub struct MarkGlyphSets {
    /// Array of offsets to mark glyph set coverage tables, from the
    /// start of the MarkGlyphSets table.
    pub coverage_offsets: Vec<OffsetMarker<CoverageTable, WIDTH_32>>,
}

impl FontWrite for MarkGlyphSets {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        (array_len(&self.coverage_offsets).unwrap() as u16).write_into(writer);
        self.coverage_offsets.write_into(writer);
    }
}

impl Validate for MarkGlyphSets {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("MarkGlyphSets", |ctx| {
            ctx.in_field("coverage_offsets", |ctx| {
                if self.coverage_offsets.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
                self.coverage_offsets.validate_impl(ctx);
            });
        })
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromObjRef<read_fonts::layout::gdef::MarkGlyphSets<'a>> for MarkGlyphSets {
    fn from_obj_ref(obj: &read_fonts::layout::gdef::MarkGlyphSets<'a>, _: FontData) -> Self {
        MarkGlyphSets {
            coverage_offsets: obj.coverages().map(|x| x.into()).collect(),
        }
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromTableRef<read_fonts::layout::gdef::MarkGlyphSets<'a>> for MarkGlyphSets {}

#[cfg(feature = "parsing")]
impl<'a> FontRead<'a> for MarkGlyphSets {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::layout::gdef::MarkGlyphSets as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}
