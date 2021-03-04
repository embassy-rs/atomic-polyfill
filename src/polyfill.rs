use core::ops::Deref;
use core::ops::DerefMut;

pub use core::sync::atomic::Ordering;
macro_rules! atomic_int {
    ($int_type:ident,$atomic_type:ident) => {
        #[derive(Default)]
        #[repr(transparent)]
        pub struct $atomic_type {
            inner: core::sync::atomic::$atomic_type,
        }

        impl Deref for $atomic_type {
            type Target = core::sync::atomic::$atomic_type;
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
                    inner: core::sync::atomic::$atomic_type::new(v),
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
                        self.store(new, set_order);
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
    inner: core::sync::atomic::AtomicBool,
}

impl Deref for AtomicBool {
    type Target = core::sync::atomic::AtomicBool;
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
            inner: core::sync::atomic::AtomicBool::new(v),
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
                self.store(new, set_order);
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
    inner: core::sync::atomic::AtomicPtr<T>,
}

impl<T> Deref for AtomicPtr<T> {
    type Target = core::sync::atomic::AtomicPtr<T>;
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
            inner: core::sync::atomic::AtomicPtr::new(v),
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
                self.store(new, set_order);
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
    cortex_m::interrupt::free(|_| f())
}
