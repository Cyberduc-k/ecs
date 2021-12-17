use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use std::{
    any::{type_name, Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

#[derive(Default)]
pub struct Resources {
    resources: HashMap<TypeId, AtomicRefCell<Box<dyn Resource>>>,
}

pub trait Resource: Any {}

pub trait ResourceSet<'resources> {
    type Result: 'resources;

    unsafe fn fetch_unchecked(resources: &'resources Resources) -> Self::Result;

    fn fetch(resources: &'resources Resources) -> Self::Result
    where
        Self: Readonly,
    {
        unsafe { Self::fetch_unchecked(resources) }
    }

    fn fetch_mut(resources: &'resources mut Resources) -> Self::Result {
        unsafe { Self::fetch_unchecked(resources) }
    }
}

pub trait Readonly {}

pub struct Read<T>(PhantomData<*const T>);
pub struct Write<T>(PhantomData<*mut T>);

pub struct TryRead<T>(PhantomData<Option<*const T>>);
pub struct TryWrite<T>(PhantomData<Option<*mut T>>);

impl<T> Readonly for Read<T> {
}
impl<T> Readonly for TryRead<T> {
}

impl<T: 'static> Resource for T {
}

impl<'resources> ResourceSet<'resources> for () {
    type Result = ();

    unsafe fn fetch_unchecked(_: &'resources Resources) -> Self::Result {
        ()
    }
}

impl<'resources, T: Resource> ResourceSet<'resources> for Read<T> {
    type Result = AtomicRef<'resources, T>;

    unsafe fn fetch_unchecked(resources: &'resources Resources) -> Self::Result {
        resources.get()
    }
}

impl<'resources, T: Resource> ResourceSet<'resources> for Write<T> {
    type Result = AtomicRefMut<'resources, T>;

    unsafe fn fetch_unchecked(resources: &'resources Resources) -> Self::Result {
        resources.get_mut()
    }
}

impl<'resources, T: Resource> ResourceSet<'resources> for TryRead<T> {
    type Result = Option<AtomicRef<'resources, T>>;

    unsafe fn fetch_unchecked(resources: &'resources Resources) -> Self::Result {
        resources.try_get()
    }
}

impl<'resources, T: Resource> ResourceSet<'resources> for TryWrite<T> {
    type Result = Option<AtomicRefMut<'resources, T>>;

    unsafe fn fetch_unchecked(resources: &'resources Resources) -> Self::Result {
        resources.try_get_mut()
    }
}

impl<'resources> ResourceSet<'resources> for Resources {
    type Result = &'resources Resources;

    unsafe fn fetch_unchecked(resources: &'resources Resources) -> Self::Result {
        resources
    }
}

unsafe impl Send for Resources {
}

unsafe impl Sync for Resources {
}

impl Resources {
    pub fn contains<T: Resource>(&self) -> bool {
        self.resources.contains_key(&TypeId::of::<T>())
    }

    pub fn insert<T: Resource>(&mut self, resource: T) {
        self.resources
            .insert(TypeId::of::<T>(), AtomicRefCell::new(Box::new(resource)));
    }

    pub fn remove<T: Resource>(&mut self) -> Option<T> {
        self.resources
            .remove(&TypeId::of::<T>())
            .map(AtomicRefCell::into_inner)
            .and_then(<dyn Resource>::downcast)
            .map(|v| *v)
    }

    pub fn get<T: Resource>(&self) -> AtomicRef<T> {
        self.try_get()
            .expect(&format!("Resource `{}` not available", type_name::<T>()))
    }

    pub fn get_mut<T: Resource>(&self) -> AtomicRefMut<T> {
        self.try_get_mut()
            .expect(&format!("Resource `{}` not available", type_name::<T>()))
    }

    pub fn try_get<T: Resource>(&self) -> Option<AtomicRef<T>> {
        self.resources.get(&TypeId::of::<T>()).map(|v| {
            let borrow = v.borrow();
            AtomicRef::map(borrow, |v| v.downcast_ref().unwrap())
        })
    }

    pub fn try_get_mut<T: Resource>(&self) -> Option<AtomicRefMut<T>> {
        self.resources.get(&TypeId::of::<T>()).map(|v| {
            let borrow = v.borrow_mut();
            AtomicRefMut::map(borrow, |v| v.downcast_mut().unwrap())
        })
    }

    pub fn get_or_insert_with<T: Resource, F: FnOnce() -> T>(&mut self, insert: F) -> AtomicRef<T> {
        let borrow = self
            .resources
            .entry(TypeId::of::<T>())
            .or_insert_with(move || AtomicRefCell::new(Box::new(insert())))
            .borrow();

        AtomicRef::map(borrow, |v| v.downcast_ref().unwrap())
    }

    pub fn get_mut_or_insert_with<T: Resource, F: FnOnce() -> T>(&mut self, insert: F) -> AtomicRefMut<T> {
        let borrow = self
            .resources
            .entry(TypeId::of::<T>())
            .or_insert_with(move || AtomicRefCell::new(Box::new(insert())))
            .borrow_mut();

        AtomicRefMut::map(borrow, |v| v.downcast_mut().unwrap())
    }

    pub fn get_or_insert<T: Resource>(&mut self, resource: T) -> AtomicRef<T> {
        self.get_or_insert_with(|| resource)
    }

    pub fn get_mut_or_insert<T: Resource>(&mut self, resource: T) -> AtomicRefMut<T> {
        self.get_mut_or_insert_with(|| resource)
    }

    pub fn get_or_default<T: Resource + Default>(&mut self) -> AtomicRef<T> {
        self.get_or_insert_with(T::default)
    }

    pub fn get_mut_or_default<T: Resource + Default>(&mut self) -> AtomicRefMut<T> {
        self.get_mut_or_insert_with(T::default)
    }
}

impl<'a> dyn Resource + 'a {
    pub fn is<T: Resource>(&self) -> bool {
        self.type_id() == TypeId::of::<T>()
    }

    pub fn downcast<T: Resource>(self: Box<dyn Resource>) -> Option<Box<T>> {
        if self.is::<T>() {
            unsafe {
                let raw = Box::into_raw(self);
                Some(Box::from_raw(raw as *mut T))
            }
        } else {
            None
        }
    }

    pub fn downcast_ref<T: Resource>(&self) -> Option<&T> {
        if self.is::<T>() {
            Some(unsafe { &*(self as *const dyn Resource as *const T) })
        } else {
            None
        }
    }

    pub fn downcast_mut<T: Resource>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            Some(unsafe { &mut *(self as *mut dyn Resource as *mut T) })
        } else {
            None
        }
    }
}

macro_rules! impl_resource_set {
    ($head:ident) => {
        impl_resource_set!(@impl $head);
    };

    ($head:ident, $($tail:ident),+) => {
        impl_resource_set!($($tail),+);
        impl_resource_set!(@impl $head, $($tail),+);
    };

    (@impl $($ty:ident),+) => {
        impl<'resources, $($ty: ResourceSet<'resources>),+> ResourceSet<'resources> for ($($ty,)+) {
            type Result = ($($ty::Result,)+);

            unsafe fn fetch_unchecked(resources: &'resources Resources) -> Self::Result {
                ($($ty::fetch_unchecked(resources),)+)
            }
        }
    };
}

impl_resource_set!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
