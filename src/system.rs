use crate::query::{self, Fetch, IntoQuery, Readonly};
use crate::world::World;
use std::marker::PhantomData;

pub trait System {
    type Data: for<'a> SystemData<'a>;

    fn run(&mut self, data: <Self::Data as SystemData>::Result);
}

pub trait SystemData<'a>: Sized {
    type Result;

    fn fetch(world: &'a mut World) -> Self::Result;
}

pub struct Query<T: IntoQuery>(PhantomData<fn() -> T::Fetch>);

pub struct SystemQuery<'a, T: Fetch<'a>> {
    world: *mut World,
    query: query::Query<'a, T>,
}

impl<'a, T: IntoQuery> SystemData<'a> for Query<T> {
    type Result = SystemQuery<'a, T::Fetch>;

    fn fetch(world: &'a mut World) -> Self::Result {
        SystemQuery {
            world,
            query: T::query(),
        }
    }
}

impl<'a, T: Fetch<'a>> SystemQuery<'a, T> {
    pub fn iter(&'a mut self) -> T::Iter
    where
        T: Readonly,
    {
        self.query.iter(unsafe { &*self.world })
    }

    pub fn iter_mut(&'a mut self) -> T::Iter {
        self.query.iter_mut(unsafe { &mut *self.world })
    }
}

macro_rules! impl_system_data {
    ($head:ident) => {
        impl_system_data!(@impl $head);
    };

    ($head:ident, $($tail:ident),+) => {
        impl_system_data!($($tail),+);
        impl_system_data!(@impl $head, $($tail),+);
    };

    (@impl $($ty:ident),+) => {
        impl<'a, $($ty: SystemData<'a>),+> SystemData<'a> for ($($ty,)+) {
            type Result = ($($ty::Result,)*);

            fn fetch(world: &'a mut World) -> Self::Result {
                todo!();
            }
        }
    };
}

impl_system_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);