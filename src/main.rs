#![allow(unused_imports)]
mod particle;
#[cfg(test)]
mod tests; // tell the compiler to exclude tests from executable
mod util;

use nalgebra::Vector2;
use particle::Particle;
use std::time::Duration;
use util::{r, rcolor, rrng, v, solve_overlap, collide, solve_collision};
use ggez::{
    conf::{Backend, Conf, WindowMode, WindowSetup},
    event::{run, EventHandler},
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Text},
    timer,
    mint::Point2,
    Context, ContextBuilder, GameError, GameResult,
};

const W: f32 = 1200.0;
const H: f32 = 600.0;

struct State {
    dt: f32,
    particles: Vec<Particle>,
}

impl State {
    fn new(dt: f32, p_n: usize, max_r: Vector2<f32>, max_v: Vector2<f32>, acc: Vector2<f32>) -> Self {
        let mut particles: Vec<Particle> = Vec::with_capacity(p_n);
        for _i in 0..p_n {
            let radius = rrng(max_r.x, max_r.y);
            let pos = v(rrng(0.0 + radius, W - radius), rrng(0.0 + radius, H - radius));
            let vel = v(r(max_v.x), r(max_v.y));
            let mass = if acc.y > 0.0 { 10.0 * radius * acc.y } else { 10.0 * radius };
            particles.push(Particle::new(pos, vel, acc, radius, mass, rcolor()));
        } 
        return State {dt, particles};
    }
}

impl EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta().as_secs_f32();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        for i in 0 .. self.particles.len() {
            self.particles[i].update(self.dt); // TODO: get real fps as use the delta time of that 
            self.particles[i].solve_edge_collision(W as i32, H as i32);

            for j in 0 .. self.particles.len() {
                if i != j && collide(&self.particles[i], &self.particles[j]) {
                    let r1 = &mut self.particles[i] as *mut Particle;
                    let r2 = &mut self.particles[j] as *mut Particle;
                    unsafe {    
                        solve_overlap(&mut *r1, &mut *r2);
                        solve_collision(&mut *r1, &mut *r2);  
                    }
                }
            }

            let circle = Mesh::new_circle(ctx, DrawMode::fill(), 
                Point2 {x: self.particles[i].p.x, y: self.particles[i].p.y}, 
                self.particles[i].r,
                0.1,
                self.particles[i].c,
            )?;
            canvas.draw(&circle, DrawParam::default());
        }

        #[allow(deprecated)]
        let fps_text = Text::new(format!("FPS: {:.2}", timer::fps(ctx)));
        canvas.draw(&fps_text, DrawParam::new().dest([10.0, 10.0]));

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() {
    // fps, particle_n, radius_range, velocity_range, acceleration_range
    //let state = State::new(120.0, 50, v(5.0, 10.0), v(500.0, 500.0), v(0.0, 0.0));
    let state = State::new(120.0, 10, v(10.0, 10.0), v(500.0, 500.0), v(0.0, 2000.0));

    let c = Conf {
        window_mode: WindowMode::default().dimensions(W, H).resizable(false),
        window_setup: WindowSetup::default(), //.vsync(true),
        backend: Backend::default(),
    }; // or use Conf::new();

    let (ctx, event_loop) = ContextBuilder::new("hello ggez", "lucky toaster")
        .default_conf(c).build().unwrap();

    run(ctx, event_loop, state);
}

