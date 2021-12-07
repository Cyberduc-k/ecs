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

    fn query<'a>() -> Query<'a, Self::Fetch> {
        Query::default()
    }
}

pub struct Query<'a, T: Fetch<'a>> {
    _marker: PhantomData<fn(&'a World) -> T>,
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

impl<'a, T: Fetch<'a>> Default for Query<'a, T> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
            archetypes: None,
        }
    }
}

impl<'a, T: Fetch<'a>> Clone for Query<'a, T> {
    fn clone(&self) -> Self {
        Self {
            _marker: PhantomData,
            archetypes: self.archetypes.clone(),
        }
    }
}

impl<'a, T: Fetch<'a>> Query<'a, T> {
    pub fn get(&mut self, world: &'a World, entity: Entity) -> Option<T::Item>
    where
        T: Readonly,
    {
        let data = world.entities().get(entity)?;
        let index: &[ArchetypeIndex] = &[data.archetype()];
        let index = unsafe { std::mem::transmute(index) };
        let mut iter = T::fetch(world.components(), world.archetypes(), index);

        iter.nth(data.component().0 as usize)
    }

    pub fn get_mut(&mut self, world: &'a mut World, entity: Entity) -> Option<T::Item> {
        let data = world.entities().get(entity)?;
        let index: &[ArchetypeIndex] = &[data.archetype()];
        let index = unsafe { std::mem::transmute(index) };
        let mut iter = T::fetch(world.components(), world.archetypes(), index);

        iter.nth(data.component().0 as usize)
    }

    pub fn iter(&'a mut self, world: &'a World) -> T::Iter
    where
        T: Readonly,
    {
        let index = self.find_archetypes(world);

        T::fetch(world.components(), world.archetypes(), index)
    }

    pub fn iter_mut(&'a mut self, world: &'a mut World) -> T::Iter {
        let index = self.find_archetypes(world);

        T::fetch(world.components(), world.archetypes(), index)
    }

    fn find_archetypes(&'a mut self, world: &World) -> &'a [ArchetypeIndex] {
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
            
            fn query<'a>() -> Query<'a, Self::Fetch> {
                Query::default()
            }
        }
    };
}

impl_tuple_query!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);