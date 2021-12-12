use crate::query::{self, Fetch, IntoQuery, QueryIter, Readonly};
use crate::resource::Resources;
use crate::world::World;
use std::marker::PhantomData;

pub trait System {
    type Data: for<'data> SystemData<'data>;

    fn run(&mut self, data: <Self::Data as SystemData>::Result);
}

pub trait SystemData<'data>: Sized {
    type Result: 'data;

    fn fetch(world: &'data mut World, resources: &'data mut Resources) -> Self::Result;
}

pub struct Query<T: IntoQuery>(PhantomData<fn() -> T::Fetch>);
pub struct WorldAndResources;

pub struct SystemQuery<'data, T: for<'fetch> Fetch<'fetch>> {
    world: *mut World,
    query: query::Query<T>,
    _marker: PhantomData<fn(&'data World) -> T>,
}

pub struct SystemFn<F>(pub F);

impl<F> System for SystemFn<F>
where
    F: for<'data> FnMut(&'data mut World, &'data Resources),
{
    type Data = WorldAndResources;

    #[inline]
    fn run(&mut self, (world, resources): <Self::Data as SystemData>::Result) {
        (self.0)(world, resources)
    }
}

impl SystemData<'_> for () {
    type Result = ();

    #[inline]
    fn fetch(_: &mut World, _: &mut Resources) -> Self::Result {
        ()
    }
}

impl<'data> SystemData<'data> for WorldAndResources {
    type Result = (&'data mut World, &'data Resources);

    #[inline]
    fn fetch(world: &'data mut World, resources: &'data mut Resources) -> Self::Result {
        (world, resources)
    }
}

impl<'data, T: IntoQuery> SystemData<'data> for Query<T>
where
    T::Fetch: 'data,
{
    type Result = SystemQuery<'data, T::Fetch>;

    fn fetch(world: &'data mut World, _: &'data mut Resources) -> Self::Result {
        SystemQuery {
            world,
            query: T::query(),
            _marker: PhantomData,
        }
    }
}

impl<'data, T: for<'fetch> Fetch<'fetch>> SystemQuery<'data, T> {
    pub fn iter<'index>(&'index self) -> QueryIter<'data, 'index, T>
    where
        T: Readonly,
    {
        self.query.iter(unsafe { &*self.world })
    }

    pub fn iter_mut<'index>(&'index self) -> QueryIter<'data, 'index, T> {
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
        impl<'data, $($ty: SystemData<'data>),+> SystemData<'data> for ($($ty,)+) {
            type Result = ($($ty::Result,)*);

            fn fetch(world: &'data mut World, resources: &'data mut Resources) -> Self::Result {
                let world = world as *mut World;
                let resources = resources as *mut Resources;

                unsafe {
                    ($($ty::fetch(&mut *world, &mut *resources),)*)
                }
            }
        }
    };
}

impl_system_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
