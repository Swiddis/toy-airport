use std::collections::{HashMap, HashSet};
use std::error::Error;

use lyon_geom::Vector;

use crate::simulate::airport::Airport;
use crate::simulate::plane::Plane;

fn accelerations() -> Vec<Vector<i32>> {
    (-1..2)
        .into_iter()
        .map(|x| (-1..2).into_iter().map(move |y| Vector::new(x, y)))
        .flatten()
        .collect()
}

pub fn compute_landing_plan(plane: &Plane, airport: &Airport) -> Vec<Vector<i32>> {
    let accelerations = accelerations();
    let goal_state = Plane {
        position: airport.position,
        velocity: Vector::new(0, 0),
    };
    unimplemented!()
}
