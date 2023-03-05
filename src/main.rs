mod atc;
mod simulate;

use crate::atc::planning::compute_landing_plan;
use crate::simulate::airport::Airport;
use crate::simulate::plane::Plane;
use lyon_geom::{Point, Vector};
use nannou::prelude::*;

struct Model {
    path: Vec<Point<i32>>
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model {
        path: generate_plane_path()
    }
}

fn event(_app: &App, _model: & mut Model, _event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    for (a, b) in model.path.iter().zip(model.path.iter().skip(1)) {
        let (a, b) = ((*a * 10).to_f32(), (*b * 10).to_f32());
        let (pt_a, pt_b) = (pt2(a.x, a.y), pt2(b.x, b.y));
        draw.line().start(pt_a).end(pt_b).weight(4.0).color(STEELBLUE);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn generate_plane_path() -> Vec<Point<i32>> {
    let airport = Airport {
        position: Point::new(10, 0),
        runway_direction: Vector::new(1, -1),
    };
    let plane = Plane {
        position: Point::new(-10, 0),
        velocity: Vector::new(10, 0),
    };
    let plan = compute_landing_plan(&plane, &airport).unwrap();
    plan.0.into_iter().map(|p| p.position).collect()
}
