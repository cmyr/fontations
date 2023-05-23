//! Identifiers for specific tables
//!
//! These are used to record the type of certain serialized tables & subtables
//! that may require special attention while compiling the object graph.

use font_types::Tag;
use read::TopLevelTable;

use crate::tables::layout::LookupType;

/// A marker for identifying the original source of various compiled tables.
///
/// In the general case, once a table has been compiled we do not need to know
/// what the bytes represent; however in certain special cases we do need this
/// information, in order to try alternate compilation strategies.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum TableType {
    /// An unknown table
    #[default]
    Unknown,
    /// A table with a given name (the name is used for debugging)
    Named(&'static str),
    /// A top-level table
    TopLevel(Tag),
    GposLookup(u16),
    GsubLookup(u16),
}

impl TableType {
    pub(crate) const GSUB: TableType = TableType::TopLevel(crate::tables::gsub::Gsub::TAG);
    pub(crate) const GPOS: TableType = TableType::TopLevel(crate::tables::gpos::Gpos::TAG);
}

impl From<LookupType> for TableType {
    fn from(src: LookupType) -> TableType {
        match src {
            LookupType::Gpos(type_) => TableType::GposLookup(type_),
            LookupType::Gsub(type_) => TableType::GsubLookup(type_),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tables::{gpos, gsub};
    use crate::FontWrite;

    use super::*;

    #[test]
    fn tagged_table_type() {
        assert_eq!(gsub::Gsub::default().table_type(), TableType::GSUB);
        assert_eq!(gpos::Gpos::default().table_type(), TableType::GPOS);

        assert_eq!(
            crate::tables::name::Name::default().table_type(),
            TableType::TopLevel(Tag::new(b"name"))
        );
    }

    #[test]
    fn promotable() {
        assert_eq!(
            gsub::SubstitutionLookup::Single(Default::default()).table_type(),
            TableType::GsubLookup(1)
        );
        assert_eq!(
            gsub::SubstitutionLookup::Extension(Default::default()).table_type(),
            TableType::GsubLookup(7)
        );
    }
}
