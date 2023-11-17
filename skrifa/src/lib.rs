//! A robust, ergonomic, high performance crate for OpenType fonts.
//!  
//! Skrifa is a mid level library that provides access to various types
//! of [`metadata`](MetadataProvider) contained in a font as well as support
//! for [`scaling`](scale) (extraction) of glyph outlines.
//!
//! It is described as "mid level" because the library is designed to sit
//! above low level font parsing (provided by [`read-fonts`](https://crates.io/crates/read-fonts))
//! and below a higher level text layout engine.
//!
//! See the [readme](https://github.com/googlefonts/fontations/blob/main/skrifa/README.md)
//! for additional details.

/// Expose our "raw" underlying parser crate.
pub extern crate read_fonts as raw;

pub mod attribute;
pub mod charmap;
pub mod font;
pub mod instance;
pub mod metrics;
pub mod scale;
pub mod setting;
pub mod string;

mod outline;
mod provider;
mod small_array;
mod variation;

pub use outline::{
    Hinting, Outline, OutlineCollection, OutlineFormat, Scaler, ScalerError, ScalerMetrics,
};
pub use variation::{Axis, AxisCollection, NamedInstance, NamedInstanceCollection};

#[doc(inline)]
pub use {
    attribute::{Attributes, Stretch, Style, Weight},
    charmap::{Charmap, MapVariant, MappingIndex as CharmapIndex},
    instance::{Location, LocationRef, NormalizedCoord, Size},
    font::FontRef,
    read_fonts::TableProvider,
    metrics::{Metrics, GlyphMetrics, Decoration as DecorationMetrics, BoundingBox},
    setting::{Setting, VariationSetting},
    string::{StringId, LocalizedString, LocalizedStrings},
};

/// Collection of embedded bitmap strikes.
pub struct StrikeCollection {}

/// Embedded bitmaps at a particular resolution and size.
pub struct Strike {}


/// Useful collection of common types suitable for glob importing.
pub mod prelude {
    #[doc(no_inline)]
    pub use super::{
        font::{FontRef, UniqueId},
        instance::{LocationRef, NormalizedCoord, Size},
        GlyphId, MetadataProvider, Tag,
    };
}

pub use read_fonts::types::{GlyphId, Tag};

#[doc(inline)]
pub use provider::MetadataProvider;

/// Limit for recursion when loading TrueType composite glyphs.
const GLYF_COMPOSITE_RECURSION_LIMIT: usize = 32;
