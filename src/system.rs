use crate::query::{self, IntoQuery, QueryIter};
use crate::resource::{AllResources, Readonly, ResourceSet, Resources};
use crate::subworld::SubWorld;
use crate::world::World;

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

pub struct WholeWorld;

pub struct SystemQuery<'world, T: IntoQuery> {
    world: SubWorld<'world>,
    query: query::Query<T::Fetch>,
}

pub struct SystemFn<F>(pub F);

impl<F> System for SystemFn<F>
where
    F: for<'data> FnMut(&'data mut World, &'data Resources),
{
    type Resources = AllResources;
    type Queries = WholeWorld;

    #[inline]
    fn run(&mut self, world: &mut World, resources: &Resources) {
        (self.0)(world, resources)
    }
}

impl<'world> QuerySet<'world> for WholeWorld {
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
