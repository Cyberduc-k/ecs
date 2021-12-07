use ecs::entity::Entity;
use ecs::query::IntoQuery;
use ecs::world::World;

fn main() {
    let mut world = World::default();

    world.create((32i32, 8i8, true));
    world.create(("test", 324i32));
    world.create((64i32, 16i8, false));

    for (e, i) in <(Entity, &mut i32)>::query().iter_mut(&mut world) {
        *i += 5;
        println!("{:?}: {}", e, i);
    }
}

use ecs::query::Read;
use ecs::system::{Query, System, SystemData};

struct TestSystem;

impl System for TestSystem {
    type Data = Query<(Read<i32>, Read<i8>)>;

    fn run(&mut self, data: <Self::Data as SystemData>::Result) {}
}