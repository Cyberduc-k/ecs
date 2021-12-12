mod entity;
mod multiple;
mod read;
mod write;

pub use multiple::Multiple;
pub use read::Read;
pub use write::Write;

use crate::{
    archetype::{Archetype, ArchetypeIndex},
    component::Component,
    entity::Entity,
    storage::{ArchetypeStorage, Components, Storage},
    world::World,
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

pub trait Readonly {}

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
    pub fn get<'world>(&self, world: &'world World, entity: Entity) -> Option<<T as Fetch<'world>>::Item>
    where
        T: Readonly,
    {
        let data = world.entities().get(entity)?;
        let index: &[ArchetypeIndex] = &[data.archetype()];
        let index = unsafe { std::mem::transmute(index) };
        let mut iter = T::fetch(world.components(), world.archetypes(), index);

        iter.nth(data.component().0 as usize)
    }

    pub fn get_mut<'world>(&self, world: &'world mut World, entity: Entity) -> Option<<T as Fetch<'world>>::Item> {
        let data = world.entities().get(entity)?;
        let index: &[ArchetypeIndex] = &[data.archetype()];
        let index = unsafe { std::mem::transmute(index) };
        let mut iter = T::fetch(world.components(), world.archetypes(), index);

        iter.nth(data.component().0 as usize)
    }

    pub fn iter<'world, 'index>(&'index self, world: &'world World) -> QueryIter<'world, 'index, T>
    where
        T: Readonly,
    {
        let index = self.find_archetypes(world);
        let index = unsafe { std::mem::transmute::<_, &'world [ArchetypeIndex]>(index) };

        QueryIter {
            iter: T::fetch(world.components(), world.archetypes(), index),
            _marker: PhantomData,
        }
    }

    pub fn iter_mut<'world, 'index>(&'index self, world: &'world mut World) -> QueryIter<'world, 'index, T> {
        let index = self.find_archetypes(world);
        let index = unsafe { std::mem::transmute::<_, &'world [ArchetypeIndex]>(index) };

        QueryIter {
            iter: T::fetch(world.components(), world.archetypes(), index),
            _marker: PhantomData,
        }
    }

    fn find_archetypes<'index>(&'index self, world: &World) -> &'index [ArchetypeIndex] {
        self.archetypes.get_or_init(move || {
            let components = T::components();

            world
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
