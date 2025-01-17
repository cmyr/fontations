// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// Sbix header flags.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, bytemuck :: AnyBitPattern)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct HeaderFlags {
    bits: u16,
}

impl HeaderFlags {
    /// Bit 0: Set to 1.
    pub const ALWAYS_SET: Self = Self { bits: 0x0001 };

    /// Bit 1: Draw outlines.
    pub const DRAW_OUTLINES: Self = Self { bits: 0x0002 };
}

impl HeaderFlags {
    ///  Returns an empty set of flags.
    #[inline]
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    /// Returns the set containing all flags.
    #[inline]
    pub const fn all() -> Self {
        Self {
            bits: Self::ALWAYS_SET.bits | Self::DRAW_OUTLINES.bits,
        }
    }

    /// Returns the raw value of the flags currently stored.
    #[inline]
    pub const fn bits(&self) -> u16 {
        self.bits
    }

    /// Convert from underlying bit representation, unless that
    /// representation contains bits that do not correspond to a flag.
    #[inline]
    pub const fn from_bits(bits: u16) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self { bits })
        } else {
            None
        }
    }

    /// Convert from underlying bit representation, dropping any bits
    /// that do not correspond to flags.
    #[inline]
    pub const fn from_bits_truncate(bits: u16) -> Self {
        Self {
            bits: bits & Self::all().bits,
        }
    }

    /// Returns `true` if no flags are currently stored.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }

    /// Returns `true` if there are flags common to both `self` and `other`.
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !(Self {
            bits: self.bits & other.bits,
        })
        .is_empty()
    }

    /// Returns `true` if all of the flags in `other` are contained within `self`.
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }

    /// Inserts the specified flags in-place.
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.bits |= other.bits;
    }

    /// Removes the specified flags in-place.
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.bits &= !other.bits;
    }

    /// Toggles the specified flags in-place.
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.bits ^= other.bits;
    }

    /// Returns the intersection between the flags in `self` and
    /// `other`.
    ///
    /// Specifically, the returned set contains only the flags which are
    /// present in *both* `self` *and* `other`.
    ///
    /// This is equivalent to using the `&` operator (e.g.
    /// [`ops::BitAnd`]), as in `flags & other`.
    ///
    /// [`ops::BitAnd`]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self {
            bits: self.bits & other.bits,
        }
    }

    /// Returns the union of between the flags in `self` and `other`.
    ///
    /// Specifically, the returned set contains all flags which are
    /// present in *either* `self` *or* `other`, including any which are
    /// present in both.
    ///
    /// This is equivalent to using the `|` operator (e.g.
    /// [`ops::BitOr`]), as in `flags | other`.
    ///
    /// [`ops::BitOr`]: https://doc.rust-lang.org/std/ops/trait.BitOr.html
    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    /// Returns the difference between the flags in `self` and `other`.
    ///
    /// Specifically, the returned set contains all flags present in
    /// `self`, except for the ones present in `other`.
    ///
    /// It is also conceptually equivalent to the "bit-clear" operation:
    /// `flags & !other` (and this syntax is also supported).
    ///
    /// This is equivalent to using the `-` operator (e.g.
    /// [`ops::Sub`]), as in `flags - other`.
    ///
    /// [`ops::Sub`]: https://doc.rust-lang.org/std/ops/trait.Sub.html
    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self {
            bits: self.bits & !other.bits,
        }
    }
}

impl std::ops::BitOr for HeaderFlags {
    type Output = Self;

    /// Returns the union of the two sets of flags.
    #[inline]
    fn bitor(self, other: HeaderFlags) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }
}

impl std::ops::BitOrAssign for HeaderFlags {
    /// Adds the set of flags.
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.bits |= other.bits;
    }
}

impl std::ops::BitXor for HeaderFlags {
    type Output = Self;

    /// Returns the left flags, but with all the right flags toggled.
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        Self {
            bits: self.bits ^ other.bits,
        }
    }
}

impl std::ops::BitXorAssign for HeaderFlags {
    /// Toggles the set of flags.
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        self.bits ^= other.bits;
    }
}

impl std::ops::BitAnd for HeaderFlags {
    type Output = Self;

    /// Returns the intersection between the two sets of flags.
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self {
            bits: self.bits & other.bits,
        }
    }
}

impl std::ops::BitAndAssign for HeaderFlags {
    /// Disables all flags disabled in the set.
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.bits &= other.bits;
    }
}

impl std::ops::Sub for HeaderFlags {
    type Output = Self;

    /// Returns the set difference of the two sets of flags.
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            bits: self.bits & !other.bits,
        }
    }
}

impl std::ops::SubAssign for HeaderFlags {
    /// Disables all flags enabled in the set.
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.bits &= !other.bits;
    }
}

impl std::ops::Not for HeaderFlags {
    type Output = Self;

    /// Returns the complement of this set of flags.
    #[inline]
    fn not(self) -> Self {
        Self { bits: !self.bits } & Self::all()
    }
}

impl std::fmt::Debug for HeaderFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let members: &[(&str, Self)] = &[
            ("ALWAYS_SET", Self::ALWAYS_SET),
            ("DRAW_OUTLINES", Self::DRAW_OUTLINES),
        ];
        let mut first = true;
        for (name, value) in members {
            if self.contains(*value) {
                if !first {
                    f.write_str(" | ")?;
                }
                first = false;
                f.write_str(name)?;
            }
        }
        if first {
            f.write_str("(empty)")?;
        }
        Ok(())
    }
}

impl std::fmt::Binary for HeaderFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.bits, f)
    }
}

impl std::fmt::Octal for HeaderFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.bits, f)
    }
}

impl std::fmt::LowerHex for HeaderFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.bits, f)
    }
}

impl std::fmt::UpperHex for HeaderFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.bits, f)
    }
}

impl font_types::Scalar for HeaderFlags {
    type Raw = <u16 as font_types::Scalar>::Raw;
    fn to_raw(self) -> Self::Raw {
        self.bits().to_raw()
    }
    fn from_raw(raw: Self::Raw) -> Self {
        let t = <u16>::from_raw(raw);
        Self::from_bits_truncate(t)
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> From<HeaderFlags> for FieldType<'a> {
    fn from(src: HeaderFlags) -> FieldType<'a> {
        src.bits().into()
    }
}

/// The [sbix (Standard Bitmap Graphics)](https://docs.microsoft.com/en-us/typography/opentype/spec/sbix) table
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct SbixMarker {
    num_glyphs: u16,
    strike_offsets_byte_len: usize,
}

impl SbixMarker {
    pub fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn flags_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + HeaderFlags::RAW_BYTE_LEN
    }

    pub fn num_strikes_byte_range(&self) -> Range<usize> {
        let start = self.flags_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn strike_offsets_byte_range(&self) -> Range<usize> {
        let start = self.num_strikes_byte_range().end;
        start..start + self.strike_offsets_byte_len
    }
}

impl MinByteRange for SbixMarker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.strike_offsets_byte_range().end
    }
}

impl TopLevelTable for Sbix<'_> {
    /// `sbix`
    const TAG: Tag = Tag::new(b"sbix");
}

impl ReadArgs for Sbix<'_> {
    type Args = u16;
}

impl<'a> FontReadWithArgs<'a> for Sbix<'a> {
    fn read_with_args(data: FontData<'a>, args: &u16) -> Result<Self, ReadError> {
        let num_glyphs = *args;
        let mut cursor = data.cursor();
        cursor.advance::<u16>();
        cursor.advance::<HeaderFlags>();
        let num_strikes: u32 = cursor.read()?;
        let strike_offsets_byte_len = (num_strikes as usize)
            .checked_mul(Offset32::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(strike_offsets_byte_len);
        cursor.finish(SbixMarker {
            num_glyphs,
            strike_offsets_byte_len,
        })
    }
}

impl<'a> Sbix<'a> {
    /// A constructor that requires additional arguments.
    ///
    /// This type requires some external state in order to be
    /// parsed.
    pub fn read(data: FontData<'a>, num_glyphs: u16) -> Result<Self, ReadError> {
        let args = num_glyphs;
        Self::read_with_args(data, &args)
    }
}

/// The [sbix (Standard Bitmap Graphics)](https://docs.microsoft.com/en-us/typography/opentype/spec/sbix) table
pub type Sbix<'a> = TableRef<'a, SbixMarker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> Sbix<'a> {
    /// Table version number — set to 1.
    pub fn version(&self) -> u16 {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Bit 0: Set to 1.
    /// Bit 1: Draw outlines.
    /// Bits 2 to 15: reserved (set to 0).
    pub fn flags(&self) -> HeaderFlags {
        let range = self.shape.flags_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Number of bitmap strikes.
    pub fn num_strikes(&self) -> u32 {
        let range = self.shape.num_strikes_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Offsets from the beginning of the 'sbix' table to data for each individual bitmap strike.
    pub fn strike_offsets(&self) -> &'a [BigEndian<Offset32>] {
        let range = self.shape.strike_offsets_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// A dynamically resolving wrapper for [`strike_offsets`][Self::strike_offsets].
    pub fn strikes(&self) -> ArrayOfOffsets<'a, Strike<'a>, Offset32> {
        let data = self.data;
        let offsets = self.strike_offsets();
        let args = self.num_glyphs();
        ArrayOfOffsets::new(offsets, data, args)
    }

    pub(crate) fn num_glyphs(&self) -> u16 {
        self.shape.num_glyphs
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for Sbix<'a> {
    fn type_name(&self) -> &str {
        "Sbix"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new("flags", self.flags())),
            2usize => Some(Field::new("num_strikes", self.num_strikes())),
            3usize => Some({
                let data = self.data;
                let args = self.num_glyphs();
                Field::new(
                    "strike_offsets",
                    FieldType::array_of_offsets(
                        better_type_name::<Strike>(),
                        self.strike_offsets(),
                        move |off| {
                            let target = off.get().resolve_with_args::<Strike>(data, &args);
                            FieldType::offset(off.get(), target)
                        },
                    ),
                )
            }),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for Sbix<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// [Strike](https://learn.microsoft.com/en-us/typography/opentype/spec/sbix#strikes) header table
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct StrikeMarker {
    glyph_data_offsets_byte_len: usize,
}

impl StrikeMarker {
    pub fn ppem_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn ppi_byte_range(&self) -> Range<usize> {
        let start = self.ppem_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn glyph_data_offsets_byte_range(&self) -> Range<usize> {
        let start = self.ppi_byte_range().end;
        start..start + self.glyph_data_offsets_byte_len
    }
}

impl MinByteRange for StrikeMarker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.glyph_data_offsets_byte_range().end
    }
}

impl ReadArgs for Strike<'_> {
    type Args = u16;
}

impl<'a> FontReadWithArgs<'a> for Strike<'a> {
    fn read_with_args(data: FontData<'a>, args: &u16) -> Result<Self, ReadError> {
        let num_glyphs = *args;
        let mut cursor = data.cursor();
        cursor.advance::<u16>();
        cursor.advance::<u16>();
        let glyph_data_offsets_byte_len = (transforms::add(num_glyphs, 1_usize))
            .checked_mul(u32::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(glyph_data_offsets_byte_len);
        cursor.finish(StrikeMarker {
            glyph_data_offsets_byte_len,
        })
    }
}

impl<'a> Strike<'a> {
    /// A constructor that requires additional arguments.
    ///
    /// This type requires some external state in order to be
    /// parsed.
    pub fn read(data: FontData<'a>, num_glyphs: u16) -> Result<Self, ReadError> {
        let args = num_glyphs;
        Self::read_with_args(data, &args)
    }
}

/// [Strike](https://learn.microsoft.com/en-us/typography/opentype/spec/sbix#strikes) header table
pub type Strike<'a> = TableRef<'a, StrikeMarker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> Strike<'a> {
    /// The PPEM size for which this strike was designed.
    pub fn ppem(&self) -> u16 {
        let range = self.shape.ppem_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The device pixel density (in PPI) for which this strike was designed. (E.g., 96 PPI, 192 PPI.)
    pub fn ppi(&self) -> u16 {
        let range = self.shape.ppi_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Offset from the beginning of the strike data header to bitmap data for an individual glyph ID.
    pub fn glyph_data_offsets(&self) -> &'a [BigEndian<u32>] {
        let range = self.shape.glyph_data_offsets_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for Strike<'a> {
    fn type_name(&self) -> &str {
        "Strike"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("ppem", self.ppem())),
            1usize => Some(Field::new("ppi", self.ppi())),
            2usize => Some(Field::new("glyph_data_offsets", self.glyph_data_offsets())),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for Strike<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// [Glyph data](https://learn.microsoft.com/en-us/typography/opentype/spec/sbix#glyph-data) table
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct GlyphDataMarker {
    data_byte_len: usize,
}

impl GlyphDataMarker {
    pub fn origin_offset_x_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + i16::RAW_BYTE_LEN
    }

    pub fn origin_offset_y_byte_range(&self) -> Range<usize> {
        let start = self.origin_offset_x_byte_range().end;
        start..start + i16::RAW_BYTE_LEN
    }

    pub fn graphic_type_byte_range(&self) -> Range<usize> {
        let start = self.origin_offset_y_byte_range().end;
        start..start + Tag::RAW_BYTE_LEN
    }

    pub fn data_byte_range(&self) -> Range<usize> {
        let start = self.graphic_type_byte_range().end;
        start..start + self.data_byte_len
    }
}

impl MinByteRange for GlyphDataMarker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.data_byte_range().end
    }
}

impl<'a> FontRead<'a> for GlyphData<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<i16>();
        cursor.advance::<i16>();
        cursor.advance::<Tag>();
        let data_byte_len = cursor.remaining_bytes() / u8::RAW_BYTE_LEN * u8::RAW_BYTE_LEN;
        cursor.advance_by(data_byte_len);
        cursor.finish(GlyphDataMarker { data_byte_len })
    }
}

/// [Glyph data](https://learn.microsoft.com/en-us/typography/opentype/spec/sbix#glyph-data) table
pub type GlyphData<'a> = TableRef<'a, GlyphDataMarker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> GlyphData<'a> {
    /// The horizontal (x-axis) position of the left edge of the bitmap graphic in relation to the glyph design space origin.
    pub fn origin_offset_x(&self) -> i16 {
        let range = self.shape.origin_offset_x_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The vertical (y-axis) position of the bottom edge of the bitmap graphic in relation to the glyph design space origin.
    pub fn origin_offset_y(&self) -> i16 {
        let range = self.shape.origin_offset_y_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Indicates the format of the embedded graphic data: one of 'jpg ', 'png ' or 'tiff', or the special format 'dupe'.
    pub fn graphic_type(&self) -> Tag {
        let range = self.shape.graphic_type_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// The actual embedded graphic data. The total length is inferred from sequential entries in the glyphDataOffsets array and the fixed size (8 bytes) of the preceding fields.
    pub fn data(&self) -> &'a [u8] {
        let range = self.shape.data_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for GlyphData<'a> {
    fn type_name(&self) -> &str {
        "GlyphData"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("origin_offset_x", self.origin_offset_x())),
            1usize => Some(Field::new("origin_offset_y", self.origin_offset_y())),
            2usize => Some(Field::new("graphic_type", self.graphic_type())),
            3usize => Some(Field::new("data", self.data())),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for GlyphData<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}
