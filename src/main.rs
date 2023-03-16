mod components;

use crate::components::plane::Plane;
use lyon_geom::Vector;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let (p_x, p_y, v_x, v_y): (f32, f32, f32, f32) = rng.gen();
    let mut plane = Plane::new(
        (p_x * 20.0 - 10.0, p_y * 20.0 - 10.0).into(),
        (v_x * 10.0 - 5.0, v_y * 10.0 - 5.0).into(),
    );
    for _ in 0..100 {
        let pos_vec: Vector<f32> = plane.pos.to_vector();
        let accel = -pos_vec.normalize();
        plane.accelerate(accel);
        plane.tick(1.0);
        println!("{plane:?}");
    }
}
