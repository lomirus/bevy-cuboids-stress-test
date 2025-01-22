use std::f32::consts::PI;

use bevy::{dev_tools::fps_overlay::FpsOverlayPlugin, prelude::*};

const HALF_SIDE_LENGTH: i32 = 125;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight::default());

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, -(HALF_SIDE_LENGTH as f32) / (PI / 8.0).tan())
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let mesh = meshes.add(Cuboid::from_length(0.5));
    let material = materials.add(Color::srgb(1.0, 1.0, 1.0));

    for x in -HALF_SIDE_LENGTH..=HALF_SIDE_LENGTH {
        for y in -HALF_SIDE_LENGTH..=HALF_SIDE_LENGTH {
            commands.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(x as f32, y as f32, 0.0),
            ));
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FpsOverlayPlugin::default()))
        .add_systems(Startup, setup)
        .run();
}
