use super::{AnyStorage, Storage};
use crate::component::{Component, ComponentIndex};
use std::mem::MaybeUninit;

pub struct SingleStorage<T> {
    value: Option<T>,
}

impl<T> Default for SingleStorage<T> {
    fn default() -> Self {
        Self {
            value: None,
        }
    }
}

impl<T> AnyStorage for SingleStorage<T> {
    unsafe fn extend_memcpy(&mut self, ptr: *const u8, len: usize) {
        if self.value.is_some() || len > 1 {
            panic!("SingleStorage can only hold one value");
        }

        if len == 1 {
            let mut val = MaybeUninit::uninit();

            std::ptr::copy_nonoverlapping(ptr as *const T, val.as_mut_ptr(), 1);
            self.value = Some(val.assume_init());
        }
    }

    fn swap_remove(&mut self, component: ComponentIndex) {
        assert_eq!(component.0, 0);
        self.value = None;
    }
}

impl<'a, T: Component> Storage<'a, T> for SingleStorage<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, items: I) {
        let mut items = items.into_iter();

        if let Some(item) = items.next() {
            assert!(self.value.is_none(), "SingleStorage can only hold one value");
            assert!(items.next().is_none(), "SingleStorage can only hold one value");
            self.value = Some(item);
        }
    }

    fn remove(&mut self, component: ComponentIndex) -> Option<T> {
        assert_eq!(component.0, 0);
        self.value.take()
    }
}