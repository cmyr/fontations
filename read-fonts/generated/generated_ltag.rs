// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [language tag](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6ltag.html) table.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct LtagMarker {
    tag_ranges_byte_len: usize,
}

impl LtagMarker {
    pub fn version_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn flags_byte_range(&self) -> Range<usize> {
        let start = self.version_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn num_tags_byte_range(&self) -> Range<usize> {
        let start = self.flags_byte_range().end;
        start..start + u32::RAW_BYTE_LEN
    }

    pub fn tag_ranges_byte_range(&self) -> Range<usize> {
        let start = self.num_tags_byte_range().end;
        start..start + self.tag_ranges_byte_len
    }
}

impl TopLevelTable for Ltag<'_> {
    /// `ltag`
    const TAG: Tag = Tag::new(b"ltag");
}

impl<'a> FontRead<'a> for Ltag<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        cursor.advance::<u32>();
        cursor.advance::<u32>();
        let num_tags: u32 = cursor.read()?;
        let tag_ranges_byte_len = (num_tags as usize)
            .checked_mul(FTStringRange::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        cursor.advance_by(tag_ranges_byte_len);
        cursor.finish(LtagMarker {
            tag_ranges_byte_len,
        })
    }
}

/// The [language tag](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6ltag.html) table.
pub type Ltag<'a> = TableRef<'a, LtagMarker>;

impl<'a> Ltag<'a> {
    /// Table version; currently 1.
    pub fn version(&self) -> u32 {
        let range = self.shape.version_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Table flags; currently none defined.
    pub fn flags(&self) -> u32 {
        let range = self.shape.flags_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Number of language tags which follow.
    pub fn num_tags(&self) -> u32 {
        let range = self.shape.num_tags_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Range of each tag's string.
    pub fn tag_ranges(&self) -> &'a [FTStringRange] {
        let range = self.shape.tag_ranges_byte_range();
        self.data.read_array(range).unwrap()
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeTable<'a> for Ltag<'a> {
    fn type_name(&self) -> &str {
        "Ltag"
    }
    fn get_field(&self, idx: usize) -> Option<Field<'a>> {
        match idx {
            0usize => Some(Field::new("version", self.version())),
            1usize => Some(Field::new("flags", self.flags())),
            2usize => Some(Field::new("num_tags", self.num_tags())),
            3usize => Some(Field::new(
                "tag_ranges",
                traversal::FieldType::array_of_records(
                    stringify!(FTStringRange),
                    self.tag_ranges(),
                    self.offset_data(),
                ),
            )),
            _ => None,
        }
    }
}

#[cfg(feature = "experimental_traverse")]
impl<'a> std::fmt::Debug for Ltag<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn SomeTable<'a>).fmt(f)
    }
}

/// Offset and length of string in `ltag` table.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, bytemuck :: AnyBitPattern)]
#[repr(C)]
#[repr(packed)]
pub struct FTStringRange {
    /// Offset from the start of the table to the beginning of the string.
    pub offset: BigEndian<u16>,
    /// String length (in bytes).
    pub length: BigEndian<u16>,
}

impl FTStringRange {
    /// Offset from the start of the table to the beginning of the string.
    pub fn offset(&self) -> u16 {
        self.offset.get()
    }

    /// String length (in bytes).
    pub fn length(&self) -> u16 {
        self.length.get()
    }
}

impl FixedSize for FTStringRange {
    const RAW_BYTE_LEN: usize = u16::RAW_BYTE_LEN + u16::RAW_BYTE_LEN;
}

#[cfg(feature = "experimental_traverse")]
impl<'a> SomeRecord<'a> for FTStringRange {
    fn traverse(self, data: FontData<'a>) -> RecordResolver<'a> {
        RecordResolver {
            name: "FTStringRange",
            get_field: Box::new(move |idx, _data| match idx {
                0usize => Some(Field::new("offset", self.offset())),
                1usize => Some(Field::new("length", self.length())),
                _ => None,
            }),
            data,
        }
    }
}
