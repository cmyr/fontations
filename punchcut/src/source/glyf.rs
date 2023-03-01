/*!
TrueType outlines loaded from the `glyf` table.

*/

mod deltas;
mod outline;
mod scaler;

pub use outline::Outline;
pub use scaler::Scaler;

pub use read_fonts::types::Point;

use read_fonts::types::{F26Dot6, Fixed};

/// Point that actually represents a vector holding a variation delta.
pub type Delta = Point<Fixed>;

/// Context for loading for TrueType glyphs.
#[derive(Clone, Default, Debug)]
pub struct Context {
    /// Unscaled points.
    unscaled: Vec<Point<i32>>,
    /// Original scaled points.
    original: Vec<Point<F26Dot6>>,
    /// Storage for simple glyph deltas.
    deltas: Vec<Delta>,
    /// Storage for composite glyph deltas.
    composite_deltas: Vec<Delta>,
    /// Temporary point storage that is used for storing intermediate
    /// interpolated values while computing deltas.
    working_points: Vec<Point<Fixed>>,
}

impl Context {
    /// Creates a new context.
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{Context, Outline, Scaler};
    use crate::{font::*, GlyphId};

    use read_fonts::test_data::test_fonts;
    use read_fonts::types::F26Dot6;

    #[test]
    fn vazirmatin_var() {
        let font = FontRef::new(test_fonts::VAZIRMATN_VAR).unwrap();
        let outlines = crate::test::parse_glyph_outlines(test_fonts::VAZIRMATN_VAR_GLYPHS);
        let mut cx = Context::new();
        let mut outline = Outline::new();
        for expected_outline in &outlines {
            #[cfg(feature = "hinting")]
            let mut scaler = Scaler::new(
                &mut cx,
                &font,
                None,
                expected_outline.size,
                None,
                &expected_outline.coords,
            )
            .unwrap();
            #[cfg(not(feature = "hinting"))]
            let mut scaler = Scaler::new(
                &mut cx,
                &font,
                None,
                expected_outline.size,
                &expected_outline.coords,
            )
            .unwrap();
            scaler
                .load(expected_outline.glyph_id, &mut outline)
                .unwrap();
            assert_eq!(&outline.points, &expected_outline.points);
            assert_eq!(&outline.contours, &expected_outline.contours);
            assert_eq!(&outline.flags, &expected_outline.flags);
        }
    }
}
