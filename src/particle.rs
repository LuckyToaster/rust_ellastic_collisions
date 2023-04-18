#![allow(dead_code)]
use getset::{Getters, MutGetters, Setters};
use ggez::graphics::Color;
use nalgebra::Vector2;

#[derive(Debug, Clone, PartialEq, Getters, MutGetters, Setters)]
pub struct Particle{
    pub p: Vector2<f32>, // position
    pub v: Vector2<f32>, // velocity
    pub a: Vector2<f32>, // acceleration
    pub r: f32,          // radius
    pub m: f32,          // mass
    pub c: Color,          // color
}

impl Particle {
    pub fn new(p: Vector2<f32>, v: Vector2<f32>, a: Vector2<f32>, r: f32, m: f32, c: Color) -> Self {
        return Particle{p, v, a, r, m, c};
    }

    pub fn update(&mut self, dt: f32) {
        self.v.x += self.a.x * dt * 0.98;
        self.v.y += self.a.y * dt * 0.98;
	self.p.x += self.v.x * dt * 0.98;
	self.p.y += self.v.y * dt * 0.98; 
    }

    fn solve_width_overlap(&mut self, w: i32) {
        let right_overlap = self.p.x + self.r - w as f32;
        let left_overlap = self.r - self.p.x;
        self.p.x = if right_overlap > 0.0 {self.p.x - right_overlap} else {self.p.x + left_overlap}
    }

    fn solve_height_overlap(&mut self, h: i32) {
        let down_overlap = self.p.y + self.r - h as f32;
        let up_overlap = self.r - self.p.y;
        self.p.y = if down_overlap > 0.0 {self.p.y - down_overlap} else {self.p.y + up_overlap}
    }

    pub fn solve_edge_collision(&mut self, w: i32, h: i32) {
        if self.p.x - self.r <= 0.0 || self.p.x + self.r >= w as f32 {
            self.solve_width_overlap(w);
            self.v.x = -self.v.x;
        }
        if self.p.y - self.r <= 0.0 || self.p.y + self.r >= h as f32 {
            self.solve_height_overlap(h);
            self.v.y = -self.v.y;
        }
    }
}

impl ToString for Particle {
    fn to_string(&self) -> String {
        format!("Particle:\n  pos({}, {})\n  vel({}, {})\n  acc({}, {})\n  radius: {}\n  mass: {}\n", 
            self.p.x, self.p.y, self.v.x, self.v.y, self.a.x, self.a.y, self.r, self.m)
    }
}
