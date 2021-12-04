use crate::component::Component;
use crate::storage::{ArchetypeStorage, AnyArchetypeStorage, Components};
use std::any::TypeId;
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

pub struct EditComponents<'a> {
    components: &'a mut Components,
    borrowed: HashSet<TypeId>,
}

pub struct EditComponent<'a, T: Component> {
    borrowed: *mut HashSet<TypeId>,
    storage: &'a mut ArchetypeStorage<T>,
}

pub struct EditAnyComponent<'a> {
    borrowed: *mut HashSet<TypeId>,
    storage: &'a mut dyn AnyArchetypeStorage,
    ty: TypeId,
}

impl Components {
    pub fn edit(&mut self) -> EditComponents {
        EditComponents {
            components: self,
            borrowed: HashSet::new(),
        }
    }
}

impl<'a> EditComponents<'a> {
    pub fn get<T: Component>(&mut self) -> Option<EditComponent<'a, T>> {
        assert!(self.borrowed.insert(TypeId::of::<T>()));

        Some(EditComponent {
            borrowed: &mut self.borrowed,
            storage: Self::extend_lifetime(self.components.get_mut::<T>()?),
        })
    }

    pub fn get_any(&mut self, ty: TypeId) -> Option<EditAnyComponent<'a>> {
        assert!(self.borrowed.insert(ty));

        Some(EditAnyComponent {
            borrowed: &mut self.borrowed,
            storage: Self::extend_lifetime(self.components.get_any_mut(ty)?),
            ty,
        })
    }

    fn extend_lifetime<T: ?Sized>(a: &mut T) -> &'a mut T {
        unsafe { std::mem::transmute(a) }
    }
}

impl<'a, T: Component> Deref for EditComponent<'a, T> {
    type Target = ArchetypeStorage<T>;

    fn deref(&self) -> &Self::Target {
        self.storage
    }
}

impl<'a, T: Component> DerefMut for EditComponent<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.storage
    }
}

impl<'a> Deref for EditAnyComponent<'a> {
    type Target = dyn AnyArchetypeStorage;

    fn deref(&self) -> &Self::Target {
        self.storage
    }
}

impl<'a> DerefMut for EditAnyComponent<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.storage
    }
}

impl<'a, T: Component> Drop for EditComponent<'a, T> {
    fn drop(&mut self) {
        unsafe {
            let borrowed = &mut *self.borrowed;
            borrowed.remove(&TypeId::of::<T>());
        }
    }
}

impl<'a> Drop for EditAnyComponent<'a> {
    fn drop(&mut self) {
        unsafe {
            let borrowed = &mut *self.borrowed;
            borrowed.remove(&self.ty);
        }
    }
}