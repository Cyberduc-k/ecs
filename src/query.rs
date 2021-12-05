use crate::{
    archetype::ArchetypeIndex,
    component::Component,
    storage::{ArchetypeStorage, Components, Storage},
};
use std::marker::PhantomData;

pub trait Fetch<'a> {
    type Item: 'a;
    type Iter: Iterator<Item = Self::Item>;

    fn fetch(components: &'a Components, archetypes: &'a [ArchetypeIndex]) -> Self::Iter;
}

pub struct Read<T>(PhantomData<*const T>);
pub struct Write<T>(PhantomData<*mut T>);
pub struct Multiple<T>(T);

pub enum ReadIter<'a, T: Component> {
    Empty,
    Iter {
        storage: &'a ArchetypeStorage<T>,
        components: Option<<T::Storage as Storage<'a, T>>::Iter>,
        archetypes: std::slice::Iter<'a, ArchetypeIndex>,
    },
}

pub enum WriteIter<'a, T: Component> {
    Empty,
    Iter {
        storage: &'a ArchetypeStorage<T>,
        components: Option<<T::Storage as Storage<'a, T>>::IterMut>,
        archetypes: std::slice::Iter<'a, ArchetypeIndex>,
    },
}

pub struct MultiIter<T>(T);

impl<'a, T: Component> Fetch<'a> for Read<T> {
    type Item = &'a T;
    type Iter = ReadIter<'a, T>;

    fn fetch(components: &'a Components, archetypes: &'a [ArchetypeIndex]) -> Self::Iter {
        match components.get::<T>() {
            None => ReadIter::Empty,
            Some(storage) => ReadIter::Iter {
                storage,
                components: None,
                archetypes: archetypes.iter(),
            },
        }
    }
}

impl<'a, T: Component> Fetch<'a> for Write<T> {
    type Item = &'a mut T;
    type Iter = WriteIter<'a, T>;

    fn fetch(components: &'a Components, archetypes: &'a [ArchetypeIndex]) -> Self::Iter {
        match components.get::<T>() {
            None => WriteIter::Empty,
            Some(storage) => WriteIter::Iter {
                storage,
                components: None,
                archetypes: archetypes.iter(),
            },
        }
    }
}

impl<'a, T: Component> Iterator for ReadIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Empty => None,
            Self::Iter {
                storage,
                components,
                archetypes,
            } => match components {
                Some(comps) => match comps.next() {
                    Some(comp) => Some(comp),
                    None => {
                        *components = None;
                        self.next()
                    }
                },
                None => {
                    *components = storage.get(*archetypes.next()?).map(|s| s.iter());
                    self.next()
                }
            },
        }
    }
}

impl<'a, T: Component> Iterator for WriteIter<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Empty => None,
            Self::Iter {
                storage,
                components,
                archetypes,
            } => match components {
                Some(comps) => match comps.next() {
                    Some(comp) => Some(comp),
                    None => {
                        *components = None;
                        self.next()
                    }
                },
                None => unsafe {
                    *components = storage
                        .get_mut_unchecked(*archetypes.next()?)
                        .map(|s| s.iter_mut());
                    self.next()
                },
            },
        }
    }
}

macro_rules! impl_multi {
    ($head:ident) => {
        impl_multi!(@impl $head);
    };

    ($head:ident, $($tail:ident),+) => {
        impl_multi!($($tail),+);
        impl_multi!(@impl $head, $($tail),+);
    };

    (@impl $($ty:ident),*) => {
        impl<'a, $($ty: Fetch<'a>),+> Fetch<'a> for Multiple<($($ty,)+)> {
            type Item = ($($ty::Item,)+);
            type Iter = MultiIter<($($ty::Iter,)+)>;

            #[allow(non_snake_case)]
            fn fetch(components: &'a Components, archetypes: &'a [ArchetypeIndex]) -> Self::Iter {
                $(let $ty = $ty::fetch(components, archetypes);)*
                MultiIter(($($ty,)+))
            }
        }
    };
}

impl_multi!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

macro_rules! impl_multi_iter {
    ($head:ident) => {
        impl_multi_iter!(@impl $head);
    };

    ($head:ident, $($tail:ident),+) => {
        impl_multi_iter!($($tail),+);
        impl_multi_iter!(@impl $head, $($tail),+);
    };

    (@impl $($ty:ident),*) => {
        impl<$($ty: Iterator),+> Iterator for MultiIter<($($ty,)+)> {
            type Item = ($($ty::Item,)+);

            #[allow(non_snake_case)]
            fn next(&mut self) -> Option<Self::Item> {
                let Self(($($ty,)+)) = self;
                $(let $ty = $ty.next()?;)+
                Some(($($ty,)+))
            }
        }
    };
}

impl_multi_iter!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
