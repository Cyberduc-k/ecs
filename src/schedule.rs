use crate::system::{System, SystemData, SystemFn};
use crate::type_list::{Append, Flatten};
use crate::world::World;

pub struct Schedule<S> {
    systems: S,
}

pub struct DynSchedule<'system> {
    systems: Vec<Box<dyn DynSystem + 'system>>,
}

pub trait Systems {
    fn run(&mut self, world: &mut World);
}

pub trait SystemBundle<S> {
    type Output;

    fn with_systems(self, schedule: Schedule<S>) -> Schedule<Self::Output>;
}

pub trait DynSystemBundle<'system>: 'system {
    fn add_systems(self, schedule: &mut DynSchedule<'system>);
}

impl<S, A: System, B: System> SystemBundle<S> for (A, B)
where
    S: Append<A>,
    <S as Append<A>>::Output: Append<B>,
{
    type Output = <<S as Append<A>>::Output as Append<B>>::Output;

    fn with_systems(self, schedule: Schedule<S>) -> Schedule<Self::Output> {
        schedule.with_system(self.0).with_system(self.1)
    }
}

pub trait DynSystem {
    fn run(&mut self, world: &mut World);
}

impl<T: System> DynSystem for T {
    fn run(&mut self, world: &mut World) {
        let data = T::Data::fetch(world);
        System::run(self, data);
    }
}

impl Schedule<()> {
    pub fn new() -> Self {
        Self { systems: () }
    }
}

impl<S> Schedule<S> {
    pub fn with_system<T>(self, system: T) -> Schedule<S::Output>
    where
        S: Append<T>,
        T: System,
    {
        Schedule {
            systems: self.systems.append(system),
        }
    }

    pub fn with_system_fn<F>(self, func: F) -> Schedule<S::Output>
    where
        S: Append<SystemFn<F>>,
        F: for<'world> FnMut(&'world mut World),
    {
        Schedule {
            systems: self.systems.append(SystemFn(func)),
        }
    }

    pub fn with_bundle<B>(self, bundle: B) -> Schedule<B::Output>
    where
        B: SystemBundle<S>,
    {
        bundle.with_systems(self)
    }
}

impl<S: Flatten> Schedule<S> {
    pub fn finish(self) -> Schedule<S::Output> {
        Schedule {
            systems: self.systems.flatten(),
        }
    }
}

impl<S: Systems> Schedule<S> {
    pub fn run(&mut self, world: &mut World) {
        self.systems.run(world);
    }
}

impl<'system> DynSchedule<'system> {
    pub fn new() -> Self {
        Self { systems: Vec::new() }
    }

    pub fn with_system<S: System + 'system>(mut self, system: S) -> Self {
        self.add_system(system);
        self
    }

    pub fn with_system_fn<F: for<'world> FnMut(&'world mut World) + 'system>(self, func: F) -> Self {
        self.with_system(SystemFn(func))
    }

    pub fn with_bundle<B: DynSystemBundle<'system>>(mut self, bundle: B) -> Self {
        self.add_bundle(bundle);
        self
    }

    pub fn add_system<S: DynSystem + 'system>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn add_system_fn<F: for<'world> FnMut(&'world mut World) + 'system>(&mut self, func: F) {
        self.add_system(SystemFn(func));
    }

    pub fn add_bundle<B: DynSystemBundle<'system>>(&mut self, bundle: B) {
        bundle.add_systems(self);
    }

    pub fn run(&mut self, world: &mut World) {
        let world = world as *mut World;

        self.systems
            .iter_mut()
            .for_each(move |system| unsafe { system.run(&mut *world) });
    }
}

impl Systems for () {
    fn run(&mut self, _: &mut World) {
    }
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
        impl<$($ty: System),+> Systems for ($($ty,)+) {
            #[allow(non_snake_case)]
            fn run(&mut self, world: &mut World) {
                let world = world as *mut World;
                let ($($ty,)+) = self;
                $($ty.run($ty::Data::fetch(unsafe { &mut *world }));)+
            }
        }
    };
}

impl_systems!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
