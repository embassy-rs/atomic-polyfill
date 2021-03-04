
use core::ops::Deref;
use core::ops::DerefMut;

pub use core::sync::atomic::Ordering;

#[derive(Default)]
#[repr(transparent)]
pub struct AtomicU32 {
    inner: core::sync::atomic::AtomicU32,
}

impl Deref for AtomicU32 {
    type Target = core::sync::atomic::AtomicU32;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for AtomicU32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AtomicU32 {
    pub const fn new(v: u32) -> AtomicU32 {
        Self {
            inner: core::sync::atomic::AtomicU32::new(v),
        }
    }
}

impl AtomicU32 {
    pub fn swap(&self, val: u32, order: Ordering) -> u32 {
        self.op(order, |_| val)
    }

    pub fn compare_exchange(
        &self,
        current: u32,
        new: u32,
        success: Ordering,
        failure: Ordering,
    ) -> Result<u32, u32> {
        self.compare_exchange_weak(current, new, success, failure)
    }

    pub fn compare_exchange_weak(
        &self,
        current: u32,
        new: u32,
        success: Ordering,
        _failure: Ordering,
    ) -> Result<u32, u32> {
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

    pub fn fetch_add(&self, val: u32, order: Ordering) -> u32 {
        self.op(order, |old| old.wrapping_add(val))
    }

    pub fn fetch_sub(&self, val: u32, order: Ordering) -> u32 {
        self.op(order, |old| old.wrapping_sub(val))
    }

    pub fn fetch_and(&self, val: u32, order: Ordering) -> u32 {
        self.op(order, |old| old & val)
    }

    pub fn fetch_nand(&self, val: u32, order: Ordering) -> u32 {
        self.op(order, |old| !(old & val))
    }

    pub fn fetch_or(&self, val: u32, order: Ordering) -> u32 {
        self.op(order, |old| old | val)
    }

    pub fn fetch_xor(&self, val: u32, order: Ordering) -> u32 {
        self.op(order, |old| old ^ val)
    }

    pub fn fetch_update<F>(
        &self,
        set_order: Ordering,
        _fetch_order: Ordering,
        mut f: F,
    ) -> Result<u32, u32>
    where
        F: FnMut(u32) -> Option<u32>,
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

    pub fn fetch_max(&self, val: u32, order: Ordering) -> u32 {
        self.op(order, |old| old.max(val))
    }

    pub fn fetch_min(&self, val: u32, order: Ordering) -> u32 {
        self.op(order, |old| old.min(val))
    }

    fn op(&self, order: Ordering, f: impl FnOnce(u32) -> u32) -> u32 {
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
