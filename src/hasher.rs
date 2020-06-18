use core::fmt::Debug;
use generic_array::{ArrayLength, GenericArray};
use generic_array::typenum::marker_traits::Unsigned;

/// Size marker trait.
pub trait Size: ArrayLength<u8> + Debug + Default + Eq + Send + Sync + 'static {}

impl<T: ArrayLength<u8> + Debug + Default + Eq + Send + Sync + 'static> Size for T {}

/// Stack allocated digest trait.
pub trait Digest<S: Size>:
    AsRef<[u8]>
    + From<GenericArray<u8, S>>
    + Into<GenericArray<u8, S>>
    + Clone
    + Debug
    + Default
    + Eq
    + Send
    + Sync
    + 'static
{
}

/// Trait implemented by a hash function implementation.
pub trait Hasher: Default {
    /// Digest size.
    type Size: Size;

    /// Digest type.
    type Digest: Digest<Self::Size>;

    /// Consume input and update internal state.
    fn update(&mut self, input: &[u8]);

    /// Returns the internal state digest.
    fn finalize(&self) -> Self::Digest;

    /// Reset the internal hasher state.
    fn reset(&mut self);

    /// Returns the size of the digest.
    fn size() -> u8 {
        Self::Size::to_u8()
    }

    /// Returns the digest of the input.
    fn digest(input: &[u8]) -> Self::Digest
    where
        Self: Sized,
    {
        let mut hasher = Self::default();
        hasher.update(input);
        hasher.finalize()
    }
}

/// New type wrapper for a hasher that implements the `std::io::Write` trait.
#[cfg(feature = "std")]
pub struct WriteHasher<H: Hasher>(H);

#[cfg(feature = "std")]
impl<H: Hasher> std::io::Write for WriteHasher<H> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
