use crate::query::{self, Fetch, IntoQuery, QueryIter, Readonly};
use crate::world::World;
use std::marker::PhantomData;

pub trait System {
    type Data: for<'world> SystemData<'world>;

    fn run(&mut self, data: <Self::Data as SystemData>::Result);
}

pub trait SystemData<'world>: Sized {
    type Result: 'world;

    fn fetch(world: &'world mut World) -> Self::Result;
}

pub struct Query<T: IntoQuery>(PhantomData<fn() -> T::Fetch>);
pub struct WorldData;

pub struct SystemQuery<'world, T: for<'fetch> Fetch<'fetch>> {
    world: *mut World,
    query: query::Query<T>,
    _marker: PhantomData<fn(&'world World) -> T>,
}

pub struct SystemFn<F: for<'world> FnMut(&'world mut World)>(pub F);

impl<F> System for SystemFn<F>
where
    F: for<'world> FnMut(&'world mut World),
{
    type Data = WorldData;

    #[inline]
    fn run(&mut self, world: &mut World) {
        (self.0)(world)
    }
}

impl SystemData<'_> for () {
    type Result = ();

    #[inline]
    fn fetch(_: &mut World) -> Self::Result {
        ()
    }
}

impl<'world> SystemData<'world> for WorldData {
    type Result = &'world mut World;

    #[inline]
    fn fetch(world: &'world mut World) -> Self::Result {
        world
    }
}

impl<'world, T: IntoQuery> SystemData<'world> for Query<T>
where
    T::Fetch: 'world,
{
    type Result = SystemQuery<'world, T::Fetch>;

    fn fetch(world: &'world mut World) -> Self::Result {
        SystemQuery {
            world,
            query: T::query(),
            _marker: PhantomData,
        }
    }
}

impl<'world, T: for<'fetch> Fetch<'fetch>> SystemQuery<'world, T> {
    pub fn iter<'index>(&'index self) -> QueryIter<'world, 'index, T>
    where
        T: Readonly,
    {
        self.query.iter(unsafe { &*self.world })
    }

    pub fn iter_mut<'index>(&'index self) -> QueryIter<'world, 'index, T> {
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
        impl<'world, $($ty: SystemData<'world>),+> SystemData<'world> for ($($ty,)+) {
            type Result = ($($ty::Result,)*);

            fn fetch(world: &'world mut World) -> Self::Result {
                let world = world as *mut World;

                unsafe {
                    ($($ty::fetch(&mut *world),)*)
                }
            }
        }
    };
}

impl_system_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
