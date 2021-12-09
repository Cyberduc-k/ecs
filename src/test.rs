use ecs::entity::Entity;
use ecs::schedule::Schedule;
use ecs::world::World;

fn main() {
    let mut world = World::default();

    world.create((32i32, 8i8, true));
    world.create(("test", 324i32));
    world.create((64i32, 16i8, false));

    let mut schedule = Schedule::new()
        .with_system(A)
        .with_system(B::default())
        .with_system(|world| {})
        .finish();
    
    schedule.run(&mut world);
}

use ecs::query::{Read, Write};
use ecs::system::{Query, System, SystemData};

struct A;

#[derive(Default)]
struct B<'a>(Vec<&'a i32>);

impl<'a> System<'a> for A {
    type Data = (
        Query<(Read<i32>, Write<i8>)>,
        Query<(Entity, (Read<i32>, Read<&'static str>))>,
    );

    fn run(&mut self, (mut a, mut b): <Self::Data as SystemData<'a>>::Result) {
        for (int, byte) in a.iter_mut() {
            println!("{:?}, {:?}", int, byte);
        }

        for (int, string) in b.iter() {
            println!("{:?}, {:?}", int, string);
        }
    }
}

impl<'a> System<'a> for B<'a> {
    type Data = Query<Read<i32>>;

    fn run(&mut self, mut ints: <Self::Data as SystemData<'a>>::Result) {
        self.0.extend(ints.iter());
        dbg!(&self.0);
    }
}