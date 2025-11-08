use super::vec::LpVec;
use crate::error::AllocError;
use core::fmt::{self, Write};

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

    pub fn try_push_char(&mut self, ch: char) -> Result<(), AllocError> {
        let mut buf = [0u8; 4];
        let encoded = ch.encode_utf8(&mut buf);
        self.try_push_str(encoded)
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

    pub fn clear(&mut self) {
        self.vec.clear();
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), AllocError> {
        let target = self
            .vec
            .len()
            .checked_add(additional)
            .ok_or(AllocError::PoolExhausted)?;
        self.vec.try_reserve(target)
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

impl Write for LpString {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.try_push_str(s).map_err(|_| fmt::Error)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.try_push_char(c).map_err(|_| fmt::Error)
    }
}

#[cfg(test)]
mod tests {
    use core::ptr::NonNull;

    use super::*;
    use crate::memory_pool::LpMemoryPool;
    use crate::with_global_alloc;

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
            Ok::<(), AllocError>(())
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

    // === Comprehensive Edge Case Tests ===

    #[test]
    fn test_string_very_long() {
        let pool = setup_pool();
        pool.run(|| {
            let mut s = LpString::new();

            // Build a long string that requires multiple reallocations
            for i in 0..100 {
                let chunk = with_global_alloc(|| alloc::format!("Line {} ", i));
                s.try_push_str(&chunk)?;
            }

            assert!(s.len() > 500);
            assert!(s.as_str().contains("Line 0"));
            assert!(s.as_str().contains("Line 99"));

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_string_unicode() {
        let pool = setup_pool();
        pool.run(|| {
            let mut s = LpString::new();
            s.try_push_str("Hello ")?;
            s.try_push_str("🦀")?; // Rust crab emoji (4 bytes)
            s.try_push_str(" World")?;
            s.try_push_str(" 世界")?; // Chinese characters

            assert_eq!(s.as_str(), "Hello 🦀 World 世界");
            assert!(s.len() > 17); // More than ASCII length

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_string_is_empty() {
        let pool = setup_pool();
        pool.run(|| {
            let s = LpString::new();
            assert!(s.is_empty());

            let mut s2 = LpString::new();
            s2.try_push_str("x")?;
            assert!(!s2.is_empty());

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_string_repeated_operations() {
        let pool = setup_pool();
        pool.run(|| {
            let mut s = LpString::new();

            for _ in 0..10 {
                s.try_push_str("hello")?;
            }

            assert_eq!(s.len(), 50);
            assert_eq!(
                s.as_str(),
                "hellohellohellohellohellohellohellohellohellohello"
            );

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_string_from_str_with_scope() {
        let pool = setup_pool();
        pool.run(|| {
            let s = LpString::try_from_str_with_scope("test", Some("test_scope"))?;
            assert_eq!(s.as_str(), "test");
            assert_eq!(s.len(), 4);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_string_memory_is_freed() {
        let pool = setup_pool();
        let before = pool.used_bytes().unwrap();

        pool.run(|| {
            {
                let mut s1 = LpString::new();
                s1.try_push_str("This is a test string")?;

                let mut s2 = LpString::new();
                s2.try_push_str("Another test string")?;
                // Both dropped here
            }

            let after = pool.used_bytes().unwrap();
            assert_eq!(after, before, "Memory should be freed after strings drop");

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_string_debug_display() {
        let pool = setup_pool();
        pool.run(|| {
            let s = LpString::try_from_str("test")?;

            // Test Debug formatting
            let debug_str = with_global_alloc(|| alloc::format!("{:?}", s));
            assert_eq!(debug_str, "\"test\"");

            // Test Display formatting
            let display_str = with_global_alloc(|| alloc::format!("{}", s));
            assert_eq!(display_str, "test");

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_string_eq_with_str_ref() {
        let pool = setup_pool();
        pool.run(|| {
            let s = LpString::try_from_str("hello")?;

            let hello_str: &str = "hello";
            let world_str: &str = "world";

            assert!(s == "hello");
            assert!(s == hello_str);
            assert!(s != "world");
            assert!(s != world_str);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_string_empty_then_push() {
        let pool = setup_pool();
        pool.run(|| {
            let mut s = LpString::new();
            assert_eq!(s.as_str(), "");
            assert_eq!(s.len(), 0);

            s.try_push_str("")?; // Push empty string
            assert_eq!(s.as_str(), "");

            s.try_push_str("a")?;
            assert_eq!(s.as_str(), "a");
            assert_eq!(s.len(), 1);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_string_growth_pattern() {
        let pool = setup_pool();
        pool.run(|| {
            let mut s = LpString::new();

            // Push strings of increasing size
            s.try_push_str("a")?;
            assert_eq!(s.len(), 1);

            s.try_push_str("bb")?;
            assert_eq!(s.len(), 3);

            s.try_push_str("ccc")?;
            assert_eq!(s.len(), 6);

            s.try_push_str("dddd")?;
            assert_eq!(s.len(), 10);

            assert_eq!(s.as_str(), "abbcccdddd");

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_string_partial_eq_symmetric() {
        let pool = setup_pool();
        pool.run(|| {
            let s1 = LpString::try_from_str("test")?;
            let s2 = LpString::try_from_str("test")?;
            let str_ref = "test";

            // Test symmetry of PartialEq
            assert!(s1 == s2);
            assert!(s2 == s1);

            assert!(s1 == str_ref);
            assert!(str_ref == s1.as_str());

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }

    #[test]
    fn test_string_ordering_with_different_lengths() {
        let pool = setup_pool();
        pool.run(|| {
            let short = LpString::try_from_str("a")?;
            let long = LpString::try_from_str("aaa")?;

            assert!(short < long);
            assert!(long > short);

            Ok::<(), AllocError>(())
        })
        .unwrap();
    }
}
