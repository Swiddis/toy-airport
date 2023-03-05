mod atc;
mod simulate;

use crate::atc::planning::compute_landing_plan;
use crate::simulate::airport::Airport;
use crate::simulate::plane::Plane;
use lyon_geom::{Point, Vector};

fn main() {
    let airport = Airport {
        position: Point::new(100, 0),
        runway_direction: Vector::new(2, -1),
    };
    let plane = Plane {
        position: Point::new(-100, 0),
        velocity: Vector::new(0, 10),
    };
    let plan = compute_landing_plan(&plane, &airport).unwrap();
    println!(
        "Positions: {:?}",
        plan.0
            .iter()
            .map(|p| (p.position.x, p.position.y))
            .collect::<Vec<(i32, i32)>>()
    )
}
