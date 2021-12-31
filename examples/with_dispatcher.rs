/// Use dispatcher to handle interdependent systems
/// https://specs.amethyst.rs/docs/tutorials/03_dispatcher.html
extern crate dla as lib;
use specs::prelude::*;

use lib::ecs;
use lib::walker::{Position, Velocity};
use lib::integrator::Timestep;

fn main() {
    let mut world = World::new();
    ecs::register_components(&mut world);
    ecs::register_resources(&mut world);

    let mut dispatcher = ecs::create_dispatcher();
    dispatcher.setup(&mut world);

    world.create_entity()
        .with(Position::new(1.2, 1.1))
        .with(Velocity::new(1.0, 0.0))
        .build();
    world.create_entity()
        .with(Position::new(-1.2, 1.1))
        .with(Velocity::new(-1.0, 0.0))
        .build();
    
    world.insert(Timestep{delta: 0.05});

    for _i in 0..10 {
        dispatcher.dispatch(&mut world);
        world.maintain();
    }
    
}
