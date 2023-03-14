use pathfinding::directed::astar::astar;

use crate::simulate::airport::Airport;
use crate::simulate::plane::Plane;
use crate::simulate::plane::LANDING_SPEED;

pub fn compute_landing_plan(plane: &Plane, airport: &Airport) -> Option<(Vec<Plane>, usize)> {
    astar(
        plane,
        |p| {
            let succ = p.astar_successors();
            succ.into_iter()
                .map(|(p, c)| (p, c))
                .collect::<Vec<(Plane, usize)>>()
        },
        |p| p.astar_heuristic(airport),
        |p| {
            airport
                .runways
                .iter()
                .any(|r| p.position == r.position && p.velocity == r.landing_vector(LANDING_SPEED))
        },
    )
}
