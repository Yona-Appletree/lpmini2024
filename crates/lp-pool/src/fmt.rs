use crate::collections::string::LpString;
use crate::error::AllocError;
use core::fmt::{self, Write};

/// Write formatted data into an existing `LpString`.
pub fn write_lp_string(target: &mut LpString, args: fmt::Arguments<'_>) -> Result<(), AllocError> {
    target
        .write_fmt(args)
        .map_err(|_| AllocError::PoolExhausted)
}

/// Format data into a new `LpString`.
pub fn lp_format(args: fmt::Arguments<'_>) -> Result<LpString, AllocError> {
    let mut string = LpString::new();
    write_lp_string(&mut string, args)?;
    Ok(string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_pool::LpMemoryPool;
    use core::ptr::NonNull;

    fn setup_pool() -> LpMemoryPool {
        let mut memory = [0u8; 8192];
        let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();
        unsafe { LpMemoryPool::new(memory_ptr, memory.len()).unwrap() }
    }

    #[test]
    fn test_lp_format() {
        let pool = setup_pool();
        pool.run(|| {
            let formatted =
                lp_format(format_args!("{} {}", "hello", 42)).expect("formatting should succeed");
            assert_eq!(formatted.as_str(), "hello 42");

            let mut buf = LpString::new();
            write_lp_string(&mut buf, format_args!("{}!", formatted.as_str()))
                .expect("write should succeed");
            assert_eq!(buf.as_str(), "hello 42!");
            Ok::<(), AllocError>(())
        })
        .unwrap();
    }
}
