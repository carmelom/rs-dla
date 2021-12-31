use specs::{Component, VecStorage, NullStorage, World, WorldExt};
use nalgebra::Vector2;
use std::fmt;

// Position
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub pos: Vector2<f32>
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
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub vel: Vector2<f32>
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
        Self {force: Vector2::new(0.0, 0.0), old_force: Vector2::new(0.0, 0.0)}
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
