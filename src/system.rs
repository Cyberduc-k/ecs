use crate::query::{self, IntoQuery, QueryIter};
use crate::resource::{Readonly, ResourceSet, Resources};
use crate::subworld::SubWorld;
use crate::type_list::{Append, Flatten};
use crate::world::World;
use std::marker::PhantomData;

pub trait System {
    type Resources: for<'resources> ResourceSet<'resources>;
    type Queries: for<'world> QuerySet<'world>;

    fn run(
        &mut self,
        queries: <Self::Queries as QuerySet>::Result,
        resources: <Self::Resources as ResourceSet>::Result,
    );
}

pub trait QuerySet<'world>: Sized {
    type Result: 'world;

    fn fetch(world: &'world mut World) -> Self::Result;
}

pub struct SystemQuery<'world, T: IntoQuery> {
    world: SubWorld<'world>,
    query: query::Query<T::Fetch>,
}

pub struct AnySystem<R, Q, F>(F, PhantomData<(R, Q)>);

pub struct SystemFn<F>(pub F);

impl<R, Q, F> System for AnySystem<R, Q, F>
where
    R: for<'resources> ResourceSet<'resources>,
    Q: for<'world> QuerySet<'world>,
    F: for<'resources, 'world> FnMut(
        <Q as QuerySet<'world>>::Result,
        <R as ResourceSet<'resources>>::Result
    ),
{
    type Resources = R;
    type Queries = Q;

    #[inline]
    fn run(&mut self, queries: <Q as QuerySet>::Result, resources: <R as ResourceSet>::Result) {
        (self.0)(queries, resources)
    }
}

impl<F> System for SystemFn<F>
where
    F: for<'data> FnMut(&'data mut World, &'data Resources),
{
    type Resources = Resources;
    type Queries = World;

    #[inline]
    fn run(&mut self, world: &mut World, resources: &Resources) {
        (self.0)(world, resources)
    }
}

impl<'world> QuerySet<'world> for World {
    type Result = &'world mut World;

    fn fetch(world: &'world mut World) -> Self::Result {
        world
    }
}

impl<'world, T: IntoQuery> SystemQuery<'world, T> {
    pub fn iter<'index>(&'index self) -> QueryIter<'world, 'index, T::Fetch>
    where
        T::Fetch: Readonly,
    {
        self.query.iter(self.world.world())
    }

    pub fn iter_mut<'index>(&'index mut self) -> QueryIter<'world, 'index, T::Fetch> {
        self.query.iter_mut(unsafe { self.world.world_mut() })
    }
}

pub struct AnySystemBuilder<R, Q>(PhantomData<(R, Q)>);

impl AnySystem<(), (), ()> {
    pub fn new() -> AnySystemBuilder<(), ()> {
        AnySystemBuilder(PhantomData)
    }
}

impl<R, Q> AnySystemBuilder<R, Q> {
    pub fn with_query<T>(self) -> AnySystemBuilder<R, Q::Output>
    where
        Q: Append<T>,
        T: IntoQuery,
    {
        AnySystemBuilder(PhantomData)
    }

    pub fn with_resource<T>(self) -> AnySystemBuilder<R::Output, Q>
    where
        R: Append<T>,
        T: for<'resources> ResourceSet<'resources>,
    {
        AnySystemBuilder(PhantomData)
    }

    pub fn build<F>(self, f: F) -> AnySystem<R::Output, Q::Output, F>
    where
        R: Flatten,
        Q: Flatten,
        R::Output: for<'resources> ResourceSet<'resources>,
        Q::Output: for<'world> QuerySet<'world>,
        F: for<'resources, 'world> FnMut(
            <Q::Output as QuerySet<'world>>::Result,
            <R::Output as ResourceSet<'resources>>::Result,
        ),
    {
        AnySystem(f, PhantomData)
    }
}

macro_rules! impl_query_set {
    ($head:ident) => {
        impl_query_set!(@impl);
        impl_query_set!(@impl $head);
    };

    ($head:ident, $($tail:ident),+) => {
        impl_query_set!($($tail),+);
        impl_query_set!(@impl $head, $($tail),+);
    };

    (@impl $($ty:ident),*) => {
        impl<'world $(,$ty: IntoQuery + 'world)*> QuerySet<'world> for ($($ty,)*) {
            type Result = ($(SystemQuery<'world, $ty>,)*);

            #[allow(unused_variables)]
            fn fetch(world: &'world mut World) -> Self::Result {
                ($({
                    SystemQuery {
                        world: world.subworld(),
                        query: $ty::query(),
                    }
                },)*)
            }
        }
    };
}

impl_query_set!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
