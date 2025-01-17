// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// An array of variable-sized objects in a `CFF` table.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct Index1Marker {
    offsets_byte_len: usize,
    data_byte_len: usize,
}

impl Index1Marker {
    pub fn count_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn off_size_byte_range(&self) -> Range<usize> {
        let start = self.count_byte_range().end;
        start..start + u8::RAW_BYTE_LEN
    }

    pub fn offsets_byte_range(&self) -> Range<usize> {
        let start = self.off_size_byte_range().end;
        start..start + self.offsets_byte_len
    }

    pub fn data_byte_range(&self) -> Range<usize> {
        let start = self.offsets_byte_range().end;
        start..start + self.data_byte_len
    }
}

impl MinByteRange for Index1Marker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.data_byte_range().end
    }
}

impl<'a> FontRead<'a> for Index1<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let count: u16 = cursor.read()?;
        let off_size: u8 = cursor.read()?;
        let offsets_byte_len = (transforms::add_multiply(count, 1_usize, off_size))
            .checked_mul(u8::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(offsets_byte_len);
        let data_byte_len = cursor.remaining_bytes() / u8::RAW_BYTE_LEN * u8::RAW_BYTE_LEN;
        cursor.advance_by(data_byte_len);
        cursor.finish(Index1Marker {
            offsets_byte_len,
            data_byte_len,
        })
    }
}

/// An array of variable-sized objects in a `CFF` table.
pub type Index1<'a> = TableRef<'a, Index1Marker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> Index1<'a> {
    /// Number of objects stored in INDEX.
    pub fn count(&self) -> u16 {
        let range = self.shape.count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Object array element size.
    pub fn off_size(&self) -> u8 {
        let range = self.shape.off_size_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Bytes containing `count + 1` offsets each of `off_size`.
    pub fn offsets(&self) -> &'a [u8] {
        let range = self.shape.offsets_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// Array containing the object data.
    pub fn data(&self) -> &'a [u8] {
        let range = self.shape.data_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for Index1<'a> {
    fn type_name(&self) -> &str {
        "Index1"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("count", self.count())),
            1usize => Some(Field::new("off_size", self.off_size())),
            2usize => Some(Field::new("offsets", self.offsets())),
            3usize => Some(Field::new("data", self.data())),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for Index1<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// An array of variable-sized objects in a `CFF2` table.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct Index2Marker {
    offsets_byte_len: usize,
    data_byte_len: usize,
}

impl Index2Marker {
    pub fn count_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn off_size_byte_range(&self) -> Range<usize> {
        let start = self.count_byte_range().end;
        start..start + u8::RAW_BYTE_LEN
    }

    pub fn offsets_byte_range(&self) -> Range<usize> {
        let start = self.off_size_byte_range().end;
        start..start + self.offsets_byte_len
    }

    pub fn data_byte_range(&self) -> Range<usize> {
        let start = self.offsets_byte_range().end;
        start..start + self.data_byte_len
    }
}

impl MinByteRange for Index2Marker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.data_byte_range().end
    }
}

impl<'a> FontRead<'a> for Index2<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let count: u32 = cursor.read()?;
        let off_size: u8 = cursor.read()?;
        let offsets_byte_len = (transforms::add_multiply(count, 1_usize, off_size))
            .checked_mul(u8::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(offsets_byte_len);
        let data_byte_len = cursor.remaining_bytes() / u8::RAW_BYTE_LEN * u8::RAW_BYTE_LEN;
        cursor.advance_by(data_byte_len);
        cursor.finish(Index2Marker {
            offsets_byte_len,
            data_byte_len,
        })
    }
}

/// An array of variable-sized objects in a `CFF2` table.
pub type Index2<'a> = TableRef<'a, Index2Marker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> Index2<'a> {
    /// Number of objects stored in INDEX.
    pub fn count(&self) -> u32 {
        let range = self.shape.count_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Object array element size.
    pub fn off_size(&self) -> u8 {
        let range = self.shape.off_size_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Bytes containing `count + 1` offsets each of `off_size`.
    pub fn offsets(&self) -> &'a [u8] {
        let range = self.shape.offsets_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// Array containing the object data.
    pub fn data(&self) -> &'a [u8] {
        let range = self.shape.data_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for Index2<'a> {
    fn type_name(&self) -> &str {
        "Index2"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("count", self.count())),
            1usize => Some(Field::new("off_size", self.off_size())),
            2usize => Some(Field::new("offsets", self.offsets())),
            3usize => Some(Field::new("data", self.data())),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for Index2<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// Associates a glyph identifier with a Font DICT.
#[derive(Clone)]
pub enum FdSelect<'a> {
    Format0(FdSelectFormat0<'a>),
    Format3(FdSelectFormat3<'a>),
    Format4(FdSelectFormat4<'a>),
}

impl<'a> FdSelect<'a> {
    ///Return the `FontData` used to resolve offsets for this table.
    pub fn offset_data(&self) -> FontData<'a> {
        match self {
            Self::Format0(item) => item.offset_data(),
            Self::Format3(item) => item.offset_data(),
            Self::Format4(item) => item.offset_data(),
        }
    }

    /// Format = 0.
    pub fn format(&self) -> u8 {
        match self {
            Self::Format0(item) => item.format(),
            Self::Format3(item) => item.format(),
            Self::Format4(item) => item.format(),
        }
    }
}

impl<'a> FontRead<'a> for FdSelect<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let format: u8 = data.read_at(0usize)?;
        match format {
            FdSelectFormat0Marker::FORMAT => Ok(Self::Format0(FontRead::read(data)?)),
            FdSelectFormat3Marker::FORMAT => Ok(Self::Format3(FontRead::read(data)?)),
            FdSelectFormat4Marker::FORMAT => Ok(Self::Format4(FontRead::read(data)?)),
            other => Err(ReadError::InvalidFormat(other.into())),
        }
    }
}

impl MinByteRange for FdSelect<'_> {
    fn min_byte_range(&self) -> Range<usize> {
        match self {
            Self::Format0(item) => item.min_byte_range(),
            Self::Format3(item) => item.min_byte_range(),
            Self::Format4(item) => item.min_byte_range(),
        }
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> FdSelect<'a> {
    fn dyn_inner<'b>(&'b self) -> &'b dyn SomeTable<'a> {
        match self {
            Self::Format0(table) => table,
            Self::Format3(table) => table,
            Self::Format4(table) => table,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
impl std::fmt::Debug for FdSelect<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.dyn_inner().fmt(f)
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for FdSelect<'a> {
    fn type_name(&self) -> &str {
        self.dyn_inner().type_name()
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        self.dyn_inner().get_field(idx)
    }
}

impl Format<u8> for FdSelectFormat0Marker {
    const FORMAT: u8 = 0;
}

/// FdSelect format 0.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct FdSelectFormat0Marker {
    fds_byte_len: usize,
}

impl FdSelectFormat0Marker {
    pub fn format_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u8::RAW_BYTE_LEN
    }

    pub fn fds_byte_range(&self) -> Range<usize> {
        let start = self.format_byte_range().end;
        start..start + self.fds_byte_len
    }
}

impl MinByteRange for FdSelectFormat0Marker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.fds_byte_range().end
    }
}

impl<'a> FontRead<'a> for FdSelectFormat0<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u8>();
        let fds_byte_len = cursor.remaining_bytes() / u8::RAW_BYTE_LEN * u8::RAW_BYTE_LEN;
        cursor.advance_by(fds_byte_len);
        cursor.finish(FdSelectFormat0Marker { fds_byte_len })
    }
}

/// FdSelect format 0.
pub type FdSelectFormat0<'a> = TableRef<'a, FdSelectFormat0Marker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> FdSelectFormat0<'a> {
    /// Format = 0.
    pub fn format(&self) -> u8 {
        let range = self.shape.format_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// FD selector array (one entry for each glyph).
    pub fn fds(&self) -> &'a [u8] {
        let range = self.shape.fds_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for FdSelectFormat0<'a> {
    fn type_name(&self) -> &str {
        "FdSelectFormat0"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("format", self.format())),
            1usize => Some(Field::new("fds", self.fds())),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for FdSelectFormat0<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

impl Format<u8> for FdSelectFormat3Marker {
    const FORMAT: u8 = 3;
}

/// FdSelect format 3.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct FdSelectFormat3Marker {
    ranges_byte_len: usize,
}

impl FdSelectFormat3Marker {
    pub fn format_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u8::RAW_BYTE_LEN
    }

    pub fn n_ranges_byte_range(&self) -> Range<usize> {
        let start = self.format_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }

    pub fn ranges_byte_range(&self) -> Range<usize> {
        let start = self.n_ranges_byte_range().end;
        start..start + self.ranges_byte_len
    }

    pub fn sentinel_byte_range(&self) -> Range<usize> {
        let start = self.ranges_byte_range().end;
        start..start + u16::RAW_BYTE_LEN
    }
}

impl MinByteRange for FdSelectFormat3Marker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.sentinel_byte_range().end
    }
}

impl<'a> FontRead<'a> for FdSelectFormat3<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u8>();
        let n_ranges: u16 = cursor.read()?;
        let ranges_byte_len = (n_ranges as usize)
            .checked_mul(FdSelectRange3::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(ranges_byte_len);
        cursor.advance::<u16>();
        cursor.finish(FdSelectFormat3Marker { ranges_byte_len })
    }
}

/// FdSelect format 3.
pub type FdSelectFormat3<'a> = TableRef<'a, FdSelectFormat3Marker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> FdSelectFormat3<'a> {
    /// Format = 3.
    pub fn format(&self) -> u8 {
        let range = self.shape.format_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Number of ranges.
    pub fn n_ranges(&self) -> u16 {
        let range = self.shape.n_ranges_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Range3 array.
    pub fn ranges(&self) -> &'a [FdSelectRange3] {
        let range = self.shape.ranges_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// Sentinel GID. Set equal to the number of glyphs in the font.
    pub fn sentinel(&self) -> u16 {
        let range = self.shape.sentinel_byte_range();
        self.data.read_at(range.start).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for FdSelectFormat3<'a> {
    fn type_name(&self) -> &str {
        "FdSelectFormat3"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("format", self.format())),
            1usize => Some(Field::new("n_ranges", self.n_ranges())),
            2usize => Some(Field::new(
                "ranges",
                traversal::FieldType::array_of_records(
                    stringify!(FdSelectRange3),
                    self.ranges(),
                    self.offset_data(),
                ),
            )),
            3usize => Some(Field::new("sentinel", self.sentinel())),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for FdSelectFormat3<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// Range struct for FdSelect format 3.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, bytemuck :: AnyBitPattern)]
#[repr(C)]
#[repr(packed)]
pub struct FdSelectRange3 {
    /// First glyph index in range.
    pub first: BigEndian<u16>,
    /// FD index for all glyphs in range.
    pub fd: u8,
}

impl FdSelectRange3 {
    /// First glyph index in range.
    pub fn first(&self) -> u16 {
        self.first.get()
    }

    /// FD index for all glyphs in range.
    pub fn fd(&self) -> u8 {
        self.fd
    }
}

impl FixedSize for FdSelectRange3 {
    const RAW_BYTE_LEN: usize = u16::RAW_BYTE_LEN + u8::RAW_BYTE_LEN;
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeRecord<'a> for FdSelectRange3 {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "FdSelectRange3",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("first", self.first())),
                1usize => Some(Field::new("fd", self.fd())),
                _ => None,
            }),
            data,
        }
    }
}

impl Format<u8> for FdSelectFormat4Marker {
    const FORMAT: u8 = 4;
}

/// FdSelect format 4.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct FdSelectFormat4Marker {
    ranges_byte_len: usize,
}

impl FdSelectFormat4Marker {
    pub fn format_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u8::RAW_BYTE_LEN
    }

    pub fn n_ranges_byte_range(&self) -> Range<usize> {
        let start = self.format_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn ranges_byte_range(&self) -> Range<usize> {
        let start = self.n_ranges_byte_range().end;
        start..start + self.ranges_byte_len
    }

    pub fn sentinel_byte_range(&self) -> Range<usize> {
        let start = self.ranges_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }
}

impl MinByteRange for FdSelectFormat4Marker {
    fn min_byte_range(&self) -> Range<usize> {
        0..self.sentinel_byte_range().end
    }
}

impl<'a> FontRead<'a> for FdSelectFormat4<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u8>();
        let n_ranges: u32 = cursor.read()?;
        let ranges_byte_len = (n_ranges as usize)
            .checked_mul(FdSelectRange4::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(ranges_byte_len);
        cursor.advance::<u32>();
        cursor.finish(FdSelectFormat4Marker { ranges_byte_len })
    }
}

/// FdSelect format 4.
pub type FdSelectFormat4<'a> = TableRef<'a, FdSelectFormat4Marker>;

#[allow(clippy::needless_lifetimes)]
impl<'a> FdSelectFormat4<'a> {
    /// Format = 4.
    pub fn format(&self) -> u8 {
        let range = self.shape.format_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Number of ranges.
    pub fn n_ranges(&self) -> u32 {
        let range = self.shape.n_ranges_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Range4 array.
    pub fn ranges(&self) -> &'a [FdSelectRange4] {
        let range = self.shape.ranges_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// Sentinel GID. Set equal to the number of glyphs in the font.
    pub fn sentinel(&self) -> u32 {
        let range = self.shape.sentinel_byte_range();
        self.data.read_at(range.start).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for FdSelectFormat4<'a> {
    fn type_name(&self) -> &str {
        "FdSelectFormat4"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("format", self.format())),
            1usize => Some(Field::new("n_ranges", self.n_ranges())),
            2usize => Some(Field::new(
                "ranges",
                traversal::FieldType::array_of_records(
                    stringify!(FdSelectRange4),
                    self.ranges(),
                    self.offset_data(),
                ),
            )),
            3usize => Some(Field::new("sentinel", self.sentinel())),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
#[allow(clippy::needless_lifetimes)]
impl<'a> std::fmt::Debug for FdSelectFormat4<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// Range struct for FdSelect format 4.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, bytemuck :: AnyBitPattern)]
#[repr(C)]
#[repr(packed)]
pub struct FdSelectRange4 {
    /// First glyph index in range.
    pub first: BigEndian<u32>,
    /// FD index for all glyphs in range.
    pub fd: BigEndian<u16>,
}

impl FdSelectRange4 {
    /// First glyph index in range.
    pub fn first(&self) -> u32 {
        self.first.get()
    }

    /// FD index for all glyphs in range.
    pub fn fd(&self) -> u16 {
        self.fd.get()
    }
}

impl FixedSize for FdSelectRange4 {
    const RAW_BYTE_LEN: usize = u32::RAW_BYTE_LEN + u16::RAW_BYTE_LEN;
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeRecord<'a> for FdSelectRange4 {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "FdSelectRange4",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("first", self.first())),
                1usize => Some(Field::new("fd", self.fd())),
                _ => None,
            }),
            data,
        }
    }
}
