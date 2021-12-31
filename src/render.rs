use crate::globals;
use crate::walker::{Mobile, Position};
use crate::integrator::Step;
use ggez::{graphics::*, timer, Context};
use glam::Vec2;
use rayon::prelude::*;
use specs::{ReadStorage, System, ReadExpect, WriteExpect, ParJoin};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};


pub struct RenderSystem<'draw> {
    pub ctx: &'draw mut Context,
}
impl<'draw, 'world> System<'world> for RenderSystem<'draw> {
    type SystemData = (ReadStorage<'world, Position>, ReadStorage<'world, Mobile>);

    fn run(&mut self, (position, mobile): Self::SystemData) {


        // Make it thread-safe for par_join
        // https://stackoverflow.com/a/30560208
        let mesh = Arc::new(Mutex::new(MeshBuilder::new()));
        (&position, !&mobile).par_join().for_each(|(pos, _)| {
            draw_circle(&mesh, pos, Color::RED);
        });
        if globals::RENDER_ALL {
            (&position, &mobile).par_join().for_each(|(pos, _)| {
                draw_circle(&mesh, pos, Color::WHITE);
            });
        }
        let mesh = mesh.lock().unwrap();
        let mesh = mesh.build(self.ctx).unwrap();
        draw(self.ctx, &mesh, (Vec2::new(0.0, 0.0), 0.0, Color::WHITE)).unwrap();
    }
}

fn draw_circle(mesh: &Arc<Mutex<MeshBuilder>>, pos: &Position, color: Color) {
    mesh.lock()
        .unwrap()
        .circle(
            DrawMode::fill(),
            Vec2::new(pos.pos[0], pos.pos[1]),
            globals::RADIUS,
            0.01,
            color,
        )
        .unwrap();
}

// fn draw_circle_with_border(mesh: &Arc<Mutex<MeshBuilder>>, pos: &Position, color: Color) {
//     let (r, g, b) = color.to_rgb();
//     let color_alpha = Color::from_rgba(r, g, b, 128);
//     mesh.lock().unwrap().circle(
//         DrawMode::fill(),
//         Vec2::new(pos.pos[0], pos.pos[1]),
//         globals::RADIUS,
//         0.01,
//         color_alpha,
//     )
//     .unwrap();
//     mesh.lock().unwrap().circle(
//         DrawMode::stroke(globals::BORDER),
//         Vec2::new(pos.pos[0], pos.pos[1]),
//         globals::RADIUS,
//         0.01,
//         color_alpha,
//     )
//     .unwrap();
// }

/// Log the framerate as a way to measure performance
pub struct FrameRate {
    pub rates: VecDeque<f32>,
}
impl FrameRate {
    pub fn new() -> Self {
        Self {
            rates: VecDeque::new(),
        }
    }
}

pub struct FrameRateLoggerSystem<'draw> {
    pub ctx: &'draw mut Context,
}
impl<'draw, 'world> System<'world> for FrameRateLoggerSystem<'draw> {
    type SystemData = (
        ReadExpect<'world, Step>,
        WriteExpect<'world, FrameRate>,
    );

    fn run(&mut self, (step, mut fr): Self::SystemData) {
        let dt = timer::delta(&self.ctx);
        let dt = timer::duration_to_f64(dt) as f32;
        fr.rates.push_back(1.0 / dt);
        if fr.rates.len() > 50 {
            fr.rates.pop_front();
        }
        if step.n % 100 == 0 {
            let avg_fr: f32 = fr.rates.iter().sum::<f32>() / (fr.rates.len() as f32);
            println!("Average frame rate {:?}", avg_fr);
        }
    }
}
