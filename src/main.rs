mod atc;
mod simulate;

use crate::atc::planning::compute_landing_plan;
use crate::simulate::airport::Airport;
use crate::simulate::plane::Plane;
use lyon_geom::{Point, Vector};

fn main() {
    let airport = Airport {
        position: Point::new(0, 0),
    };
    let mut plane = Plane {
        position: Point::new(-5, 0),
        velocity: Vector::new(2, 1),
    };
    let plan = compute_landing_plan(&plane, &airport);
    for step in plan {
        plane.accelerate(step.x, step.y);
        plane.tick();
        println!("{:?}", plane);
        println!(
            "Distance: {}",
            plane
                .position
                .to_f64()
                .distance_to(airport.position.to_f64())
        );
    }
}
