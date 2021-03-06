use ecs::entity::Entity;
use ecs::resource::Resources;
use ecs::schedule::Schedule;
use ecs::world::World;

fn main() {
    let mut world = World::default();
    let mut resources = Resources::default();

    world.create((32i32, 8i8, true));
    world.create(("test", 324i32));
    world.create((64i32, 16i8, false));

    let mut schedule = Schedule::new()
        .with_system(A)
        .finish();

    schedule.run(&mut world, &mut resources);
}

use ecs::query::{Read, TryWrite};
use ecs::system::{QuerySet, System};

struct A;

impl System for A {
    type Resources = ();
    type Queries = (
        (Read<i32>, TryWrite<i8>),
        (Entity, Read<i32>, Read<&'static str>)
    );

    fn run(&mut self, (mut a, b): <Self::Queries as QuerySet>::Result, _: ()) {
        for (int, byte) in a.iter_mut() {
            println!("{:?}, {:?}", int, byte);
        }

        for (entity, int, string) in b.iter() {
            println!("{:?}, ({:?}, {:?})", entity, int, string);
        }
    }
}
