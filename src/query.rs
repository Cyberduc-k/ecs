mod entity;
mod read;
mod write;
mod multiple;

pub use read::Read;
pub use write::Write;
pub use multiple::Multiple;

use crate::{
    archetype::{Archetype, ArchetypeIndex},
    component::Component,
    entity::Entity,
    storage::{ArchetypeStorage, Components, Storage},
    world::World,
};
use std::{
    any::TypeId,
    marker::PhantomData,
};

pub trait IntoQuery: Sized {
    type Fetch: for<'a> Fetch<'a>;

    fn query() -> Query<Self::Fetch> {
        Query::default()
    }
}

pub struct Query<T: for<'a> Fetch<'a>> {
    archetypes: Option<Vec<ArchetypeIndex>>,
    _marker: PhantomData<T>,
}

pub struct QueryIter<'data, 'index, F: Fetch<'data>> {
    iter: F::Iter,
    _marker: PhantomData<&'index [ArchetypeIndex]>,
}

pub trait Fetch<'a>: ComponentTypes {
    type Item: 'a;
    type Iter: Iterator<Item = Self::Item> + 'a;

    fn fetch(components: &'a Components, archetypes: &'a [Archetype], index: &'a [ArchetypeIndex]) -> Self::Iter;
}

pub trait ComponentTypes {
    fn components() -> Vec<TypeId>;
}

pub trait Readonly {}

impl<T: for<'a> Fetch<'a>> Default for Query<T> {
    fn default() -> Self {
        Self {
            archetypes: None,
            _marker: PhantomData,
        }
    }
}

impl<T: for<'a> Fetch<'a>> Clone for Query<T> {
    fn clone(&self) -> Self {
        Self {
            archetypes: self.archetypes.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T: for<'a> Fetch<'a>> Query<T> {
    pub fn get<'a>(&mut self, world: &'a World, entity: Entity) -> Option<<T as Fetch<'a>>::Item>
    where
        T: Readonly,
    {
        let data = world.entities().get(entity)?;
        let index: &[ArchetypeIndex] = &[data.archetype()];
        let index = unsafe { std::mem::transmute(index) };
        let mut iter = T::fetch(world.components(), world.archetypes(), index);

        iter.nth(data.component().0 as usize)
    }

    pub fn get_mut<'a>(&mut self, world: &'a mut World, entity: Entity) -> Option<<T as Fetch<'a>>::Item> {
        let data = world.entities().get(entity)?;
        let index: &[ArchetypeIndex] = &[data.archetype()];
        let index = unsafe { std::mem::transmute(index) };
        let mut iter = T::fetch(world.components(), world.archetypes(), index);

        iter.nth(data.component().0 as usize)
    }

    pub fn iter<'a, 'b>(&'b mut self, world: &'a World) -> QueryIter<'a, 'b, T>
    where
        T: Readonly,
    {
        let index = self.find_archetypes(world);
        let index = unsafe { std::mem::transmute::<_, &'a [ArchetypeIndex]>(index) };

        QueryIter {
            iter: T::fetch(world.components(), world.archetypes(), index),
            _marker: PhantomData,
        }
    }

    pub fn iter_mut<'a, 'b>(&'b mut self, world: &'a mut World) -> QueryIter<'a, 'b, T> {
        let index = self.find_archetypes(world);
        let index = unsafe { std::mem::transmute::<_, &'a [ArchetypeIndex]>(index) };

        QueryIter {
            iter: T::fetch(world.components(), world.archetypes(), index),
            _marker: PhantomData,
        }
    }

    fn find_archetypes<'a>(&'a mut self, world: &World) -> &'a [ArchetypeIndex] {
        match self.archetypes {
            Some(ref archetypes) => archetypes,
            None => {
                let components = T::components();
        
                self.archetypes = Some(world.archetypes().iter().filter_map(|a| if a.layout.contains(&components) {
                    Some(a.index)
                } else {
                    None
                }).collect());

                self.archetypes.as_ref().unwrap()
            }
        }
    }
}

impl<'data, 'index, T: Fetch<'data>> Iterator for QueryIter<'data, 'index, T> {
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