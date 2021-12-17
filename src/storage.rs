mod null;
mod single;
mod vec;

pub use null::NullStorage;
pub use single::SingleStorage;
pub use vec::VecStorage;

use crate::archetype::ArchetypeIndex;
use crate::component::{Component, ComponentIndex};
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub trait AnyStorage {
    unsafe fn extend_memcpy(&mut self, ptr: *const u8, len: usize);
    fn swap_remove(&mut self, component: ComponentIndex);
}

pub trait Storage<'a, T: Component>: AnyStorage + Default {
    type Iter: Iterator<Item = &'a T>;
    type IterMut: Iterator<Item = &'a mut T>;

    fn get(&'a self, component: ComponentIndex) -> Option<&'a T>;
    fn get_mut(&'a mut self, component: ComponentIndex) -> Option<&'a mut T>;
    fn extend<I: IntoIterator<Item = T>>(&mut self, items: I);
    fn remove(&mut self, component: ComponentIndex) -> Option<T>;

    fn iter(&'a self) -> Self::Iter;
    fn iter_mut(&'a mut self) -> Self::IterMut;
}

pub struct ArchetypeStorage<T: Component> {
    index: Vec<usize>,
    data: Vec<T::Storage>,
}

pub trait AnyArchetypeStorage: Any {
    fn register_archetype(&mut self, archetype: ArchetypeIndex);
    unsafe fn extend_memcpy(&mut self, archetype: ArchetypeIndex, ptr: *const u8, len: usize);
    fn swap_remove(&mut self, archetype: ArchetypeIndex, component: ComponentIndex);
}

#[derive(Default)]
pub struct Components {
    storages: HashMap<TypeId, Box<dyn AnyArchetypeStorage>>,
}

impl<T: Component> Default for ArchetypeStorage<T> {
    fn default() -> Self {
        Self {
            index: Vec::new(),
            data: Vec::new(),
        }
    }
}

impl<T: Component> ArchetypeStorage<T> {
    pub fn any() -> Box<dyn AnyArchetypeStorage> {
        Box::new(Self::default())
    }

    pub fn get(&self, archetype: ArchetypeIndex) -> Option<&T::Storage> {
        self.index.get(archetype.0 as usize).and_then(|&index| {
            self.data.get(index)
        })
    }

    pub fn get_mut(&mut self, archetype: ArchetypeIndex) -> Option<&mut T::Storage> {
        self.index.get(archetype.0 as usize).copied().and_then(move |index| {
            self.data.get_mut(index)
        })
    }

    pub unsafe fn get_mut_unchecked(&self, archetype: ArchetypeIndex) -> Option<&mut T::Storage> {
        self.index.get(archetype.0 as usize).and_then(|&index| {
            std::mem::transmute(self.data.get(index))
        })
    }

    pub fn extend<I: IntoIterator<Item = T>>(&mut self, archetype: ArchetypeIndex, items: I) {
        if let Some(&index) = self.index.get(archetype.0 as usize) {
            self.data[index].extend(items);
        }
    }
}

impl<T: Component> AnyArchetypeStorage for ArchetypeStorage<T> {
    fn register_archetype(&mut self, archetype: ArchetypeIndex) {
        let index = archetype.0 as usize;

        if index >= self.index.len() {
            self.index.resize(index + 1, !0);
        }

        self.index[index] = self.data.len();
        self.data.push(T::Storage::default());
    }

    unsafe fn extend_memcpy(&mut self, archetype: ArchetypeIndex, ptr: *const u8, len: usize) {
        let index = self.index[archetype.0 as usize];
        self.data[index].extend_memcpy(ptr, len);
    }

    fn swap_remove(&mut self, archetype: ArchetypeIndex, component: ComponentIndex) {
        let index = self.index[archetype.0 as usize];
        self.data[index].remove(component).unwrap();
    }
}

impl Components {
    pub fn get_or_insert<F>(&mut self, ty: TypeId, ctor: F) -> &mut dyn AnyArchetypeStorage
    where
        F: FnOnce() -> Box<dyn AnyArchetypeStorage>,
    {
        &mut **self.storages.entry(ty).or_insert_with(ctor)
    }

    pub fn get<T: Component>(&self) -> Option<&ArchetypeStorage<T>> {
        self.storages
            .get(&TypeId::of::<T>())
            .and_then(|s| s.downcast_ref::<T>())
    }

    pub fn get_mut<T: Component>(&mut self) -> Option<&mut ArchetypeStorage<T>> {
        self.storages
            .get_mut(&TypeId::of::<T>())
            .and_then(|s| s.downcast_mut::<T>())
    }

    pub fn get_any(&self, ty: TypeId) -> Option<&dyn AnyArchetypeStorage> {
        self.storages.get(&ty).map(|s| &**s)
    }

    pub fn get_any_mut(&mut self, ty: TypeId) -> Option<&mut dyn AnyArchetypeStorage> {
        self.storages.get_mut(&ty).map(|s| &mut **s)
    }
}

impl dyn AnyArchetypeStorage {
    #[inline]
    pub fn is<T: Component>(&self) -> bool {
        self.type_id() == TypeId::of::<ArchetypeStorage<T>>()
    }

    pub fn downcast_ref<T: Component>(&self) -> Option<&ArchetypeStorage<T>> {
        if self.is::<T>() {
            Some(unsafe { &*(self as *const _ as *const ArchetypeStorage<T>) })
        } else {
            None
        }
    }

    pub fn downcast_mut<T: Component>(&mut self) -> Option<&mut ArchetypeStorage<T>> {
        if self.is::<T>() {
            Some(unsafe { &mut *(self as *mut _ as *mut ArchetypeStorage<T>) })
        } else {
            None
        }
    }
}
