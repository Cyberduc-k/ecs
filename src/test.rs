fn main() {
    let mut world = ecs::world::World::default();
    let a = world.create((32i32, 8i8, true));
    let b = world.create((64i32, 16i8, false));
    let c = world.create(("test",));

    dbg!(a, b, c);

    if let Some(entry) = world.entry(b) {
        let comp = entry.component::<i8>();

        dbg!(comp);
    }
}

