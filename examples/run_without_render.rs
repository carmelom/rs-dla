/// Use dispatcher to handle interdependent systems
/// https://specs.amethyst.rs/docs/tutorials/03_dispatcher.html
extern crate dla as lib;
use specs::prelude::*;

use lib::ecs;
use lib::walker;
use lib::globals;

fn main() {
    let mut world = World::new();
    ecs::register_components(&mut world);
    ecs::register_resources(&mut world);

    let mut dispatcher = ecs::create_dispatcher();
    dispatcher.setup(&mut world);

    walker::spawn_walkers(&mut world, globals::NUM_WALKERS);

    loop {
        dispatcher.dispatch(&mut world);
        world.maintain();
        let counter = world.read_resource::<walker::Counter>();
        if counter.fix > 90 * counter.tot / 100 {
            break;
        }
    }
    
}
