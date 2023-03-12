use lyon_geom::{Point, Vector};
use std::cmp::max;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Plane {
    pub position: Point<i32>,
    pub velocity: Vector<i32>,
}

const MAX_FLIGHT_SPEED: f64 = 10.0; // TODO make dynamic
const MIN_FLIGHT_SPEED: f64 = 2.5;

fn single_axis_heuristic(curr_pos: i32, curr_vel: i32, goal_pos: i32, goal_vel: i32) -> usize {
    // We assume a "suicide burn" strategy, from astronautics.
    // Assume we accelerate as hard as possible to the goal velocity, where do we land?
    let d_vel = goal_vel - curr_vel;
    // Position change depends on triangle number for delta
    let pos = curr_pos
        + if d_vel < 0 {
            (-d_vel - 2 * curr_vel + 1) * d_vel / 2
        } else {
            (d_vel + 2 * curr_vel + 1) * d_vel / 2
        };
    // How far we are from the goal at the end of the change determines our error.
    let d_pos = (pos - goal_pos).abs();
    // For now, error is our heuristic
    d_pos as usize
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
    pub fn astar_heuristic(&self, goal: &Plane) -> usize {
        // Each axis can be controlled independently,
        // so we solve each axis and merge them.
        let h_x = single_axis_heuristic(
            self.position.x,
            self.velocity.x,
            goal.position.x,
            goal.velocity.x,
        );
        let h_y = single_axis_heuristic(
            self.position.y,
            self.velocity.y,
            goal.position.y,
            goal.velocity.y,
        );
        // Current implementation of axis heuristics is an error from the source.
        // For now, we use the error magnitude as the real heuristic.
        let dist = ((h_x * h_x + h_y * h_y) as f64).sqrt();
        dist.ceil() as usize
    }
}
