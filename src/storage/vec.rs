use super::{AnyStorage, Storage};
use crate::component::{Component, ComponentIndex};

pub struct VecStorage<T> {
    vec: Vec<T>,
}

impl<T> Default for VecStorage<T> {
    fn default() -> Self {
        Self {
            vec: Vec::default(),
        }
    }
}

impl<T> AnyStorage for VecStorage<T> {
    unsafe fn extend_memcpy(&mut self, ptr: *const u8, len: usize) {
        self.vec.reserve(len);

        let dst = self.vec.as_mut_ptr().add(self.vec.len());
        let new_len = self.vec.len() + len;

        std::ptr::copy_nonoverlapping(ptr as *const T, dst, len);
        self.vec.set_len(new_len);
    }

    fn swap_remove(&mut self, component: ComponentIndex) {
        self.vec.swap_remove(component.0 as usize);
    }
}

impl<'a, T: Component> Storage<'a, T> for VecStorage<T> {
    type Iter = std::slice::Iter<'a, T>;
    type IterMut = std::slice::IterMut<'a, T>;

    fn get(&'a self, component: ComponentIndex) -> Option<&'a T> {
        self.vec.get(component.0 as usize)
    }

    fn get_mut(&'a mut self, component: ComponentIndex) -> Option<&'a mut T> {
        self.vec.get_mut(component.0 as usize)
    }

    fn extend<I: IntoIterator<Item = T>>(&mut self, items: I) {
        self.vec.extend(items);
    }

    fn remove(&mut self, component: ComponentIndex) -> Option<T> {
        let index = component.0 as usize;

        if index < self.vec.len() {
            Some(self.vec.swap_remove(component.0 as usize))
        } else {
            None
        }
    }

    fn iter(&'a self) -> Self::Iter {
        self.vec.iter()
    }

    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.vec.iter_mut()
    }
}
