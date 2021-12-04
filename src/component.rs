use crate::archetype::ArchetypeDescriptor;
use crate::insert::EntityInserter;
use crate::storage::{Storage, VecStorage};

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct ComponentIndex(pub(crate) u32);

pub trait Component: Sized + 'static {
    type Storage: for<'a> Storage<'a, Self>;
}

pub trait ComponentSource: ArchetypeDescriptor {
    fn insert_components(self, inserter: &mut EntityInserter<'_>);
}

impl<T: 'static> Component for T {
    type Storage = VecStorage<Self>;
}

macro_rules! impl_component_source {
    ($head:ident) => {
        impl_component_source!(@impl $head);
    };

    ($head:ident, $($tail:ident),+) => {
        impl_component_source!($($tail),+);
        impl_component_source!(@impl $head, $($tail),+);
    };

    (@impl $($ty:ident),+) => {
        impl<$($ty: Component),+> ComponentSource for ($($ty,)+) {
            #[allow(non_snake_case)]
            fn insert_components(self, inserter: &mut EntityInserter<'_>) {
                let ($($ty,)+) = self;

                $(
                    let mut edit = inserter.component::<$ty>();
                    edit.extend(std::iter::once($ty));
                )+

                inserter.finish_entity();
            }
        }
    };
}

impl_component_source!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);