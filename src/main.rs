mod components;

use crate::components::plane::Plane;
use rand::prelude::*;

const _SCALE: f32 = 10.0;

fn main() {
    let mut rng = rand::thread_rng();
    let (p_x, p_y, v_x, v_y): (f32, f32, f32, f32) = rng.gen();
    let plane = Plane::new(
        p_x * 20.0 - 10.0,
        p_y * 20.0 - 10.0,
        v_x * 10.0 - 5.0,
        v_y * 10.0 - 5.0,
    );
    println!("{plane:#?}");
}
