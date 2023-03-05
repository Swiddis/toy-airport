use lyon_geom::{Point, Vector};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Plane {
    pub position: Point<i32>,
    pub velocity: Vector<i32>,
}

const MAX_FLIGHT_SPEED: f64 = 10.0; // TODO make dynamic
const MIN_FLIGHT_SPEED: f64 = 2.5;

impl Plane {
    pub fn astar_successors(&self) -> Vec<(Plane, f64)> {
        let accelerations: Vec<(i32, i32)> = (-1..2)
            .into_iter()
            .map(|x| (-1..2).into_iter().map(move |y| (x, y)))
            .flatten()
            .collect();
        accelerations
            .iter()
            .filter_map(|a| {
                let vel = self.velocity + Vector::new(a.0, a.1);
                let pos = self.position + vel;
                match vel.to_f64().length() {
                    v if v < MIN_FLIGHT_SPEED => None,
                    v if v > MAX_FLIGHT_SPEED => None,
                    _ => Some((
                        Plane {
                            position: pos,
                            velocity: vel,
                        },
                        1.0,
                    )),
                }
            })
            .collect()
    }

    pub fn astar_heuristic(&self, goal: &Plane) -> f64 {
        let dist = self.position.to_f64().distance_to(goal.position.to_f64());
        return dist / MAX_FLIGHT_SPEED;
    }
}
