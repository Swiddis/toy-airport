use lyon_geom::{Point, Vector};

use super::airport::Airport;
use super::airport::Runway;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Plane {
    pub position: Point<i32>,
    pub velocity: Vector<i32>,
}

const MAX_FLIGHT_SPEED: f64 = 10.0; // TODO make dynamic
const MIN_FLIGHT_SPEED: f64 = 3.0;
pub const LANDING_SPEED: f64 = 4.0;

fn runway_heuristic(plane: &Plane, runway: &Runway) -> usize {
    let lv = runway.landing_vector(LANDING_SPEED);
    let dp = (runway.position - plane.position).abs().to_usize();
    let dv = (lv - plane.velocity).abs().to_usize();
    dp.x + dp.y + dv.x + dv.y
}

impl Plane {
    pub fn astar_successors(&self) -> Vec<(Self, usize)> {
        let accelerations: Vec<(i32, i32)> =
            (-1..2).flat_map(|x| (-1..2).map(move |y| (x, y))).collect();
        accelerations
            .iter()
            .filter_map(|a| {
                let vel = self.velocity + Vector::new(a.0, a.1);
                let pos = self.position + vel;
                match vel.to_f64().length() {
                    v if v < MIN_FLIGHT_SPEED => None,
                    v if v > MAX_FLIGHT_SPEED => None,
                    _ => Some((
                        Self {
                            position: pos,
                            velocity: vel,
                        },
                        1,
                    )),
                }
            })
            .collect()
    }

    // Non-admissable, we don't actually need an optimal path.
    // We want to optimize for solution speed.
    pub fn astar_heuristic(&self, goal: &Airport) -> usize {
        goal.runways
            .iter()
            .map(|r| runway_heuristic(self, r))
            .min()
            .unwrap()
    }
}
