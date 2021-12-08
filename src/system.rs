use crate::query::{self, Fetch, IntoQuery, Readonly, QueryIter};
use crate::world::World;
use std::marker::PhantomData;

pub trait System<'a> {
    type Data: SystemData<'a>;

    fn run(&mut self, data: <Self::Data as SystemData<'a>>::Result);
}

pub trait SystemData<'a>: Sized {
    type Result: 'a;

    fn fetch(world: &'a mut World) -> Self::Result;
}

pub struct Query<T: IntoQuery>(PhantomData<fn() -> T::Fetch>);

pub struct SystemQuery<'a, T: for<'b> Fetch<'b>> {
    world: *mut World,
    query: query::Query<T>,
    _marker: PhantomData<fn(&'a World) -> T>
}

impl<'a, T: IntoQuery> SystemData<'a> for Query<T>
where
    T::Fetch: 'a,
{
    type Result = SystemQuery<'a, T::Fetch>;

    fn fetch(world: &'a mut World) -> Self::Result {
        SystemQuery {
            world,
            query: T::query(),
            _marker: PhantomData,
        }
    }
}

impl<'a, T: for<'b> Fetch<'b>> SystemQuery<'a, T> {
    pub fn iter<'b>(&'b mut self) -> QueryIter<'a, 'b, T>
    where
        T: Readonly,
    {
        self.query.iter(unsafe { &*self.world })
    }

    pub fn iter_mut<'b>(&'b mut self) -> QueryIter<'a, 'b, T> {
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
                let world = world as *mut World;

                unsafe {
                    ($($ty::fetch(&mut *world),)*)
                }
            }
        }
    };
}

impl_system_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);