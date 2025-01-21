use std::f32::consts::PI;

use bevy::{dev_tools::fps_overlay::FpsOverlayPlugin, prelude::*};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight::default());

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, -50.0 / (PI / 8.0).tan()).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    for x in -50..=50 {
        for y in -50..=50 {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::from_length(0.5))),
                MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
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