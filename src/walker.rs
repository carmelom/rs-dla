use crate::globals;
use crate::neighborhood::Neighborhood;
use nalgebra::Vector2;
use rand::{thread_rng, Rng};
use specs::prelude::*;
use specs::{Component, Entities, Join, NullStorage, VecStorage, World, WorldExt, WriteExpect};
use std::fmt;

// Position
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Position {
    pub pos: Vector2<f32>,
}
impl Default for Position {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}
impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position {
            pos: Vector2::new(x, y),
        }
    }
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.pos[0], self.pos[1])
    }
}

// Velocity
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Velocity {
    pub vel: Vector2<f32>,
}
impl Default for Velocity {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}
impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity {
            vel: Vector2::new(x, y),
        }
    }
}
impl fmt::Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.vel[0], self.vel[1])
    }
}

// Force
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Force {
    pub force: Vector2<f32>,
    pub old_force: Vector2<f32>,
}
impl Default for Force {
    fn default() -> Self {
        Self {
            force: Vector2::new(0.0, 0.0),
            old_force: Vector2::new(0.0, 0.0),
        }
    }
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Mobile;

/// Registers resources required by `atom_sources` to the ecs world.
pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Force>();
    world.register::<Mobile>();
}

pub fn spawn_walkers(world: &mut World, num_walkers: u32) {
    let mut rng = thread_rng();
    for _j in 1..num_walkers {
        world
            .create_entity()
            .with(Position::new(
                rng.gen::<f32>() * globals::WIDTH,
                rng.gen::<f32>() * globals::HEIGHT,
            ))
            .with(Velocity::new(0.0, 0.0))
            .with(Force::default())
            .with(Mobile)
            .build();
    }
    world
        .create_entity()
        .with(Position::new(globals::WIDTH / 2.0, globals::HEIGHT / 2.0))
        .with(Velocity::new(0.0, 0.0))
        .with(Force::default())
        .build();

    // one-time system, add entities to the neighborhood
    world.exec(
        |(ent, pos, mut nh): (
            Entities,
            ReadStorage<Position>,
            WriteExpect<Neighborhood>,
        )| {
            for (ent, pos) in (&ent, &pos).join() {
                let (nx, ny) = nh.get_area_xy(pos.pos);
                nh.insert(nx, ny, ent.id());
            }
        },
    );
    let mut counter = world.write_resource::<Counter>();
    counter.tot = num_walkers;
}

// Count aggregated walkers
pub struct Counter {
    pub fix: u32,
    pub tot: u32,
}

pub struct CounterSystem;

impl<'a> System<'a> for CounterSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Mobile>,
        WriteExpect<'a, Counter>,
    );

    fn run(&mut self, (ent, mobile, mut counter): Self::SystemData) {
        let fix_walkers = (&ent, !&mobile).join().count();
        counter.fix = fix_walkers as u32;
    }
}
