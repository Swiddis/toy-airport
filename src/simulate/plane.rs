use lyon_geom::{Point, Vector};
use std::cmp::max;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Plane {
    pub position: Point<i32>,
    pub velocity: Vector<i32>,
}

const MAX_FLIGHT_SPEED: f64 = 10.0; // TODO make dynamic
const MIN_FLIGHT_SPEED: f64 = 2.5;

// Find time for current state to reach goal state.
// 1-dimension kinematics, with max acceleration of 1.
fn min_steps(curr_pos: i32, curr_vel: i32, goal_pos: i32, _goal_vel: i32) -> usize {
    // Begin with inverse triangle to see needed velocity for optimal solve
    let pos_change = (curr_pos - goal_pos).abs() as f64;
    let pos_change_steps = (((8.0 * pos_change + 1.0).sqrt() - 1.0) * 0.5).ceil() as i32;
    let goal_vel = if curr_pos > goal_pos { -pos_change_steps } else { pos_change_steps };
    // We at least need to reach goal vel
    let vel_change_steps = (curr_vel - goal_vel).abs() as usize;
    vel_change_steps + (pos_change_steps as usize)
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
        // so we solve each axis and take the worst one.
        max(
            min_steps(
                self.position.x,
                self.velocity.x,
                goal.position.x,
                goal.velocity.x,
            ),
            min_steps(
                self.position.y,
                self.velocity.y,
                goal.position.y,
                goal.velocity.y,
            ),
        )
    }
}
