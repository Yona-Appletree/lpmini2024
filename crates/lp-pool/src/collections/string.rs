use super::vec::LpVec;
use crate::error::AllocError;

/// Pool-backed String
pub struct LpString {
    vec: LpVec<u8>,
}

impl LpString {
    pub fn new() -> Self {
        LpString { vec: LpVec::new() }
    }

    /// Create a new LpString with a scope identifier for metadata tracking
    #[cfg(feature = "alloc-meta")]
    pub fn new_with_scope(scope: Option<&'static str>) -> Self {
        LpString {
            vec: LpVec::new_with_scope(scope),
        }
    }

    /// Create a new LpString with a scope identifier for metadata tracking
    #[cfg(not(feature = "alloc-meta"))]
    pub fn new_with_scope(_scope: Option<&'static str>) -> Self {
        Self::new()
    }

    pub fn try_push_str(&mut self, s: &str) -> Result<(), AllocError> {
        for byte in s.bytes() {
            self.vec.try_push(byte)?;
        }
        Ok(())
    }

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.vec.as_raw_slice()) }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// Create a new LpString from a string slice
    pub fn try_from_str(s: &str) -> Result<Self, AllocError> {
        Self::try_from_str_with_scope(s, None)
    }

    /// Create a new LpString from a string slice with a scope
    #[cfg(feature = "alloc-meta")]
    pub fn try_from_str_with_scope(
        s: &str,
        scope: Option<&'static str>,
    ) -> Result<Self, AllocError> {
        let mut string = LpString::new_with_scope(scope);
        string.try_push_str(s)?;
        Ok(string)
    }

    /// Create a new LpString from a string slice with a scope
    #[cfg(not(feature = "alloc-meta"))]
    pub fn try_from_str_with_scope(
        s: &str,
        _scope: Option<&'static str>,
    ) -> Result<Self, AllocError> {
        let mut string = LpString::new();
        string.try_push_str(s)?;
        Ok(string)
    }
}

impl Default for LpString {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for LpString {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for LpString {}

impl PartialEq<str> for LpString {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for LpString {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialOrd for LpString {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LpString {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl core::fmt::Debug for LpString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

impl core::fmt::Display for LpString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use core::ptr::NonNull;

    use super::*;
    use crate::memory_pool::LpMemoryPool;

    fn setup_pool() -> LpMemoryPool {
        let mut memory = [0u8; 16384];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        unsafe { LpMemoryPool::new(memory_ptr, 16384).unwrap() }
    }

    #[test]
    fn test_string_new() {
        let pool = setup_pool();
        pool.run(|| {
            let s = LpString::new();
            assert_eq!(s.len(), 0);
            assert_eq!(s.as_str(), "");
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_string_push_str() {
        let pool = setup_pool();
        pool.run(|| {
            let mut s = LpString::new();
            s.try_push_str("hello")?;
            assert_eq!(s.len(), 5);
            assert_eq!(s.as_str(), "hello");
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_string_multiple_push() {
        let pool = setup_pool();
        pool.run(|| {
            let mut s = LpString::new();
            s.try_push_str("hello")?;
            s.try_push_str(" ")?;
            s.try_push_str("world")?;
            assert_eq!(s.as_str(), "hello world");
            assert_eq!(s.len(), 11);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_string_empty() {
        let pool = setup_pool();
        pool.run(|| {
            let mut s = LpString::new();
            s.try_push_str("")?;
            assert_eq!(s.len(), 0);
            assert_eq!(s.as_str(), "");
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[cfg(feature = "alloc-meta")]
    #[test]
    fn test_string_with_scope() {
        let pool = setup_pool();
        pool.run(|| {
            let mut s = LpString::new_with_scope(Some("test_scope"));
            s.try_push_str("hello")?;
            assert_eq!(s.as_str(), "hello");
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    // Test for try_from_str
    #[test]
    fn test_string_try_from_str() {
        let pool = setup_pool();
        pool.run(|| {
            let s = LpString::try_from_str("hello")?;
            assert_eq!(s.as_str(), "hello");

            let s2 = LpString::try_from_str("world")?;
            assert_eq!(s2.as_str(), "world");
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    // Test for comparison traits
    #[test]
    fn test_string_equality() {
        let pool = setup_pool();
        pool.run(|| {
            let s1 = LpString::try_from_str("hello")?;
            let s2 = LpString::try_from_str("hello")?;
            let s3 = LpString::try_from_str("world")?;

            assert_eq!(s1, s2);
            assert_ne!(s1, s3);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_string_ordering() {
        let pool = setup_pool();
        pool.run(|| {
            let s1 = LpString::try_from_str("apple")?;
            let s2 = LpString::try_from_str("banana")?;
            let s3 = LpString::try_from_str("cherry")?;

            assert!(s1 < s2);
            assert!(s2 < s3);
            assert!(s1 < s3);
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_string_eq_str() {
        let pool = setup_pool();
        pool.run(|| {
            let s = LpString::try_from_str("hello")?;
            assert_eq!(s, "hello");
            assert_ne!(s, "world");
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }
}
