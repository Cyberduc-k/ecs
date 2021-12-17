use super::*;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct NullStorage<T> {
    len: usize,
    _marker: PhantomData<[T]>,
}

pub struct NullIter<'a, T> {
    len: usize,
    _marker: PhantomData<&'a [T]>,
}

pub struct NullIterMut<'a, T> {
    len: usize,
    _marker: PhantomData<&'a mut [T]>,
}

impl<T> Default for NullStorage<T> {
    fn default() -> Self {
        assert_eq!(std::mem::size_of::<T>(), 0);

        Self {
            len: 0,
            _marker: PhantomData,
        }
    }
}

impl<T> AnyStorage for NullStorage<T> {
    #[inline]
    unsafe fn extend_memcpy(&mut self, _: *const u8, len: usize) {
        self.len += len;
    }

    #[inline]
    fn swap_remove(&mut self, component: ComponentIndex) {
        assert!(component.0 < self.len as u32);
        self.len -= 1;
    }
}

impl<'a, T: Component> Storage<'a, T> for NullStorage<T> {
    type Iter = NullIter<'a, T>;
    type IterMut = NullIterMut<'a, T>;

    fn get(&self, component: ComponentIndex) -> Option<&'a T> {
        if component.0 < self.len as u32 {
            Some(unsafe { NonNull::dangling().as_ref() })
        } else {
            None
        }
    }

    fn get_mut(&mut self, component: ComponentIndex) -> Option<&'a mut T> {
        if component.0 < self.len as u32 {
            Some(unsafe { NonNull::dangling().as_mut() })
        } else {
            None
        }
    }

    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, items: I) {
        self.len += items.into_iter().count();
    }

    #[inline]
    fn remove(&mut self, component: ComponentIndex) -> Option<T> {
        if component.0 < self.len as u32 {
            let dangling = NonNull::dangling();
            Some(unsafe { std::ptr::read(dangling.as_ptr()) })
        } else {
            None
        }
    }

    fn iter(&self) -> Self::Iter {
        NullIter {
            len: self.len,
            _marker: PhantomData,
        }
    }

    fn iter_mut(&mut self) -> Self::IterMut {
        NullIterMut {
            len: self.len,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for NullIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.len -= 1;
            Some(unsafe { NonNull::dangling().as_ref() })
        } else {
            None
        }
    }
}

impl<'a, T> Iterator for NullIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.len -= 1;
            Some(unsafe { NonNull::dangling().as_mut() })
        } else {
            None
        }
    }
}
