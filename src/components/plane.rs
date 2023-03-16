use bevy::prelude::Component;
use lyon_geom::{Point, Vector};

#[derive(Component, Debug)]
pub struct Plane {
    pub pos: Point<f32>,
    pub vel: Vector<f32>,
    acc: Vector<f32>,
}

impl Plane {
    pub fn new(position: Point<f32>, velocity: Vector<f32>) -> Self {
        Self {
            pos: position,
            vel: velocity,
            acc: (0.0, 0.0).into(),
        }
    }

    pub fn accelerate(&mut self, acceleration: Vector<f32>) {
        self.acc = acceleration;
    }

    pub fn tick(&mut self, time: f32) {
        self.pos += self.vel * time + self.acc * time * time;
        self.vel += self.acc * time;
    }
}
