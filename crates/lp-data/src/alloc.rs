// Compatibility utilities replacing the old lp_pool allocator wrappers.
// These types provide a fallible-looking API but are backed by standard
// allocation primitives. They integrate with the lp_alloc global allocator
// to keep soft limits intact during tests.

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;
use core::ops::{Deref, DerefMut};

use lp_alloc::{self, AllocLimitError};

pub type AllocError = AllocLimitError;

pub type LpBoxDyn<T> = Box<T>;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LpString(String);

impl LpString {
    pub fn new() -> Self {
        Self(String::new())
    }

    pub fn try_from_str(value: &str) -> Result<Self, AllocLimitError> {
        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LpString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for LpString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LpString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<LpString> for String {
    fn from(value: LpString) -> Self {
        value.0
    }
}

impl From<String> for LpString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a str> for LpString {
    fn from(value: &'a str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LpVec<T>(Vec<T>);

impl<T> LpVec<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn try_push(&mut self, value: T) -> Result<(), AllocLimitError> {
        self.0.push(value);
        Ok(())
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), AllocLimitError> {
        self.0.reserve(additional);
        Ok(())
    }

    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
}

impl<T> Deref for LpVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for LpVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Vec<T>> for LpVec<T> {
    fn from(vec: Vec<T>) -> Self {
        Self(vec)
    }
}

impl<T> Into<Vec<T>> for LpVec<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}

pub struct GlobalAllocAllowToken {
    previous_limit: usize,
}

impl Drop for GlobalAllocAllowToken {
    fn drop(&mut self) {
        lp_alloc::set_soft_limit(self.previous_limit);
    }
}

pub fn enter_global_alloc_allowance() -> GlobalAllocAllowToken {
    let prev = lp_alloc::soft_limit();
    lp_alloc::set_soft_limit(usize::MAX);
    GlobalAllocAllowToken {
        previous_limit: prev,
    }
}

#[macro_export]
macro_rules! lp_box_dyn {
    ($value:expr, dyn $trait:path) => {{
        let boxed: ::alloc::boxed::Box<$trait> = ::alloc::boxed::Box::new($value);
        ::core::result::Result::<_, ::lp_alloc::AllocLimitError>::Ok(boxed)
    }};
}
