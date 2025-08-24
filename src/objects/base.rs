use std::{
    marker::PhantomData,
    mem,
    ops::Deref,
    ptr::NonNull,
    sync::{ atomic::{ AtomicUsize, Ordering }, Arc },
};

use super::EmAny;

pub struct EmObject<T> {
    _marker: PhantomData<T>,
    ptr: NonNull<u8>,
    refs: Arc<AtomicUsize>,
}

impl<'a, T> EmObject<T> {
    #[inline]
    pub fn new(item: T) -> Self {
        Self::from_boxed(Box::new(item))
    }

    /// Create a new object marked as "any type."
    ///
    /// The pointer still points to the right data.
    pub fn new_any<K>(item: K) -> EmObject<EmAny> {
        let ptr = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(item)) as *mut u8) };
        EmObject { _marker: PhantomData, ptr, refs: Arc::new(AtomicUsize::new(0)) }
    }

    pub fn from_boxed(item: Box<T>) -> Self {
        let ptr = unsafe { NonNull::new_unchecked(Box::into_raw(item) as *mut u8) };
        Self { _marker: PhantomData, ptr, refs: Arc::new(AtomicUsize::new(0)) }
    }

    /// **The reference is not recorded.** Internal uses only.
    pub const unsafe fn as_ref(&self) -> &'a T {
        unsafe { mem::transmute(self.ptr.as_ptr()) }
    }

    /// **The reference is recorded.**
    pub unsafe fn em_as_ref(&self) -> ReferencedObject<'a, T> {
        self.refs.fetch_add(1, Ordering::SeqCst);
        ReferencedObject::new(self.refs.clone(), unsafe { self.as_ref() })
    }

    /// Converts into an any type.
    pub const fn into_any(self) -> EmObject<EmAny> {
        // safety: it literally doesn't matter
        unsafe {
            mem::transmute(self)
        }
    }
}

impl EmObject<EmAny> {
    /// Cast to an actual type, and not "any."
    pub const unsafe fn cast<K>(self) -> EmObject<K> {
        // safety: it literally doesn't matter
        unsafe {
            mem::transmute(self)
        }
    }
}

impl<T: Clone> EmObject<T> {
    pub fn em_clone(&self) -> EmObject<T> {
        let data = (unsafe { self.as_ref() }).clone();
        Self::new(data)
    }
}

impl<T> std::fmt::Display for EmObject<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object({:p}, refs: {})", self.ptr, self.refs.load(Ordering::SeqCst))
    }
}

impl<T> Drop for EmObject<T> {
    fn drop(&mut self) {
        drop(unsafe { Box::from_raw(self.ptr.as_ptr()) });

        if self.refs.load(Ordering::SeqCst) > 0 {
            panic!("dropped while still referenced");
        }
    }
}

#[derive(Debug)]
pub struct ReferencedObject<'a, T> {
    counter: Arc<AtomicUsize>,
    holder: &'a T,
}

impl<'a, T> ReferencedObject<'a, T> {
    pub(crate) const fn new(counter: Arc<AtomicUsize>, holder: &'a T) -> Self {
        Self { counter, holder }
    }
}

impl<'a, T> Deref for ReferencedObject<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.holder
    }
}

impl<'a, T> Drop for ReferencedObject<'a, T> {
    fn drop(&mut self) {
        self.counter.fetch_sub(1, Ordering::SeqCst);
    }
}

pub trait IntoEmObject where Self: Sized {
    fn into_object(self) -> EmObject<Self> {
        EmObject::new(self)
    }
}
