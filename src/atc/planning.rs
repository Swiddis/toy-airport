use pathfinding::directed::astar::astar;

use crate::simulate::airport::Airport;
use crate::simulate::plane::Plane;

pub const LANDING_SPEED: f64 = 3.0;

pub fn compute_landing_plan(plane: &Plane, airport: &Airport) -> Option<(Vec<Plane>, usize)> {
    let goal = Plane {
        position: airport.position,
        velocity: (airport.runway_direction.to_f64().normalize() * LANDING_SPEED)
            .round()
            .to_i32(),
    };
    astar(
        plane,
        |p| {
            let succ = p.astar_successors();
            succ.into_iter()
                .map(|(p, c)| (p, c))
                .collect::<Vec<(Plane, usize)>>()
        },
        |p| p.astar_heuristic(&goal),
        |p| p.position == goal.position && p.velocity == goal.velocity,
    )
}
