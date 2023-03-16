mod components;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use components::plane::*;
use lyon_geom::Vector;
use rand::prelude::*;

struct PlanePlugin;

impl PlanePlugin {
    const SCALE: f32 = 20.0;

    fn spawn_planes(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let mut rng = rand::thread_rng();
        for _ in 0..5 {
            let (p_x, p_y, v_x, v_y): (f32, f32, f32, f32) = rng.gen();
            let plane = Plane::new(
                (p_x * 20.0 - 10.0, p_y * 20.0 - 10.0).into(),
                (v_x * 6.0 - 3.0, v_y * 6.0 - 3.0).into(),
            );
            let mesh_bundle = MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(
                    (plane.pos * Self::SCALE).to_3d().to_tuple().into(),
                ),
                ..default()
            };
            commands.spawn((plane, mesh_bundle));
        }
        commands.spawn(Camera2dBundle::default());
    }

    fn tick_planes(time: Res<Time>, mut query: Query<(&mut Plane, &mut Transform)>) {
        for (mut plane, mut transform) in &mut query {
            let pos_vec: Vector<f32> = plane.pos.to_vector();
            let accel = -pos_vec.normalize();
            plane.accelerate(accel);
            plane.tick(time.delta().as_secs_f32());
            transform.translation = (plane.pos * Self::SCALE).to_3d().to_tuple().into();
        }
    }
}

impl Plugin for PlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(PlanePlugin::spawn_planes)
            .add_system(PlanePlugin::tick_planes);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlanePlugin)
        .run();
}
