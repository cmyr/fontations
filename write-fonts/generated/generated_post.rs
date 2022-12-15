// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// [post (PostScript)](https://docs.microsoft.com/en-us/typography/opentype/spec/post#header) table
#[derive(Clone, Debug)]
pub struct Post {
    /// 0x00010000 for version 1.0 0x00020000 for version 2.0
    /// 0x00025000 for version 2.5 (deprecated) 0x00030000 for version
    /// 3.0
    pub version: Version16Dot16,
    /// Italic angle in counter-clockwise degrees from the vertical.
    /// Zero for upright text, negative for text that leans to the
    /// right (forward).
    pub italic_angle: Fixed,
    /// This is the suggested distance of the top of the underline from
    /// the baseline (negative values indicate below baseline). The
    /// PostScript definition of this FontInfo dictionary key (the y
    /// coordinate of the center of the stroke) is not used for
    /// historical reasons. The value of the PostScript key may be
    /// calculated by subtracting half the underlineThickness from the
    /// value of this field.
    pub underline_position: FWord,
    /// Suggested values for the underline thickness. In general, the
    /// underline thickness should match the thickness of the
    /// underscore character (U+005F LOW LINE), and should also match
    /// the strikeout thickness, which is specified in the OS/2 table.
    pub underline_thickness: FWord,
    /// Set to 0 if the font is proportionally spaced, non-zero if the
    /// font is not proportionally spaced (i.e. monospaced).
    pub is_fixed_pitch: u32,
    /// Minimum memory usage when an OpenType font is downloaded.
    pub min_mem_type42: u32,
    /// Maximum memory usage when an OpenType font is downloaded.
    pub max_mem_type42: u32,
    /// Minimum memory usage when an OpenType font is downloaded as a
    /// Type 1 font.
    pub min_mem_type1: u32,
    /// Maximum memory usage when an OpenType font is downloaded as a
    /// Type 1 font.
    pub max_mem_type1: u32,
    /// Number of glyphs (this should be the same as numGlyphs in
    /// 'maxp' table).
    pub num_glyphs: Option<u16>,
    /// Array of indices into the string data. See below for details.
    pub glyph_name_index: Option<Vec<u16>>,
    /// Storage for the string data.
    pub string_data: Option<Vec<PString>>,
}

impl Default for Post {
    fn default() -> Self {
        Self {
            version: Version16Dot16::VERSION_1_0,
            italic_angle: Default::default(),
            underline_position: Default::default(),
            underline_thickness: Default::default(),
            is_fixed_pitch: Default::default(),
            min_mem_type42: Default::default(),
            max_mem_type42: Default::default(),
            min_mem_type1: Default::default(),
            max_mem_type1: Default::default(),
            num_glyphs: Default::default(),
            glyph_name_index: Default::default(),
            string_data: Default::default(),
        }
    }
}

impl Post {
    /// Construct a new `Post`
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        italic_angle: Fixed,
        underline_position: FWord,
        underline_thickness: FWord,
        is_fixed_pitch: u32,
        min_mem_type42: u32,
        max_mem_type42: u32,
        min_mem_type1: u32,
        max_mem_type1: u32,
    ) -> Self {
        Self {
            italic_angle,
            underline_position,
            underline_thickness,
            is_fixed_pitch,
            min_mem_type42,
            max_mem_type42,
            min_mem_type1,
            max_mem_type1,
            ..Default::default()
        }
    }
}

impl FontWrite for Post {
    fn write_into(&self, writer: &mut TableWriter) {
        let version = self.version;
        version.write_into(writer);
        self.italic_angle.write_into(writer);
        self.underline_position.write_into(writer);
        self.underline_thickness.write_into(writer);
        self.is_fixed_pitch.write_into(writer);
        self.min_mem_type42.write_into(writer);
        self.max_mem_type42.write_into(writer);
        self.min_mem_type1.write_into(writer);
        self.max_mem_type1.write_into(writer);
        version.compatible((2, 0)).then(|| {
            self.num_glyphs
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version.compatible((2, 0)).then(|| {
            self.glyph_name_index
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version.compatible((2, 0)).then(|| {
            self.string_data
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
    }
}

impl Validate for Post {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Post", |ctx| {
            let version = self.version;
            ctx.in_field("num_glyphs", |ctx| {
                if version.compatible((2, 0)) && self.num_glyphs.is_none() {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
            ctx.in_field("glyph_name_index", |ctx| {
                if version.compatible((2, 0)) && self.glyph_name_index.is_none() {
                    ctx.report(format!("field must be present for version {version}"));
                }
                if self.glyph_name_index.is_some()
                    && self.glyph_name_index.as_ref().unwrap().len() > (u16::MAX as usize)
                {
                    ctx.report("array excedes max length");
                }
            });
        })
    }
}

impl TopLevelTable for Post {
    const TAG: Tag = Tag::new(b"post");
}

impl<'a> FromObjRef<read_fonts::tables::post::Post<'a>> for Post {
    fn from_obj_ref(obj: &read_fonts::tables::post::Post<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        Post {
            version: obj.version(),
            italic_angle: obj.italic_angle(),
            underline_position: obj.underline_position(),
            underline_thickness: obj.underline_thickness(),
            is_fixed_pitch: obj.is_fixed_pitch(),
            min_mem_type42: obj.min_mem_type42(),
            max_mem_type42: obj.max_mem_type42(),
            min_mem_type1: obj.min_mem_type1(),
            max_mem_type1: obj.max_mem_type1(),
            num_glyphs: obj.num_glyphs(),
            glyph_name_index: obj.glyph_name_index().to_owned_obj(offset_data),
            string_data: obj.string_data().map(|obj| {
                obj.iter()
                    .filter_map(|x| x.map(|x| FromObjRef::from_obj_ref(&x, offset_data)).ok())
                    .collect()
            }),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::post::Post<'a>> for Post {}

impl<'a> FontRead<'a> for Post {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::post::Post as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}
