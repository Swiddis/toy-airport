use lyon_geom::{Point, Vector};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Plane {
    pub position: Point<i32>,
    pub velocity: Vector<i32>,
}

const MAX_SPEED: f64 = 10.0; // TODO make dynamic

impl Plane {
    pub fn astar_successors(&self) -> Vec<(Plane, i32)> {
        let accelerations: Vec<(i32, i32)> = (-1..2)
            .into_iter()
            .map(|x| (-1..2).into_iter().map(move |y| (x, y)))
            .flatten()
            .collect();
        accelerations.iter().map(|a| {
            let vel = self.velocity + Vector::new(a.0, a.1);
            let vel = match vel.to_f64().length() > MAX_SPEED {
                true => self.velocity,
                false => vel,
            };
            let pos = self.position + vel;
            (Plane {
                position: pos,
                velocity: vel,
            }, 1)
        }).collect()
    }

    pub fn astar_heuristic(&self, goal: &Plane) -> f64 {
        let dist = self.position.to_f64().distance_to(goal.position.to_f64());
        return dist / MAX_SPEED;
    }
}
