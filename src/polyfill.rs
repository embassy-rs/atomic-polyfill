use core::ops::Deref;
use core::ops::DerefMut;

pub use core::sync::atomic::{compiler_fence, fence, Ordering};

#[cfg(not(polyfill_types))]
use core::sync::atomic;

#[cfg(polyfill_types)]
mod atomic {
    use crate::polyfill::critical_section;
    use core::cell::UnsafeCell;
    use core::fmt;
    use core::sync::atomic::Ordering;

    #[repr(C, align(1))]
    pub struct AtomicBool {
        v: UnsafeCell<u8>,
    }

    impl Default for AtomicBool {
        /// Creates an `AtomicBool` initialized to `false`.
        #[inline]
        fn default() -> Self {
            Self::new(false)
        }
    }

    unsafe impl Sync for AtomicBool {}

    impl AtomicBool {
        pub const fn new(v: bool) -> AtomicBool {
            AtomicBool {
                v: UnsafeCell::new(v as u8),
            }
        }

        pub fn get_mut(&mut self) -> &mut bool {
            // SAFETY: the mutable reference guarantees unique ownership.
            unsafe { &mut *(self.v.get() as *mut bool) }
        }

        pub fn load(&self, _order: Ordering) -> bool {
            critical_section(|| unsafe { *self.v.get() != 0 })
        }

        pub fn store(&self, val: bool, _order: Ordering) {
            critical_section(|| unsafe { *self.v.get() = val as u8 })
        }
    }

    #[cfg_attr(target_pointer_width = "16", repr(C, align(2)))]
    #[cfg_attr(target_pointer_width = "32", repr(C, align(4)))]
    #[cfg_attr(target_pointer_width = "64", repr(C, align(8)))]
    pub struct AtomicPtr<T> {
        p: UnsafeCell<*mut T>,
    }

    impl<T> Default for AtomicPtr<T> {
        /// Creates a null `AtomicPtr<T>`.
        fn default() -> AtomicPtr<T> {
            AtomicPtr::new(core::ptr::null_mut())
        }
    }

    unsafe impl<T> Sync for AtomicPtr<T> {}
    unsafe impl<T> Send for AtomicPtr<T> {}

    impl<T> AtomicPtr<T> {
        pub const fn new(p: *mut T) -> AtomicPtr<T> {
            AtomicPtr {
                p: UnsafeCell::new(p),
            }
        }

        pub fn get_mut(&mut self) -> &mut *mut T {
            self.p.get_mut()
        }

        pub fn load(&self, _order: Ordering) -> *mut T {
            critical_section(|| unsafe { *self.p.get() })
        }

        pub fn store(&self, ptr: *mut T, _order: Ordering) {
            critical_section(|| unsafe { *self.p.get() = ptr })
        }
    }

    macro_rules! atomic_int {
        ($int_type:ident,$atomic_type:ident,$align:expr) => {
            #[repr(C, align($align))]
            pub struct $atomic_type {
                v: UnsafeCell<$int_type>,
            }

            unsafe impl Sync for $atomic_type {}

            impl Default for $atomic_type {
                #[inline]
                fn default() -> Self {
                    Self::new(Default::default())
                }
            }

            impl From<$int_type> for $atomic_type {
                #[inline]
                fn from(v: $int_type) -> Self {
                    Self::new(v)
                }
            }

            impl fmt::Debug for $atomic_type {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    fmt::Debug::fmt(&self.load(Ordering::SeqCst), f)
                }
            }

            impl $atomic_type {
                pub const fn new(v: $int_type) -> Self {
                    Self {
                        v: UnsafeCell::new(v),
                    }
                }

                pub fn get_mut(&mut self) -> &mut $int_type {
                    self.v.get_mut()
                }

                pub fn load(&self, _order: Ordering) -> $int_type {
                    critical_section(|| unsafe { *self.v.get() })
                }

                pub fn store(&self, val: $int_type, _order: Ordering) {
                    critical_section(|| unsafe { *self.v.get() = val })
                }
            }
        };
    }

    atomic_int!(u8, AtomicU8, 1);
    atomic_int!(u16, AtomicU16, 2);
    atomic_int!(u32, AtomicU32, 4);
    atomic_int!(i8, AtomicI8, 1);
    atomic_int!(i16, AtomicI16, 2);
    atomic_int!(i32, AtomicI32, 4);
    #[cfg(target_pointer_width = "32")]
    atomic_int!(usize, AtomicUsize, 4);
    #[cfg(target_pointer_width = "32")]
    atomic_int!(isize, AtomicIsize, 4);
    #[cfg(target_pointer_width = "64")]
    atomic_int!(usize, AtomicUsize, 8);
    #[cfg(target_pointer_width = "64")]
    atomic_int!(isize, AtomicIsize, 8);
}

macro_rules! atomic_int {
    ($int_type:ident,$atomic_type:ident) => {
        #[derive(Default)]
        #[repr(transparent)]
        pub struct $atomic_type {
            inner: atomic::$atomic_type,
        }

        impl Deref for $atomic_type {
            type Target = atomic::$atomic_type;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl DerefMut for $atomic_type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl $atomic_type {
            pub const fn new(v: $int_type) -> $atomic_type {
                Self {
                    inner: atomic::$atomic_type::new(v),
                }
            }
        }

        impl $atomic_type {
            pub fn swap(&self, val: $int_type, order: Ordering) -> $int_type {
                self.op(order, |_| val)
            }

            pub fn compare_exchange(
                &self,
                current: $int_type,
                new: $int_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$int_type, $int_type> {
                self.compare_exchange_weak(current, new, success, failure)
            }

            pub fn compare_exchange_weak(
                &self,
                current: $int_type,
                new: $int_type,
                success: Ordering,
                _failure: Ordering,
            ) -> Result<$int_type, $int_type> {
                critical_section(|| {
                    let old = self.load(load_ordering(success));
                    if old == current {
                        self.store(new, store_ordering(success));
                        Ok(old)
                    } else {
                        Err(old)
                    }
                })
            }

            pub fn fetch_add(&self, val: $int_type, order: Ordering) -> $int_type {
                self.op(order, |old| old.wrapping_add(val))
            }

            pub fn fetch_sub(&self, val: $int_type, order: Ordering) -> $int_type {
                self.op(order, |old| old.wrapping_sub(val))
            }

            pub fn fetch_and(&self, val: $int_type, order: Ordering) -> $int_type {
                self.op(order, |old| old & val)
            }

            pub fn fetch_nand(&self, val: $int_type, order: Ordering) -> $int_type {
                self.op(order, |old| !(old & val))
            }

            pub fn fetch_or(&self, val: $int_type, order: Ordering) -> $int_type {
                self.op(order, |old| old | val)
            }

            pub fn fetch_xor(&self, val: $int_type, order: Ordering) -> $int_type {
                self.op(order, |old| old ^ val)
            }

            pub fn fetch_update<F>(
                &self,
                set_order: Ordering,
                _fetch_order: Ordering,
                mut f: F,
            ) -> Result<$int_type, $int_type>
            where
                F: FnMut($int_type) -> Option<$int_type>,
            {
                critical_section(|| {
                    let old = self.load(load_ordering(set_order));
                    if let Some(new) = f(old) {
                        self.store(new, store_ordering(set_order));
                        Ok(old)
                    } else {
                        Err(old)
                    }
                })
            }

            pub fn fetch_max(&self, val: $int_type, order: Ordering) -> $int_type {
                self.op(order, |old| old.max(val))
            }

            pub fn fetch_min(&self, val: $int_type, order: Ordering) -> $int_type {
                self.op(order, |old| old.min(val))
            }

            fn op(&self, order: Ordering, f: impl FnOnce($int_type) -> $int_type) -> $int_type {
                critical_section(|| {
                    let old = self.load(load_ordering(order));
                    let new = f(old);
                    self.store(new, store_ordering(order));
                    old
                })
            }
        }
    };
}

atomic_int!(u8, AtomicU8);
atomic_int!(u16, AtomicU16);
atomic_int!(u32, AtomicU32);
atomic_int!(usize, AtomicUsize);
atomic_int!(i8, AtomicI8);
atomic_int!(i16, AtomicI16);
atomic_int!(i32, AtomicI32);
atomic_int!(isize, AtomicIsize);

#[derive(Default)]
#[repr(transparent)]
pub struct AtomicBool {
    inner: atomic::AtomicBool,
}

impl Deref for AtomicBool {
    type Target = atomic::AtomicBool;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for AtomicBool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AtomicBool {
    pub const fn new(v: bool) -> AtomicBool {
        Self {
            inner: atomic::AtomicBool::new(v),
        }
    }
}

impl AtomicBool {
    pub fn swap(&self, val: bool, order: Ordering) -> bool {
        self.op(order, |_| val)
    }

    pub fn compare_exchange(
        &self,
        current: bool,
        new: bool,
        success: Ordering,
        failure: Ordering,
    ) -> Result<bool, bool> {
        self.compare_exchange_weak(current, new, success, failure)
    }

    pub fn compare_exchange_weak(
        &self,
        current: bool,
        new: bool,
        success: Ordering,
        _failure: Ordering,
    ) -> Result<bool, bool> {
        critical_section(|| {
            let old = self.load(load_ordering(success));
            if old == current {
                self.store(new, store_ordering(success));
                Ok(old)
            } else {
                Err(old)
            }
        })
    }

    pub fn fetch_and(&self, val: bool, order: Ordering) -> bool {
        self.op(order, |old| old & val)
    }

    pub fn fetch_nand(&self, val: bool, order: Ordering) -> bool {
        self.op(order, |old| !(old & val))
    }

    pub fn fetch_or(&self, val: bool, order: Ordering) -> bool {
        self.op(order, |old| old | val)
    }

    pub fn fetch_xor(&self, val: bool, order: Ordering) -> bool {
        self.op(order, |old| old ^ val)
    }

    pub fn fetch_update<F>(
        &self,
        set_order: Ordering,
        _fetch_order: Ordering,
        mut f: F,
    ) -> Result<bool, bool>
    where
        F: FnMut(bool) -> Option<bool>,
    {
        critical_section(|| {
            let old = self.load(load_ordering(set_order));
            if let Some(new) = f(old) {
                self.store(new, store_ordering(set_order));
                Ok(old)
            } else {
                Err(old)
            }
        })
    }

    pub fn fetch_max(&self, val: bool, order: Ordering) -> bool {
        self.op(order, |old| old.max(val))
    }

    pub fn fetch_min(&self, val: bool, order: Ordering) -> bool {
        self.op(order, |old| old.min(val))
    }

    fn op(&self, order: Ordering, f: impl FnOnce(bool) -> bool) -> bool {
        critical_section(|| {
            let old = self.load(load_ordering(order));
            let new = f(old);
            self.store(new, store_ordering(order));
            old
        })
    }
}

#[derive(Default)]
#[repr(transparent)]
pub struct AtomicPtr<T> {
    inner: atomic::AtomicPtr<T>,
}

impl<T> Deref for AtomicPtr<T> {
    type Target = atomic::AtomicPtr<T>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for AtomicPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> AtomicPtr<T> {
    pub const fn new(v: *mut T) -> AtomicPtr<T> {
        Self {
            inner: atomic::AtomicPtr::new(v),
        }
    }
}

impl<T> AtomicPtr<T> {
    pub fn swap(&self, val: *mut T, order: Ordering) -> *mut T {
        self.op(order, |_| val)
    }

    pub fn compare_exchange(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> {
        self.compare_exchange_weak(current, new, success, failure)
    }

    pub fn compare_exchange_weak(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        _failure: Ordering,
    ) -> Result<*mut T, *mut T> {
        critical_section(|| {
            let old = self.load(load_ordering(success));
            if old == current {
                self.store(new, store_ordering(success));
                Ok(old)
            } else {
                Err(old)
            }
        })
    }

    pub fn fetch_update<F>(
        &self,
        set_order: Ordering,
        _fetch_order: Ordering,
        mut f: F,
    ) -> Result<*mut T, *mut T>
    where
        F: FnMut(*mut T) -> Option<*mut T>,
    {
        critical_section(|| {
            let old = self.load(load_ordering(set_order));
            if let Some(new) = f(old) {
                self.store(new, store_ordering(set_order));
                Ok(old)
            } else {
                Err(old)
            }
        })
    }

    fn op(&self, order: Ordering, f: impl FnOnce(*mut T) -> *mut T) -> *mut T {
        critical_section(|| {
            let old = self.load(load_ordering(order));
            let new = f(old);
            self.store(new, store_ordering(order));
            old
        })
    }
}

fn load_ordering(order: Ordering) -> Ordering {
    match order {
        Ordering::Release => Ordering::Relaxed,
        Ordering::Relaxed => Ordering::Relaxed,
        Ordering::SeqCst => Ordering::SeqCst,
        Ordering::Acquire => Ordering::Acquire,
        Ordering::AcqRel => Ordering::Acquire,
        x => x,
    }
}

fn store_ordering(order: Ordering) -> Ordering {
    match order {
        Ordering::Release => Ordering::Release,
        Ordering::Relaxed => Ordering::Relaxed,
        Ordering::SeqCst => Ordering::SeqCst,
        Ordering::Acquire => Ordering::Relaxed,
        Ordering::AcqRel => Ordering::Release,
        x => x,
    }
}

fn critical_section<R>(f: impl FnOnce() -> R) -> R {
    #[cfg(target_arch = "arm")]
    use cortex_m as arch;
    #[cfg(target_arch = "riscv32")]
    use riscv as arch;

    arch::interrupt::free(|_| f())
}
