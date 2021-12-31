use crate::globals;
use crate::walker::{Force, Position, Velocity, Mobile};
use crate::neighborhood::Neighborhood;
use nalgebra::Vector2;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use specs::{ParJoin, ReadExpect, System, WriteExpect, ReadStorage, WriteStorage, Entities};
use std::sync::Mutex;

const PI: f32 = std::f32::consts::PI;

/// Tracks the number of the current integration step.
pub struct Step {
    pub n: u32,
}

pub struct Timestep {
    /// Duration of the simulation timestep
    pub delta: f32,
}

pub struct VerletUpdatePositionSystem;

impl<'a> System<'a> for VerletUpdatePositionSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Timestep>,
        WriteExpect<'a, Step>,
        ReadStorage<'a, Mobile>,
        WriteStorage<'a, Force>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
        WriteExpect<'a, Neighborhood>,
    );

    fn run(&mut self, (ents, t, mut step, mobile, mut force, mut velocity, mut position, neighborhood): Self::SystemData) {
        step.n += 1;

        let nh = Mutex::new(neighborhood);

        (&ents, &mut force, &mut velocity, &mut position, &mobile)
            .par_join()
            .for_each(|(ent, force, vel, pos, _)| {
                let mut nh = nh.lock().unwrap();
                
                // check nh index before updating
                let (old_nx, old_ny) = nh.get_area_xy(pos.pos);
                
                verlet_integrator(force, vel, pos, t.delta);
                pos.pos[0] = modulus(pos.pos[0], globals::WIDTH);
                pos.pos[1] = modulus(pos.pos[1], globals::HEIGHT);

                let (new_nx, new_ny) = nh.get_area_xy(pos.pos);
				if old_nx != new_nx || old_ny != new_ny {
					nh.remove(old_nx, old_ny, ent.id());
					nh.insert(new_nx, new_ny, ent.id());
				}
            });
    }
}

fn verlet_integrator(force: &mut Force, vel: &mut Velocity, pos: &mut Position, dt: f32) {
    // Velocity Verlet
    pos.pos += vel.vel * dt + force.force / 2.0 * dt * dt;
    force.old_force = force.force;
    force.force = random_vec2(globals::MAX_FORCE);
    vel.vel += (force.old_force + force.force) / 2.0 * dt;
}

fn random_vec2(mag: f32) -> Vector2<f32> {
    let mut rng = thread_rng();
    let angle = rng.gen::<f32>() * 2.0 * PI;
    Vector2::new(angle.cos() * mag, angle.sin() * mag)
}

fn modulus(a: f32, b: f32) -> f32 {
    // https://stackoverflow.com/q/31210357
    ((a % b) + b) % b
}
