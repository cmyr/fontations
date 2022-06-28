// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::compile::*;

#[allow(unused_imports)]
use font_types::*;

#[derive(Debug, PartialEq)]
pub struct TableDirectory {
    pub sfnt_version: u32,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
    pub table_records: Vec<TableRecord>,
}

impl FontWrite for TableDirectory {
    fn write_into(&self, writer: &mut TableWriter) {
        self.sfnt_version.write_into(writer);
        u16::try_from(self.table_records.len())
            .unwrap()
            .write_into(writer);
        self.search_range.write_into(writer);
        self.entry_selector.write_into(writer);
        self.range_shift.write_into(writer);
        self.table_records.write_into(writer);
    }
}

#[derive(Debug, PartialEq)]
pub struct TableRecord {
    pub tag: Tag,
    pub checksum: u32,
    pub offset: u32,
    pub len: u32,
}

impl FontWrite for TableRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.tag.write_into(writer);
        self.checksum.write_into(writer);
        self.offset.write_into(writer);
        self.len.write_into(writer);
    }
}
