use crate::system::{System, SystemData};
use crate::type_list::{Append, Flatten};
use crate::world::World;

pub struct Schedule<S> {
    systems: S,
}

pub trait Systems<'a> {
    fn run(&mut self, world: &'a mut World);
}

impl Schedule<()> {
    pub fn new() -> Self {
        Self {
            systems: (),
        }
    }
}

impl<S> Schedule<S> {
    pub fn with_system<'a, T>(self, system: T) -> Schedule<S::Output>
    where
        S: Append<T>,
        T: System<'a>,
    {
        Schedule {
            systems: self.systems.append(system),
        }
    }
}

impl<S: Flatten> Schedule<S> {
    pub fn finish(self) -> Schedule<S::Output> {
        Schedule {
            systems: self.systems.flatten(),
        }
    }
}

impl<'a, S: Systems<'a>> Schedule<S> {
    pub fn run(&mut self, world: &'a mut World) {
        self.systems.run(world);
    }
}

impl Systems<'_> for () {
    fn run(&mut self, _: &mut World) {}
}

macro_rules! impl_systems {
    ($head:ident) => {
        impl_systems!(@impl $head);
    };

    ($head:ident, $($tail:ident),+) => {
        impl_systems!($($tail),+);
        impl_systems!(@impl $head, $($tail),+);
    };

    (@impl $($ty:ident),+) => {
        impl<'a, $($ty: System<'a>),+> Systems<'a> for ($($ty,)+) {
            #[allow(non_snake_case)]
            fn run(&mut self, world: &'a mut World) {
                let world = world as *mut World;
                let ($($ty,)+) = self;
                $($ty.run($ty::Data::fetch(unsafe { &mut *world }));)+
            }
        }
    };
}

impl_systems!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);