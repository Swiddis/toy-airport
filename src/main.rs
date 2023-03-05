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
    let plane = Plane {
        position: Point::new(-5, 0),
        velocity: Vector::new(0, 5),
    };
    let plan = compute_landing_plan(&plane, &airport).unwrap();
    for step in plan.0 {
        println!("{:?}", step);
        println!(
            "Distance: {}",
            step
                .position
                .to_f64()
                .distance_to(airport.position.to_f64())
        );
    }
}
