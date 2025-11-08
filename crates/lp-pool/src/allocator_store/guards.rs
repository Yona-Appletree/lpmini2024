#[cfg(any(feature = "std", test))]
use core::cell::Cell;
#[cfg(any(feature = "std", test))]
use std::thread_local;

#[cfg(all(not(feature = "std"), not(test)))]
use core::sync::atomic::{AtomicU32, Ordering};

#[cfg(any(feature = "std", test))]
thread_local! {
    static GUARD_DEPTH: Cell<u32> = const { Cell::new(0) };
    static ALLOW_DEPTH: Cell<u32> = const { Cell::new(0) };
}

#[cfg(all(not(feature = "std"), not(test)))]
static GUARD_DEPTH: AtomicU32 = AtomicU32::new(0);
#[cfg(all(not(feature = "std"), not(test)))]
static ALLOW_DEPTH: AtomicU32 = AtomicU32::new(0);

#[cfg(any(feature = "std", test))]
fn guard_depth() -> u32 {
    GUARD_DEPTH.with(|depth| depth.get())
}
#[cfg(all(not(feature = "std"), not(test)))]
fn guard_depth() -> u32 {
    GUARD_DEPTH.load(Ordering::SeqCst)
}

#[cfg(any(feature = "std", test))]
fn allow_depth() -> u32 {
    ALLOW_DEPTH.with(|depth| depth.get())
}
#[cfg(all(not(feature = "std"), not(test)))]
fn allow_depth() -> u32 {
    ALLOW_DEPTH.load(Ordering::SeqCst)
}

pub(crate) fn global_alloc_guard_active() -> bool {
    guard_depth() > 0
}

pub(crate) fn global_alloc_allowance_active() -> bool {
    allow_depth() > 0
}

pub(crate) struct GlobalAllocGuardToken;

pub(crate) fn enter_global_alloc_guard() -> GlobalAllocGuardToken {
    #[cfg(any(feature = "std", test))]
    {
        GUARD_DEPTH.with(|depth| depth.set(depth.get().saturating_add(1)));
    }
    #[cfg(all(not(feature = "std"), not(test)))]
    {
        GUARD_DEPTH.fetch_add(1, Ordering::SeqCst);
    }
    GlobalAllocGuardToken
}

impl Drop for GlobalAllocGuardToken {
    fn drop(&mut self) {
        #[cfg(any(feature = "std", test))]
        {
            GUARD_DEPTH.with(|depth| {
                let current = depth.get();
                if current > 0 {
                    depth.set(current - 1);
                }
            });
        }
        #[cfg(all(not(feature = "std"), not(test)))]
        {
            decrement(&GUARD_DEPTH);
        }
    }
}

pub(crate) struct GlobalAllocAllowToken;

pub(crate) fn enter_global_alloc_allowance() -> GlobalAllocAllowToken {
    #[cfg(any(feature = "std", test))]
    {
        ALLOW_DEPTH.with(|depth| depth.set(depth.get().saturating_add(1)));
    }
    #[cfg(all(not(feature = "std"), not(test)))]
    {
        ALLOW_DEPTH.fetch_add(1, Ordering::SeqCst);
    }
    GlobalAllocAllowToken
}

impl Drop for GlobalAllocAllowToken {
    fn drop(&mut self) {
        #[cfg(any(feature = "std", test))]
        {
            ALLOW_DEPTH.with(|depth| {
                let current = depth.get();
                if current > 0 {
                    depth.set(current - 1);
                }
            });
        }
        #[cfg(all(not(feature = "std"), not(test)))]
        {
            decrement(&ALLOW_DEPTH);
        }
    }
}

#[cfg(all(not(feature = "std"), not(test)))]
fn decrement(counter: &AtomicU32) {
    loop {
        let current = counter.load(Ordering::SeqCst);
        if current == 0 {
            break;
        }
        if counter
            .compare_exchange(current, current - 1, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            break;
        }
    }
}

pub(crate) fn force_clear_global_alloc_guard() {
    #[cfg(any(feature = "std", test))]
    {
        GUARD_DEPTH.with(|depth| depth.set(0));
        ALLOW_DEPTH.with(|depth| depth.set(0));
    }
    #[cfg(all(not(feature = "std"), not(test)))]
    {
        GUARD_DEPTH.store(0, Ordering::SeqCst);
        ALLOW_DEPTH.store(0, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard_depth() {
        assert_eq!(guard_depth(), 0);
        let _token1 = enter_global_alloc_guard();
        assert_eq!(guard_depth(), 1);
        let _token2 = enter_global_alloc_guard();
        assert_eq!(guard_depth(), 2);
        drop(_token2);
        assert_eq!(guard_depth(), 1);
        drop(_token1);
        assert_eq!(guard_depth(), 0);
    }

    #[test]
    fn test_allow_depth() {
        assert_eq!(allow_depth(), 0);
        let _token1 = enter_global_alloc_allowance();
        assert_eq!(allow_depth(), 1);
        let _token2 = enter_global_alloc_allowance();
        assert_eq!(allow_depth(), 2);
        drop(_token2);
        assert_eq!(allow_depth(), 1);
        drop(_token1);
        assert_eq!(allow_depth(), 0);
    }

    #[test]
    fn test_guard_active() {
        assert!(!global_alloc_guard_active());
        let _token = enter_global_alloc_guard();
        assert!(global_alloc_guard_active());
    }

    #[test]
    fn test_allowance_active() {
        assert!(!global_alloc_allowance_active());
        let _token = enter_global_alloc_allowance();
        assert!(global_alloc_allowance_active());
    }
}
