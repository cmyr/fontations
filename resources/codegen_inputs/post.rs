#![parse_module(read_fonts::tables::post)]

/// [post (PostScript)](https://docs.microsoft.com/en-us/typography/opentype/spec/post#header) table
#[tag = "post"]
table Post {
    /// 0x00010000 for version 1.0 0x00020000 for version 2.0
    /// 0x00025000 for version 2.5 (deprecated) 0x00030000 for version
    /// 3.0
    #[version]
    // NOTE: we will need some sort of builder to compile post tables, and that
    // builder should set the version. This attribute is a placeholder.
    #[default(Version16Dot16::VERSION_1_0)]
    version: Version16Dot16,
    /// Italic angle in counter-clockwise degrees from the vertical.
    /// Zero for upright text, negative for text that leans to the
    /// right (forward).
    italic_angle: Fixed,
    /// This is the suggested distance of the top of the underline from
    /// the baseline (negative values indicate below baseline). The
    /// PostScript definition of this FontInfo dictionary key (the y
    /// coordinate of the center of the stroke) is not used for
    /// historical reasons. The value of the PostScript key may be
    /// calculated by subtracting half the underlineThickness from the
    /// value of this field.
    underline_position: FWord,
    /// Suggested values for the underline thickness. In general, the
    /// underline thickness should match the thickness of the
    /// underscore character (U+005F LOW LINE), and should also match
    /// the strikeout thickness, which is specified in the OS/2 table.
    underline_thickness: FWord,
    /// Set to 0 if the font is proportionally spaced, non-zero if the
    /// font is not proportionally spaced (i.e. monospaced).
    is_fixed_pitch: u32,
    /// Minimum memory usage when an OpenType font is downloaded.
    min_mem_type42: u32,
    /// Maximum memory usage when an OpenType font is downloaded.
    max_mem_type42: u32,
    /// Minimum memory usage when an OpenType font is downloaded as a
    /// Type 1 font.
    min_mem_type1: u32,
    /// Maximum memory usage when an OpenType font is downloaded as a
    /// Type 1 font.
    max_mem_type1: u32,
    /// Number of glyphs (this should be the same as numGlyphs in
    /// 'maxp' table).
    #[available(2,0)]
    num_glyphs: u16,
    /// Array of indices into the string data. See below for details.
    #[count($num_glyphs)]
    #[available(2,0)]
    glyph_name_index: [u16],
    /// Storage for the string data.
    #[count(..)]
    #[validate(skip)]
    #[available(2,0)]
    #[traverse_with(traverse_string_data)]
    string_data: VarLenArray<PString<'a>>,
}
