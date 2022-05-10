//! This crate provides a unique token type.

#![forbid(unsafe_code)]

use std::sync::atomic::{AtomicU64, Ordering};

/// This type represents a unique token.
///
/// Each call to [`Unique::new()`] returns a unique value.
/// The only way to create a token that compares equal is to
/// clone or copy an existing token.
///
/// # Examples
///
/// ```
/// use unique_token::Unique;
///
/// let x = Unique::new();
/// let y = Unique::new();
///
/// // clones are equal
/// assert_eq!(x, x.clone());
/// assert_eq!(y, y.clone());
///
/// // tokens from different calls are unequal
/// assert_ne!(x, y);
/// ```
///
/// # Implementation
///
/// Each token is provided with a unique ID
/// by incrementing a static [`AtomicU64`](std::sync::atomic::AtomicU64).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Unique(u64);

impl Unique {
    /// Create a new token.
    ///
    /// All tokens created by this function compare unequal.
    ///
    /// # Panics
    ///
    /// This function panics if [`u64::MAX`]
    /// unique tokens have been created.
    /// In practice, this should never happen;
    /// creating one token per nanosecond allows for
    /// a runtime of almost six centuries.
    #[inline]
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);

        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        if id == 0 {
            panic!("id overflow")
        }
        Self(id)
    }
}

impl std::fmt::Debug for Unique {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let len = (u64::BITS / 4) as usize;
        write!(fmt, "0x{:0len$X}", u64::from(self))
    }
}

impl From<&Unique> for u64 {
    #[inline]
    fn from(token: &Unique) -> u64 {
        token.0
    }
}

#[cfg(test)]
mod tests {
    use super::Unique;

    #[test]
    fn test_eq() {
        let x = Unique::new();
        let y = Unique::new();
        assert_ne!(&x, &y);
        assert_eq!(&x, &x.clone());
        assert_eq!(&y, &y.clone());
    }

    #[test]
    fn test_into_u64() {
        let x = Unique::new();
        let y = Unique::new();
        assert_ne!(u64::from(&x), u64::from(&y));
        assert_eq!(u64::from(&x), u64::from(&x.clone()));
        assert_eq!(u64::from(&y), u64::from(&y.clone()));
    }
}
