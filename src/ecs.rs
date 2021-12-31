use crate::walker;
/// Register components, resources, and build a dispatcher
/// defining system dependencies
/// https://specs.amethyst.rs/docs/tutorials/03_dispatcher.html
use specs::{Dispatcher, DispatcherBuilder, World};

use crate::aggregate;
use crate::globals;
use crate::integrator;
use crate::neighborhood::Neighborhood;
use crate::output;
use crate::render;

/// Registers all components used by the modules of the program.
pub fn register_components(world: &mut World) {
    walker::register_components(world);
}

pub fn register_resources(world: &mut World) {
    world.insert(integrator::Step { n: 0 });
    world.insert(integrator::Timestep {
        delta: globals::TIMESTEP,
    });
    world.insert(render::FrameRate::new());
    world.insert(walker::Counter { fix: 0, tot: 0 });
    world.insert(Neighborhood::new_from_field(
        globals::WIDTH,
        globals::HEIGHT,
        globals::NEIGH_SIZE,
        globals::NEIGH_SIZE,
    ));
}

#[derive(Default)]
pub struct MyDispatcherBuilder {
    pub builder: DispatcherBuilder<'static, 'static>,
}
impl MyDispatcherBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    //https://specs.amethyst.rs/docs/tutorials/03_dispatcher.html#building-a-dispatcher
    fn add_systems(&mut self) {
        self.builder.add(
            integrator::VerletUpdatePositionSystem,
            "update_position",
            &[],
        );
        self.builder
            .add(aggregate::AggregateSystem, "aggregate", &[]);
        self.builder.add(walker::CounterSystem, "counter", &[]);
        self.builder
            .add(output::console::OutputPositionSystem, "output", &[]);
    }

    pub fn build(mut self) -> DispatcherBuilder<'static, 'static> {
        self.add_systems();
        self.builder
    }
}

pub fn create_dispatcher_builder() -> DispatcherBuilder<'static, 'static> {
    let builder = MyDispatcherBuilder::new();
    builder.build()
}

pub fn create_dispatcher() -> Dispatcher<'static, 'static> {
    let builder = create_dispatcher_builder();
    builder.build()
}
