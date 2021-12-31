/// Simple world example, no dispatcher
/// https://specs.amethyst.rs/docs/tutorials/02_hello_world.html
extern crate dla as lib;
use specs::prelude::*;

use lib::ecs;
use lib::walker::Position;
use lib::output::console::OutputPositionSystem;

fn main() {
    let mut world = World::new();
    ecs::register_components(&mut world);
    ecs::register_resources(&mut world);


    world.create_entity().with(Position::new(1.2, 1.1)).build();

    let mut read_system = OutputPositionSystem;
    read_system.run_now(&world);
    world.maintain();
    
}
