//! small utilities used in tests

use crate::{FontData, Scalar};

/// A convenience type for generating a buffer of big-endian bytes.
#[derive(Debug, Clone, Default)]
pub struct BeBuffer(Vec<u8>);

impl BeBuffer {
    pub fn new() -> Self {
        Default::default()
    }

    /// Write any scalar to this buffer.
    pub fn push(&mut self, item: impl Scalar) {
        self.0.extend(item.to_raw().as_ref())
    }

    /// Write multiple scalars into the buffer
    pub fn extend<T: Scalar>(&mut self, iter: impl IntoIterator<Item = T>) {
        for item in iter {
            self.0.extend(item.to_raw().as_ref())
        }
    }

    pub fn font_data(&self) -> FontData {
        FontData::new(&self.0)
    }
}

impl std::ops::Deref for BeBuffer {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
