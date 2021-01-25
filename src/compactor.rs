use lasso::{Rodeo, Spur};
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// An identifier. Used to name things.
pub struct Compact(Spur);

/// Keeps all the strings used as compacts interned in one place to reduce memory usage.
pub struct Compactor(RefCell<Rodeo>);

/// A scoped reference to a compact str.
#[derive(Debug)]
pub struct CompactRef<'a>(Ref<'a, str>);

impl Compactor {
    #[inline(always)]
    pub fn new() -> Compactor {
        Compactor(RefCell::new(Rodeo::new()))
    }

    /// Create a compact from the provided string, writing it if necessary.
    #[inline(always)]
    pub fn create(&self, val: impl AsRef<str>) -> Compact {
        Compact(self.0.borrow_mut().get_or_intern(val))
    }

    /// Resolves a compact we created to a scoped reference to its contents.
    #[inline(always)]
    pub fn resolve<'a>(&self, key: &'a Compact) -> CompactRef {
        CompactRef(Ref::map(self.0.borrow(), |r| r.resolve(&key.0)))
    }
}

impl<'a> Deref for CompactRef<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        self.0.deref()
    }
}

impl<'a> fmt::Display for CompactRef<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(fmt)
    }
}
