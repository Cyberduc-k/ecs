use ecs::entity::Entity;
use ecs::query::IntoQuery;
use ecs::resource::Resources;
use ecs::schedule::Schedule;
use ecs::world::World;

fn main() {
    let mut world = World::default();
    let mut resources = Resources::default();

    world.create((32i32, 8i8, true));
    world.create(("test", 324i32));
    world.create((64i32, 16i8, false));

    for _i in <&i32>::query().iter(&world) {}

    let mut schedule = Schedule::new()
        .with_system(A)
        .with_system(B::default())
        .with_system_fn(|_world, _resources| {})
        .finish();

    schedule.run(&mut world, &mut resources);
}

use ecs::query::{Read, Write};
use ecs::system::{QuerySet, System};

struct A;

#[derive(Default)]
struct B<'a>(Vec<&'a i32>);

impl System for A {
    type Resources = ();
    type Queries = ((Read<i32>, Write<i8>), (Entity, (Read<i32>, Read<&'static str>)));

    fn run(&mut self, (mut a, b): <Self::Queries as QuerySet>::Result, _: ()) {
        for (int, byte) in a.iter_mut() {
            println!("{:?}, {:?}", int, byte);
        }

        for (int, string) in b.iter() {
            println!("{:?}, {:?}", int, string);
        }
    }
}

impl<'a> System for B<'a> {
    type Resources = ();
    type Queries = (Read<i32>,);

    fn run(&mut self, _ints: <Self::Queries as QuerySet>::Result, _: ()) {
        dbg!(&self.0);
    }
}
