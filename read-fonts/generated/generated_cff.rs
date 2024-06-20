// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// [Compact Font Format](https://learn.microsoft.com/en-us/typography/opentype/spec/cff) table header
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct CffHeaderMarker {
    _padding_byte_len: usize,
    trailing_data_byte_len: usize,
}

impl CffHeaderMarker {
    fn major_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u8::RAW_BYTE_LEN
    }
    fn minor_byte_range(&self) -> Range<usize> {
        let start = self.major_byte_range().end;
        start..start + u8::RAW_BYTE_LEN
    }
    fn hdr_size_byte_range(&self) -> Range<usize> {
        let start = self.minor_byte_range().end;
        start..start + u8::RAW_BYTE_LEN
    }
    fn off_size_byte_range(&self) -> Range<usize> {
        let start = self.hdr_size_byte_range().end;
        start..start + u8::RAW_BYTE_LEN
    }
    fn _padding_byte_range(&self) -> Range<usize> {
        let start = self.off_size_byte_range().end;
        start..start + self._padding_byte_len
    }
    fn trailing_data_byte_range(&self) -> Range<usize> {
        let start = self._padding_byte_range().end;
        start..start + self.trailing_data_byte_len
    }
}

impl<'a> FontRead<'a> for CffHeader<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u8>();
        cursor.advance::<u8>();
        let hdr_size: u8 = cursor.read()?;
        cursor.advance::<u8>();
        let _padding_byte_len =
            (transforms::subtract(hdr_size, 4_usize)).saturating_mul(u8::RAW_BYTE_LEN);
        cursor.advance_by(_padding_byte_len);
        let trailing_data_byte_len = cursor.remaining_bytes() / u8::RAW_BYTE_LEN * u8::RAW_BYTE_LEN;
        cursor.advance_by(trailing_data_byte_len);
        cursor.finish(CffHeaderMarker {
            _padding_byte_len,
            trailing_data_byte_len,
        })
    }
}

/// [Compact Font Format](https://learn.microsoft.com/en-us/typography/opentype/spec/cff) table header
pub type CffHeader<'a> = TableRef<'a, CffHeaderMarker>;

impl<'a> CffHeader<'a> {
    /// Format major version (starting at 1).
    pub fn major(&self) -> u8 {
        let range = self.shape.major_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Format minor version (starting at 0).
    pub fn minor(&self) -> u8 {
        let range = self.shape.minor_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Header size (bytes).
    pub fn hdr_size(&self) -> u8 {
        let range = self.shape.hdr_size_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Absolute offset size.
    pub fn off_size(&self) -> u8 {
        let range = self.shape.off_size_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Padding bytes before the start of the Name INDEX.
    pub fn _padding(&self) -> &'a [u8] {
        let range = self.shape._padding_byte_range();
        self.data.read_array(range).unwrap()
    }

    /// Remaining table data.
    pub fn trailing_data(&self) -> &'a [u8] {
        let range = self.shape.trailing_data_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "traversal")]
impl<'a> SomeTable<'a> for CffHeader<'a> {
    fn type_name(&self) -> &str {
        "CffHeader"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("major", self.major())),
            1usize => Some(Field::new("minor", self.minor())),
            2usize => Some(Field::new("hdr_size", self.hdr_size())),
            3usize => Some(Field::new("off_size", self.off_size())),
            4usize => Some(Field::new("_padding", self._padding())),
            5usize => Some(Field::new("trailing_data", self.trailing_data())),
            _ => None,
        }
    }
}

#[cfg(feature = "traversal")]
impl<'a> std::fmt::Debug for CffHeader<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}
