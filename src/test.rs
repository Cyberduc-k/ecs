use ecs::entity::Entity;
use ecs::query::IntoQuery;
use ecs::world::World;

fn main() {
    let mut world = World::default();

    world.create((32i32, 8i8, true));
    let e = world.create(("test", 324i32));
    world.create((64i32, 16i8, false));

    if let Some(i) = <&mut i32>::query().get_mut(&mut world, e) {
        println!("{}", i);
    }

    for (e, i) in <(Entity, &mut i32)>::query().iter_mut(&mut world) {
        *i += 5;
        println!("{:?}: {}", e, i);
    }
}