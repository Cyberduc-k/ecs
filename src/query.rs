mod entity;
mod read;
mod write;
mod multiple;

pub use read::Read;
pub use write::Write;

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

    fn query() -> Query<Self> {
        Query::default()
    }
}

pub struct Query<T: IntoQuery> {
    _marker: PhantomData<T>,
    archetypes: Option<Vec<ArchetypeIndex>>,
}

pub trait Fetch<'a>: ComponentTypes {
    type Item: 'a;
    type Iter: Iterator<Item = Self::Item>;

    fn fetch(components: &'a Components, archetypes: &'a [Archetype], index: &'a [ArchetypeIndex]) -> Self::Iter;
}

pub trait ComponentTypes {
    fn components() -> Vec<TypeId>;
}

pub trait Readonly {}

impl<T: IntoQuery> Default for Query<T> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
            archetypes: None,
        }
    }
}

impl<T: IntoQuery> Clone for Query<T> {
    fn clone(&self) -> Self {
        Self {
            _marker: PhantomData,
            archetypes: self.archetypes.clone(),
        }
    }
}

impl<T: IntoQuery> Query<T> {
    pub fn get<'a>(&'a mut self, world: &'a World, entity: Entity) -> Option<<T::Fetch as Fetch<'a>>::Item>
    where
        T::Fetch: Readonly,
    {
        let data = world.entities().get(entity)?;
        let index: &[ArchetypeIndex] = &[data.archetype()];
        let index = unsafe { std::mem::transmute(index) };
        let mut iter = <T::Fetch as Fetch<'a>>::fetch(world.components(), world.archetypes(), index);

        iter.nth(data.component().0 as usize)
    }

    pub fn get_mut<'a>(&'a mut self, world: &'a mut World, entity: Entity) -> Option<<T::Fetch as Fetch<'a>>::Item> {
        let data = world.entities().get(entity)?;
        let index: &[ArchetypeIndex] = &[data.archetype()];
        let index = unsafe { std::mem::transmute(index) };
        let mut iter = <T::Fetch as Fetch<'a>>::fetch(world.components(), world.archetypes(), index);

        iter.nth(data.component().0 as usize)
    }

    pub fn iter<'a>(&'a mut self, world: &'a World) -> <T::Fetch as Fetch<'a>>::Iter
    where
        T::Fetch: Readonly,
    {
        let index = self.find_archetypes(world);

        <T::Fetch as Fetch<'a>>::fetch(world.components(), world.archetypes(), index)
    }

    pub fn iter_mut<'a>(&'a mut self, world: &'a mut World) -> <T::Fetch as Fetch<'a>>::Iter {
        let index = self.find_archetypes(world);

        <T::Fetch as Fetch<'a>>::fetch(world.components(), world.archetypes(), index)
    }

    fn find_archetypes<'a>(&'a mut self, world: &World) -> &'a [ArchetypeIndex] {
        match self.archetypes {
            Some(ref archetypes) => archetypes,
            None => {
                let components = T::Fetch::components();
        
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
            
            fn query() -> Query<Self> {
                Query::default()
            }
        }
    };
}

impl_tuple_query!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);