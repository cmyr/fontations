//! a trait for things that can serve font tables

use font_types::Tag;

use crate::{tables, FontData, FontRead, ReadError};

/// An interface for accessing tables from a font (or font-like object)
pub trait TableProvider {
    fn data_for_tag(&self, tag: Tag) -> Option<FontData>;

    fn expect_data_for_tag(&self, tag: Tag) -> Result<FontData, ReadError> {
        self.data_for_tag(tag).ok_or(ReadError::TableIsMissing(tag))
    }

    fn head(&self) -> Result<tables::head::Head, ReadError> {
        self.expect_data_for_tag(tables::head::TAG)
            .and_then(FontRead::read)
    }

    //fn name(&self) -> Option<name::Name> {
    //self.data_for_tag(name::TAG).and_then(name::Name::read)
    //}

    //fn hhea(&self) -> Option<hhea::Hhea> {
    //self.data_for_tag(hhea::TAG).and_then(hhea::Hhea::read)
    //}

    //fn hmtx(&self) -> Option<hmtx::Hmtx> {
    ////FIXME: should we make the user pass these in?
    //let num_glyphs = self.maxp().map(|maxp| maxp.num_glyphs())?;
    //let number_of_h_metrics = self.hhea().map(|hhea| hhea.number_of_h_metrics())?;
    //self.data_for_tag(hmtx::TAG)
    //.and_then(|data| hmtx::Hmtx::read_with_args(data, &(num_glyphs, number_of_h_metrics)))
    //.map(|(table, _)| table)
    //}

    //fn maxp(&self) -> Option<maxp::Maxp> {
    //self.data_for_tag(maxp::TAG).and_then(maxp::Maxp::read)
    //}

    //fn post(&self) -> Option<post::Post> {
    //self.data_for_tag(post::TAG).and_then(post::Post::read)
    //}

    //fn stat(&self) -> Option<stat::Stat> {
    //self.data_for_tag(stat::TAG).and_then(stat::Stat::read)
    //}

    //fn loca(&self, num_glyphs: u16, is_long: bool) -> Option<loca::Loca> {
    //let bytes = self.data_for_tag(loca::TAG)?;
    //loca::Loca::read(bytes, num_glyphs, is_long)
    //}

    //fn glyf(&self) -> Option<glyf::Glyf> {
    //self.data_for_tag(glyf::TAG).and_then(glyf::Glyf::read)
    //}

    //fn cmap(&self) -> Option<cmap::Cmap> {
    //self.data_for_tag(cmap::TAG).and_then(cmap::Cmap::read)
    //}

    //fn gdef(&self) -> Option<gdef::Gdef> {
    //self.data_for_tag(gdef::TAG).and_then(gdef::Gdef::read)
    //}

    fn gpos(&self) -> Result<tables::gpos::Gpos, ReadError> {
        self.expect_data_for_tag(tables::gpos::TAG)
            .and_then(FontRead::read)
    }
}
