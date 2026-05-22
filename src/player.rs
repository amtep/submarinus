use std::f32::consts::PI;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

#[derive(Component, Clone, Default)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(Capsule2d::new(10.0, 50.0));
    let color = materials.add(Color::srgba(0.5, 0.5, 0.5, 1.0));

    commands.spawn((
        Player,
        Mesh2d(shape),
        MeshMaterial2d(color),
        Transform::from_xyz(-1000.0, 0.0, 0.0).with_rotation(Quat::from_rotation_z(PI / 2.0)),
    ));
}
