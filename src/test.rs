use ecs::entity::Entity;
use ecs::query::IntoQuery;
use ecs::world::World;

fn main() {
    let mut world = World::default();

    world.create((32i32, 8i8, true));
    world.create(("test", 324i32));
    world.create((64i32, 16i8, false));

    let mut test = TestSystem;
    let data = <TestSystem as System>::Data::fetch(&mut world);

    test.run(data);
}

use ecs::query::{Read, Write};
use ecs::system::{Query, System, SystemData};

struct TestSystem;

impl<'a> System<'a> for TestSystem {
    type Data = (
        Query<(Read<i32>, Write<i8>)>,
        Query<(Read<i32>, Read<&'static str>)>,
    );

    fn run(&mut self, (mut a, mut b): <Self::Data as SystemData<'a>>::Result) {
        for (int, byte) in a.iter_mut() {
            println!("{}, {}", int, byte);
        }

        for (int, string) in b.iter() {
            println!("{}, {:?}", int, string);
        }
    }
}