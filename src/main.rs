mod atc;
mod simulate;

use std::collections::HashMap;

use crate::atc::planning::compute_landing_plan;
use crate::simulate::airport::Airport;
use crate::simulate::plane::Plane;
use atc::planning::LANDING_SPEED;
use lyon_geom::{Point, Vector};
use nannou::geom::*;
use nannou::prelude::*;

const SCALE: f32 = 10.0;

struct Model {
    airport: Airport,
    steps: Vec<Plane>,
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let airport = Airport {
        position: Point::new(10, 0),
        runway_direction: Vector::new(1, -1),
    };
    let plane = Plane {
        position: Point::new(-10, 0),
        velocity: Vector::new(10, 0),
    };
    Model {
        airport: airport.clone(),
        steps: generate_plane_steps(&airport, &plane),
    }
}

impl Model {
    fn average_velocities(&self) -> Option<HashMap<&Plane, Vector<f32>>> {
        if self.steps.len() < 2 {
            return None;
        }
        let mut map = HashMap::new();
        map.insert(&self.steps[0], self.steps[1].velocity.to_f32());
        map.insert(
            self.steps.last().unwrap(),
            self.steps.last().unwrap().velocity.to_f32(),
        );
        for (i, plane) in self.steps.iter().enumerate().skip(1) {
            if i == self.steps.len() - 1 {
                continue;
            }
            let (prev, next) = (&self.steps[i - 1], &self.steps[i + 1]);
            let (v1, v2) = (
                plane.position - prev.position,
                next.position - plane.position,
            );
            map.insert(plane, (v1.to_f32() + v2.to_f32()) * 0.5);
        }
        Some(map)
    }

    fn as_smooth_path(&self) -> Path {
        let start = self.steps[0].position.to_f32() * SCALE;
        let avg_vel = self.average_velocities().unwrap();
        let mut path = path().begin(vec2(start.x, start.y));
        for (p1, p2) in self.steps.iter().zip(self.steps.iter().skip(1)) {
            let (c1, c2, to) = (
                p1.position.to_f32() * SCALE + avg_vel[p1] * SCALE / 3.0,
                p2.position.to_f32() * SCALE - avg_vel[p2] * SCALE / 3.0,
                p2.position.to_f32() * SCALE,
            );
            let (c1, c2, to) = (pt2(c1.x, c1.y), pt2(c2.x, c2.y), pt2(to.x, to.y));
            path = path.cubic_bezier_to(c1, c2, to);
        }
        // One more straight line for landing
        let end = self.steps.last().unwrap();
        let end_pos = (end.position + end.velocity).to_f32() * SCALE;
        path = path.line_to(pt2(end_pos.x, end_pos.y));
        path.inner_mut().end(false);
        path.build()
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let dir = model.airport.runway_direction.to_f32();
    let pos = model.airport.position.to_f32() + dir.normalize() * (LANDING_SPEED as f32) * 0.5;
    // Runway
    draw.rect()
        .width(8.0)
        .height(2.0 * (LANDING_SPEED as f32) * SCALE)
        .z_radians(-dir.angle_from_x_axis().radians)
        .xy(vec2(pos.x, pos.y) * SCALE);
    // Flight path
    let path = model.as_smooth_path();
    draw.path()
        .stroke()
        .weight(4.0)
        .color(STEELBLUE)
        .events(path.iter());
    draw.to_frame(app, &frame).unwrap();
}

fn generate_plane_steps(airport: &Airport, plane: &Plane) -> Vec<Plane> {
    let plan = compute_landing_plan(plane, airport).unwrap();
    plan.0
}
