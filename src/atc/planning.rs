use pathfinding::directed::astar::astar;

use crate::simulate::airport::Airport;
use crate::simulate::plane::Plane;

pub fn compute_landing_plan(plane: &Plane, airport: &Airport) -> Option<(Vec<Plane>, i32)> {
    let goal = Plane {
        position: airport.position,
        velocity: airport.runway_direction * 3,
    };
    astar(
        plane,
        |p| p.astar_successors(),
        |p| p.astar_heuristic(&goal) as i32,
        |p| p.position == goal.position && p.velocity == goal.velocity,
    )
}
