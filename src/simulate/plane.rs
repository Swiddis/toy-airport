use lyon_geom::{Point, Vector};

#[derive(Debug)]
pub struct Plane {
    pub position: Point<f32>,
    pub velocity: Vector<f32>,
    pub max_speed: f32,
    pub min_speed: f32,
}

const DEFAULT_MAX_SPEED: f32 = 10.0;
const DEFAULT_MIN_SPEED: f32 = 3.0;

impl Plane {
    pub fn new(p_x: f32, p_y: f32, v_x: f32, v_y: f32) -> Self {
        let mut velocity = Vector::new(v_x, v_y);
        if velocity.length() >= DEFAULT_MAX_SPEED {
            velocity *= DEFAULT_MAX_SPEED / velocity.length();
        } else if velocity.length() <= DEFAULT_MIN_SPEED {
            velocity *= DEFAULT_MIN_SPEED / velocity.length();
        }
        Self {
            position: Point::new(p_x, p_y),
            velocity: Vector::new(v_x, v_y),
            max_speed: 10.0,
            min_speed: 3.0,
        }
    }
}
