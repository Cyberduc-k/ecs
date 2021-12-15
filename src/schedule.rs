use crate::resource::{ResourceSet, Resources};
use crate::system::{QuerySet, System, SystemFn};
use crate::type_list::{Append, Concat, Flatten, UnFlatten};
use crate::world::World;

pub struct Schedule<S> {
    systems: S,
}

pub struct DynSchedule<'system> {
    systems: Vec<Box<dyn DynSystem + 'system>>,
}

pub trait Systems {
    fn run(&mut self, world: &mut World, resources: &mut Resources);
}

pub trait SystemBundle {
    type Added: Systems + UnFlatten;

    fn load<S>(self, schedule: Schedule<S>, resources: &mut Resources) -> Schedule<S::Output>
    where
        S: Concat<<Self::Added as UnFlatten>::Output>;
}

pub trait DynSystemBundle<'system>: 'system {
    fn load(self, schedule: &mut DynSchedule<'system>, resources: &mut Resources);
}

pub trait DynSystem {
    fn run(&mut self, world: &mut World, resources: &mut Resources);
}

impl<T: System> DynSystem for T {
    fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let queries = T::Queries::fetch(world);
        let resources = unsafe { T::Resources::fetch_unchecked(resources) };

        System::run(self, queries, resources);
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

    pub fn with_systems<T>(self, systems: T) -> Schedule<S::Output>
    where
        T: Systems + UnFlatten,
        S: Concat<T::Output>,
    {
        Schedule {
            systems: self.systems.concat(systems.unflatten()),
        }
    }

    pub fn with_system_fn<F>(self, func: F) -> Schedule<S::Output>
    where
        S: Append<SystemFn<F>>,
        F: for<'data> FnMut(&'data mut World, &'data Resources),
    {
        Schedule {
            systems: self.systems.append(SystemFn(func)),
        }
    }

    pub fn with_bundle<B>(self, bundle: B, resources: &mut Resources) -> Schedule<S::Output>
    where
        B: SystemBundle,
        S: Concat<<B::Added as UnFlatten>::Output>,
    {
        bundle.load(self, resources)
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
    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        self.systems.run(world, resources);
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

    pub fn with_system_fn<F: for<'data> FnMut(&'data mut World, &'data Resources) + 'system>(self, func: F) -> Self {
        self.with_system(SystemFn(func))
    }

    pub fn with_bundle<B: DynSystemBundle<'system>>(mut self, bundle: B, resources: &mut Resources) -> Self {
        self.add_bundle(bundle, resources);
        self
    }

    pub fn add_system<S: DynSystem + 'system>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn add_system_fn<F: for<'data> FnMut(&'data mut World, &'data Resources) + 'system>(&mut self, func: F) {
        self.add_system(SystemFn(func));
    }

    pub fn add_bundle<B: DynSystemBundle<'system>>(&mut self, bundle: B, resources: &mut Resources) {
        bundle.load(self, resources);
    }

    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        let world = world as *mut World;
        let resources = resources as *mut Resources;

        self.systems
            .iter_mut()
            .for_each(move |system| unsafe { system.run(&mut *world, &mut *resources) });
    }
}

macro_rules! impl_systems {
    ($head:ident) => {
        impl_systems!(@impl);
        impl_systems!(@impl $head);
    };

    ($head:ident, $($tail:ident),+) => {
        impl_systems!($($tail),+);
        impl_systems!(@impl $head, $($tail),+);
    };

    (@impl $($ty:ident),*) => {
        impl<$($ty: System),*> Systems for ($($ty,)*) {
            #[allow(non_snake_case, unused_unsafe, unused_variables)]
            fn run(&mut self, world: &mut World, resources: &mut Resources) {
                let ($($ty,)*) = self;

                unsafe {
                    $($ty.run(
                        $ty::Queries::fetch(world),
                        $ty::Resources::fetch_unchecked(resources),
                    );)*
                }
            }
        }
    };
}

impl_systems!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
