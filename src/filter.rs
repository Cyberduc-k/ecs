use crate::component as c;
use std::any::TypeId;
use std::marker::PhantomData;

pub trait LayoutFilter {
    fn matches(&self, components: &[TypeId]) -> bool;
}

pub struct Any;
pub struct And<T>(T);
pub struct Or<T>(T);
pub struct Not<T>(T);
pub struct Component<T: c::Component>(PhantomData<T>);

impl LayoutFilter for Any {
    fn matches(&self, _: &[TypeId]) -> bool {
        true
    }
}

impl<T: LayoutFilter> LayoutFilter for Not<T> {
    fn matches(&self, components: &[TypeId]) -> bool {
        !self.0.matches(components)
    }
}

impl<T: c::Component> LayoutFilter for Component<T> {
    fn matches(&self, components: &[TypeId]) -> bool {
        components.contains(&TypeId::of::<T>())
    }
}

impl Default for Any {
    fn default() -> Self {
        Self
    }
}

impl<T: Default> Default for Not<T> {
    fn default() -> Self {
        Self(T::default())
    }
}

impl<T: c::Component> Default for Component<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

macro_rules! impl_tuple {
    ($head:ident) => {
        impl_tuple!(@impl);
        impl_tuple!(@impl $head);
    };

    ($head:ident, $($tail:ident),+) => {
        impl_tuple!($($tail),+);
        impl_tuple!(@impl $head, $($tail),+);
    };

    (@impl $($ty:ident),*) => {
        impl<$($ty: LayoutFilter),*> LayoutFilter for And<($($ty,)*)> {
            #[allow(non_snake_case, unused_variables)]
            fn matches(&self, components: &[TypeId]) -> bool {
                let Self(($($ty,)*)) = self;
                $($ty.matches(components) &&)* true
            }
        }

        impl<$($ty: LayoutFilter),*> LayoutFilter for Or<($($ty,)*)> {
            #[allow(non_snake_case, unused_variables)]
            fn matches(&self, components: &[TypeId]) -> bool {
                let Self(($($ty,)*)) = self;
                $($ty.matches(components) ||)* true
            }
        }

        impl<$($ty: Default),*> Default for And<($($ty,)*)> {
            fn default() -> Self {
                Self(($($ty::default(),)*))
            }
        }

        impl<$($ty: Default),*> Default for Or<($($ty,)*)> {
            fn default() -> Self {
                Self(($($ty::default(),)*))
            }
        }
    };
}

impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U,  V, W, X, Y, Z);