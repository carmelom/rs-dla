/// Render partcles using ggez
/// inspired by
/// https://github.com/prixt/ggez-ecs-boids/blob/master/src/main.rs
extern crate dla as lib;
extern crate ggez;
use ggez::{conf, event, graphics, Context, ContextBuilder, GameResult, GameError};
use specs::prelude::*;

use lib::ecs;
use lib::integrator::Timestep;
use lib::walker::{Position, Velocity};
use lib::render::RenderSystem;
use lib::globals;

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

        world
            .create_entity()
            .with(Position::new(300.0, 300.0))
            .with(Velocity::new(100.0, 0.0))
            .build();
        // world
        //     .create_entity()
        //     .with(Position::new(-1.2, 1.1))
        //     .with(Velocity::new(-1.0, 0.0))
        //     .build();

        world.insert(Timestep { delta: 0.05 });
        
        Ok(Self{world, dispatcher})
    }
}

impl event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx);
        let dt = ggez::timer::duration_to_f64(dt) as f32;
        self.world.insert(Timestep { delta: dt });
        // let &mut timestep = self.world.write_resource::<Timestep>();
        // timestep.delta = dt;
        self.dispatcher.dispatch(&mut self.world);
        self.world.maintain();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        RenderSystem{ctx}.run_now(&self.world);
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ContextBuilder::new("DLA", "ggez")
        .window_setup(
            conf::WindowSetup::default()
                .title("ggez + specs")
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        );
        // .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let state = State::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
