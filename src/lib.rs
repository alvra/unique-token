//! This crate provides a unique token type.

use triomphe::Arc;

/// This type represents a unique token.
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
/// Each token carries an [`Arc`]
/// (not the std one, but a variant from [`triomphe`]
/// that doesn't track weak references).
/// Equality checks are implemented as
/// a pointer check on the zero-sized type ([`unit`])
/// inside the `Arc`.
#[derive(Clone, Eq)]
pub struct Unique(Arc<()>);

impl Unique {
    /// Create a new token.
    ///
    /// All tokens created by this function compare unequal.
    #[inline]
    pub fn new() -> Self {
        Self(Arc::new(()))
    }
}

impl PartialEq for Unique {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl std::hash::Hash for Unique {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        usize::from(self).hash(state)
    }
}

impl std::fmt::Debug for Unique {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "0x{:X}", usize::from(self))
    }
}

impl From<&Unique> for usize {
    #[inline]
    fn from(token: &Unique) -> usize {
        Arc::as_ptr(&token.0) as usize
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
    fn test_into_usize() {
        let x = Unique::new();
        let y = Unique::new();
        assert_ne!(usize::from(&x), usize::from(&y));
        assert_eq!(usize::from(&x), usize::from(&x.clone()));
        assert_eq!(usize::from(&y), usize::from(&y.clone()));
    }
}
