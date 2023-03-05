mod atc;
mod simulate;

use crate::atc::planning::compute_landing_plan;
use crate::simulate::airport::Airport;
use crate::simulate::plane::Plane;
use lyon_geom::{Point, Vector};

fn main() {
    let path = generate_plane_path();
    println!("Landed in {} steps", path.len());
    println!(
        "Steps: {:?}",
        path.iter().map(|p| (p.x, p.y)).collect::<Vec<(i32, i32)>>()
    )
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
