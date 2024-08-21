//! Custom array types

#![deny(clippy::arithmetic_side_effects)]

use bytemuck::AnyBitPattern;
use font_types::FixedSize;

use crate::read::{ComputeSize, FontReadWithArgs, ReadArgs, VarSize};
use crate::{FontData, FontRead, ReadError};

/// An array whose items size is not known at compile time.
///
/// This requires the inner type to implement [`FontReadWithArgs`] as well as
/// [`ComputeSize`].
///
/// At runtime, `Args` are provided which will be used to compute the size
/// of each item; this size is then used to compute the positions of the items
/// within the underlying data, from which they will be read lazily.
#[derive(Clone)]
pub struct ComputedArray<'a, T: ReadArgs> {
    // the length of each item
    item_len: usize,
    len: usize,
    data: FontData<'a>,
    args: T::Args,
}

impl<'a, T: ComputeSize> ComputedArray<'a, T> {
    pub fn new(data: FontData<'a>, args: T::Args) -> Result<Self, ReadError> {
        let item_len = T::compute_size(&args)?;
        let len = data.len().checked_div(item_len).unwrap_or(0);
        Ok(ComputedArray {
            item_len,
            len,
            data,
            args,
        })
    }

    /// The number of items in the array
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<'a, T: ReadArgs> ReadArgs for ComputedArray<'a, T> {
    type Args = T::Args;
}

impl<'a, T> FontReadWithArgs<'a> for ComputedArray<'a, T>
where
    T: ComputeSize + FontReadWithArgs<'a>,
    T::Args: Copy,
{
    fn read_with_args(data: FontData<'a>, args: &Self::Args) -> Result<Self, ReadError> {
        Self::new(data, *args)
    }
}

impl<'a, T> ComputedArray<'a, T>
where
    T: FontReadWithArgs<'a>,
    T::Args: Copy + 'static,
{
    pub fn iter(&self) -> impl Iterator<Item = Result<T, ReadError>> + 'a {
        let mut i = 0;
        let data = self.data;
        let args = self.args;
        let item_len = self.item_len;
        let len = self.len;

        std::iter::from_fn(move || {
            if i == len {
                return None;
            }
            let item_start = item_len.checked_mul(i)?;
            i = i.checked_add(1)?;
            let data = data.split_off(item_start)?;
            Some(T::read_with_args(data, &args))
        })
    }

    pub fn get(&self, idx: usize) -> Result<T, ReadError> {
        // if our items are zero-length (itself a degenerate case) we want
        // to always return oob here since otherwise we will succeed for all indices
        if self.item_len == 0 {
            return Err(ReadError::OutOfBounds);
        }
        let item_start = idx
            .checked_mul(self.item_len)
            .ok_or(ReadError::OutOfBounds)?;
        self.data
            .split_off(item_start)
            .ok_or(ReadError::OutOfBounds)
            .and_then(|data| T::read_with_args(data, &self.args))
    }
}

impl<T: ReadArgs> std::fmt::Debug for ComputedArray<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DynSizedArray")
            .field("bytes", &self.data)
            .finish()
    }
}

/// An array of items of non-uniform length.
///
/// Random access into this array cannot be especially efficient, since it requires
/// a linear scan.
pub struct VarLenArray<'a, T> {
    data: FontData<'a>,
    phantom: std::marker::PhantomData<*const T>,
}

impl<'a, T: FontRead<'a> + VarSize> VarLenArray<'a, T> {
    /// Return the item at the provided index.
    ///
    /// This performs a linear search.
    pub fn get(&self, idx: usize) -> Option<Result<T, ReadError>> {
        let mut pos = 0usize;
        for _ in 0..idx {
            pos = pos.checked_add(T::read_len_at(self.data, pos)?)?;
        }
        self.data.split_off(pos).map(T::read)
    }

    /// Return an iterator over this array's items.
    pub fn iter(&self) -> impl Iterator<Item = Result<T, ReadError>> + 'a {
        let mut data = self.data;
        std::iter::from_fn(move || {
            if data.is_empty() {
                return None;
            }

            let item_len = T::read_len_at(data, 0)?;
            let next = T::read(data);
            data = data.split_off(item_len)?;
            Some(next)
        })
    }
}

impl<'a, T> FontRead<'a> for VarLenArray<'a, T> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        Ok(VarLenArray {
            data,
            phantom: core::marker::PhantomData,
        })
    }
}

impl<'a, T: AnyBitPattern> ReadArgs for &'a [T] {
    type Args = u16;
}

impl<'a, T: AnyBitPattern + FixedSize> FontReadWithArgs<'a> for &'a [T] {
    fn read_with_args(data: FontData<'a>, args: &u16) -> Result<Self, ReadError> {
        let len = (*args as usize)
            .checked_mul(T::RAW_BYTE_LEN)
            .ok_or(ReadError::OutOfBounds)?;
        data.read_array(0..len)
    }
}

#[cfg(test)]
mod tests {
    use crate::tables::variations::VariationRegion;

    use super::*;

    #[test]
    fn zero_len_get() {
        let non_empty_data = FontData::new(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let array = ComputedArray::<VariationRegion>::new(non_empty_data, 0).unwrap();

        assert!(matches!(array.get(usize::MAX), Err(ReadError::OutOfBounds)));
        assert!(matches!(array.get(1), Err(ReadError::OutOfBounds)));
        assert!(matches!(array.get(0), Err(ReadError::OutOfBounds)));
    }
}
