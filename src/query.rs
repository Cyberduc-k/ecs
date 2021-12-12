mod entity;
mod multiple;
mod read;
mod write;

pub use crate::resource::{Read, Readonly, Write};
pub use multiple::Multiple;

use crate::{
    archetype::{Archetype, ArchetypeIndex},
    component::Component,
    entity::Entity,
    storage::{ArchetypeStorage, Components, Storage},
    subworld::AnyWorld,
    world::StorageAccess,
};

use std::{any::TypeId, lazy::SyncOnceCell, marker::PhantomData};

pub trait IntoQuery: Sized {
    type Fetch: for<'world> Fetch<'world>;

    fn query() -> Query<Self::Fetch> {
        Query::default()
    }
}

pub struct Query<T: for<'world> Fetch<'world>> {
    archetypes: SyncOnceCell<Vec<ArchetypeIndex>>,
    _marker: PhantomData<T>,
}

pub struct QueryIter<'world, 'index, F: Fetch<'world>> {
    iter: F::Iter,
    _marker: PhantomData<&'index [ArchetypeIndex]>,
}

pub trait Fetch<'world>: ComponentTypes {
    type Item: 'world;
    type Iter: Iterator<Item = Self::Item> + 'world;

    fn fetch(
        components: &'world Components,
        archetypes: &'world [Archetype],
        index: &'world [ArchetypeIndex],
    ) -> Self::Iter;
}

pub trait ComponentTypes {
    fn components() -> Vec<TypeId>;
}

impl<T: for<'world> Fetch<'world>> Default for Query<T> {
    fn default() -> Self {
        Self {
            archetypes: SyncOnceCell::new(),
            _marker: PhantomData,
        }
    }
}

impl<T: for<'world> Fetch<'world>> Clone for Query<T> {
    fn clone(&self) -> Self {
        Self {
            archetypes: self.archetypes.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T: for<'world> Fetch<'world>> Query<T> {
    pub fn get<'world, W: AnyWorld>(&self, world: &'world W, entity: Entity) -> Option<<T as Fetch<'world>>::Item>
    where
        T: Readonly,
    {
        let access = world.storage_access();
        let data = access.entities().get(entity)?;
        let index: &[ArchetypeIndex] = &[data.archetype()];
        let index = unsafe { std::mem::transmute(index) };
        let mut iter = T::fetch(access.components(), access.archetypes(), index);

        iter.nth(data.component().0 as usize)
    }

    pub fn get_mut<'world, W: AnyWorld>(
        &self,
        world: &'world mut W,
        entity: Entity,
    ) -> Option<<T as Fetch<'world>>::Item> {
        let access = world.storage_access();
        let data = access.entities().get(entity)?;
        let index: &[ArchetypeIndex] = &[data.archetype()];
        let index = unsafe { std::mem::transmute(index) };
        let mut iter = T::fetch(access.components(), access.archetypes(), index);

        iter.nth(data.component().0 as usize)
    }

    pub fn iter<'world, 'index, W: AnyWorld>(&'index self, world: &'world W) -> QueryIter<'world, 'index, T>
    where
        T: Readonly,
    {
        let access = world.storage_access();
        let index = self.find_archetypes(&access);
        let index = unsafe { std::mem::transmute::<_, &'world [ArchetypeIndex]>(index) };

        QueryIter {
            iter: T::fetch(access.components(), access.archetypes(), index),
            _marker: PhantomData,
        }
    }

    pub fn iter_mut<'world, 'index, W: AnyWorld>(&'index self, world: &'world mut W) -> QueryIter<'world, 'index, T> {
        let access = world.storage_access();
        let index = self.find_archetypes(&access);
        let index = unsafe { std::mem::transmute::<_, &'world [ArchetypeIndex]>(index) };

        QueryIter {
            iter: T::fetch(access.components(), access.archetypes(), index),
            _marker: PhantomData,
        }
    }

    fn find_archetypes<'world, 'index>(&'index self, access: &StorageAccess<'world>) -> &'index [ArchetypeIndex] {
        self.archetypes.get_or_init(move || {
            let components = T::components();

            access
                .archetypes()
                .iter()
                .filter_map(|a| {
                    if a.layout.contains(&components) {
                        Some(a.index)
                    } else {
                        None
                    }
                })
                .collect()
        })
    }
}

impl<'world, 'index, T: Fetch<'world>> Iterator for QueryIter<'world, 'index, T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

macro_rules! impl_tuple_query {
    ($head:ident) => {
        impl_tuple_query!(@impl $head);
    };

    ($head:ident, $($tail:ident),+) =>{
        impl_tuple_query!($($tail),+);
        impl_tuple_query!(@impl $head, $($tail),+);
    };

    (@impl $($ty:ident),*) => {
        impl<$($ty: IntoQuery),*> IntoQuery for ($($ty,)*) {
            type Fetch = multiple::Multiple<($($ty::Fetch,)*)>;

            fn query() -> Query<Self::Fetch> {
                Query::default()
            }
        }
    };
}

impl_tuple_query!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
