//!https://github.com/prixt/ggez-ecs-boids/blob/master/src/main.rs
//! Basics of an ecs game:
//! the graphics loop runs all the systems that handle the game,
//! and calls a "Render" system in the end.
#![allow(dead_code)]

use ggez::{conf, event, graphics, Context, ContextBuilder, GameError, GameResult};
use rand::{thread_rng, Rng};
use specs::prelude::*;

mod ecs;
mod globals;
mod integrator;
mod aggregate;
mod output;
mod render;
mod walker;

use render::{RenderSystem, FrameRateLoggerSystem};
use walker::{Force, Mobile, Position, Velocity};

const SCREEN_WIDTH: f32 = globals::WIDTH;
const SCREEN_HEIGHT: f32 = globals::HEIGHT;

struct State {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}
impl State {
    fn new(_ctx: &mut Context) -> GameResult<Self> {
        let mut world = World::new();
        ecs::register_components(&mut world);
        ecs::register_resources(&mut world);

        let mut dispatcher = ecs::create_dispatcher();
        dispatcher.setup(&mut world);
        let mut rng = thread_rng();
        for _j in 1..globals::NUM_WALKERS {
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
            .with(Position::new(
                globals::WIDTH / 2.0,
                globals::HEIGHT / 2.0,
            ))
            .with(Velocity::new(0.0, 0.0))
            .with(Force::default())
            .build();


        Ok(Self { world, dispatcher })
    }
}

impl event::EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Sync the timestep with graphics frame rate
        // let dt = ggez::timer::delta(ctx);
        // let dt = ggez::timer::duration_to_f64(dt) as f32;
        // self.world.insert(Timestep { delta: dt });
        self.dispatcher.dispatch(&mut self.world);
        self.world.maintain();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        RenderSystem { ctx }.run_now(&self.world);
        FrameRateLoggerSystem { ctx }.run_now(&self.world);
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ContextBuilder::new("DLA", "ggez")
        .window_setup(conf::WindowSetup::default().title("Dffusion Limited Aggregation"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));
    // .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let state = State::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
