#![no_std]

pub use core::sync::atomic::{compiler_fence, fence, Ordering};

macro_rules! atomic_int {
    ($int_type:ident,$atomic_type:ident, $cfg_native:ident, $cfg_cas:ident, $cfg_full:ident) => {
        #[cfg($cfg_native)]
        pub use core::sync::atomic::$atomic_type;

        #[cfg(not($cfg_native))]
        #[repr(transparent)]
        pub struct $atomic_type {
            #[cfg($cfg_full)]
            inner: core::cell::UnsafeCell<$int_type>,
            #[cfg(not($cfg_full))]
            inner: core::sync::atomic::$atomic_type,
        }

        #[cfg(not($cfg_native))]
        unsafe impl Send for $atomic_type {}
        #[cfg(not($cfg_native))]
        unsafe impl Sync for $atomic_type {}

        #[cfg(not($cfg_native))]
        impl Default for $atomic_type {
            #[inline]
            fn default() -> Self {
                Self::new(Default::default())
            }
        }

        #[cfg(not($cfg_native))]
        impl From<$int_type> for $atomic_type {
            #[inline]
            fn from(v: $int_type) -> Self {
                Self::new(v)
            }
        }

        #[cfg(not($cfg_native))]
        impl core::fmt::Debug for $atomic_type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Debug::fmt(&self.load(Ordering::SeqCst), f)
            }
        }

        #[cfg(not($cfg_native))]
        impl $atomic_type {
            pub const fn new(v: $int_type) -> Self {
                Self {
                    #[cfg($cfg_full)]
                    inner: core::cell::UnsafeCell::new(v),
                    #[cfg(not($cfg_full))]
                    inner: core::sync::atomic::$atomic_type::new(v),
                }
            }

            pub fn get_mut(&mut self) -> &mut $int_type {
                self.inner.get_mut()
            }

            pub fn load(&self, _order: Ordering) -> $int_type {
                #[cfg($cfg_full)]
                return critical_section(|| unsafe { *self.inner.get() });
                #[cfg(not($cfg_full))]
                return self.inner.load(_order);
            }

            pub fn store(&self, val: $int_type, _order: Ordering) {
                #[cfg($cfg_full)]
                return critical_section(|| unsafe { *self.inner.get() = val });
                #[cfg(not($cfg_full))]
                return self.inner.store(val, _order);
            }

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

atomic_int!(u8, AtomicU8, u8_native, u8_cas, u8_full);
atomic_int!(u16, AtomicU16, u16_native, u16_cas, u16_full);
atomic_int!(u32, AtomicU32, u32_native, u32_cas, u32_full);
atomic_int!(usize, AtomicUsize, usize_native, usize_cas, usize_full);
atomic_int!(i8, AtomicI8, i8_native, i8_cas, i8_full);
atomic_int!(i16, AtomicI16, i16_native, i16_cas, i16_full);
atomic_int!(i32, AtomicI32, i32_native, i32_cas, i32_full);
atomic_int!(isize, AtomicIsize, isize_native, isize_cas, isize_full);

#[cfg(bool_native)]
pub use core::sync::atomic::AtomicBool;

#[cfg(not(bool_native))]
#[repr(transparent)]
pub struct AtomicBool {
    #[cfg(bool_full)]
    inner: core::cell::UnsafeCell<bool>,
    #[cfg(not(bool_full))]
    inner: core::sync::atomic::AtomicBool,
}

#[cfg(not(bool_native))]
impl Default for AtomicBool {
    /// Creates an `AtomicBool` initialized to `false`.
    #[inline]
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(not(bool_native))]
unsafe impl Send for AtomicBool {}
#[cfg(not(bool_native))]
unsafe impl Sync for AtomicBool {}

#[cfg(not(bool_native))]
impl AtomicBool {
    pub const fn new(v: bool) -> AtomicBool {
        Self {
            #[cfg(bool_full)]
            inner: core::cell::UnsafeCell::new(v),
            #[cfg(not(bool_full))]
            inner: core::sync::atomic::AtomicBool::new(v),
        }
    }

    pub fn load(&self, _order: Ordering) -> bool {
        #[cfg(bool_full)]
        return critical_section(|| unsafe { *self.inner.get() });
        #[cfg(not(bool_full))]
        return self.inner.load(_order);
    }

    pub fn store(&self, val: bool, _order: Ordering) {
        #[cfg(bool_full)]
        return critical_section(|| unsafe { *self.inner.get() = val });
        #[cfg(not(bool_full))]
        return self.inner.store(val, _order);
    }

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

#[cfg(ptr_native)]
pub use core::sync::atomic::AtomicPtr;

#[cfg(not(ptr_native))]
#[repr(transparent)]
pub struct AtomicPtr<T> {
    #[cfg(ptr_full)]
    inner: core::cell::UnsafeCell<*mut T>,
    #[cfg(not(ptr_full))]
    inner: core::sync::atomic::AtomicPtr<T>,
}

#[cfg(not(ptr_native))]
impl<T> Default for AtomicPtr<T> {
    /// Creates a null `AtomicPtr<T>`.
    #[inline]
    fn default() -> Self {
        Self::new(core::ptr::null_mut())
    }
}

#[cfg(not(ptr_native))]
unsafe impl<T> Sync for AtomicPtr<T> {}
#[cfg(not(ptr_native))]
unsafe impl<T> Send for AtomicPtr<T> {}

#[cfg(not(ptr_native))]
impl<T> AtomicPtr<T> {
    pub const fn new(v: *mut T) -> AtomicPtr<T> {
        Self {
            #[cfg(ptr_full)]
            inner: core::cell::UnsafeCell::new(v),
            #[cfg(not(ptr_full))]
            inner: core::sync::atomic::AtomicPtr::new(v),
        }
    }

    pub fn get_mut(&mut self) -> &mut *mut T {
        self.inner.get_mut()
    }

    pub fn load(&self, _order: Ordering) -> *mut T {
        #[cfg(ptr_full)]
        return critical_section(|| unsafe { *self.inner.get() });
        #[cfg(not(ptr_full))]
        return self.inner.load(_order);
    }

    pub fn store(&self, val: *mut T, _order: Ordering) {
        #[cfg(ptr_full)]
        return critical_section(|| unsafe { *self.inner.get() = val });
        #[cfg(not(ptr_full))]
        return self.inner.store(val, _order);
    }

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

#[allow(unused)]
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

#[allow(unused)]
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

#[allow(unused)]
fn critical_section<R>(f: impl FnOnce() -> R) -> R {
    critical_section::with(move |_| f())
}
