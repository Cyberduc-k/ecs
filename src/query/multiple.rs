use super::*;

pub struct Multiple<T>(T);
pub struct MultiIter<T>(T);

macro_rules! impl_multi {
    ($head:ident) => {
        impl_multi!(@impl $head);
    };

    ($head:ident, $($tail:ident),+) => {
        impl_multi!($($tail),+);
        impl_multi!(@impl $head, $($tail),+);
    };

    (@impl $($ty:ident),*) => {
        impl<'a, $($ty: Fetch<'a>),+> Fetch<'a> for Multiple<($($ty,)+)> {
            type Item = ($($ty::Item,)+);
            type Iter = MultiIter<($($ty::Iter,)+)>;

            #[allow(non_snake_case)]
            fn fetch(components: &'a Components, archetypes: &'a [Archetype], index: &'a [ArchetypeIndex]) -> Self::Iter {
                $(let $ty = $ty::fetch(components, archetypes, index);)*
                MultiIter(($($ty,)+))
            }
        }

        impl<$($ty: Readonly),+> Readonly for Multiple<($($ty,)+)> {}

        impl<'a, $($ty: Fetch<'a>),+> ComponentTypes for Multiple<($($ty,)+)> {
            fn components() -> Vec<TypeId> {
                let mut result = Vec::new();
                $(result.append(&mut $ty::components());)*
                result
            }
        }
    };
}

impl_multi!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

macro_rules! impl_multi_iter {
    ($head:ident) => {
        impl_multi_iter!(@impl $head);
    };

    ($head:ident, $($tail:ident),+) => {
        impl_multi_iter!($($tail),+);
        impl_multi_iter!(@impl $head, $($tail),+);
    };

    (@impl $($ty:ident),*) => {
        impl<$($ty: Iterator),+> Iterator for MultiIter<($($ty,)+)> {
            type Item = ($($ty::Item,)+);

            #[allow(non_snake_case)]
            fn next(&mut self) -> Option<Self::Item> {
                let Self(($($ty,)+)) = self;
                $(let $ty = $ty.next()?;)+
                Some(($($ty,)+))
            }
        }
    };
}

impl_multi_iter!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);