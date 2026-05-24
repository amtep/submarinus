use std::f32::consts::PI;

use bevy::prelude::*;

use crate::constants::LEVEL_SPEED_PX_PER_SEC;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(FixedUpdate, keys);
}

const HORIZONTAL_SPEED: f32 = 128.0;
const VERTICAL_SPEED: f32 = 64.0;

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
        Transform::from_xyz(-1000.0, 0.0, 0.0).with_rotation(Quat::from_rotation_z(-PI / 2.0)),
    ));
}

fn keys(
    mut transform: Single<&mut Transform, With<Player>>,
    buttons: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {
    let dt = time.delta_secs();

    if buttons.pressed(KeyCode::KeyW) || buttons.pressed(KeyCode::ArrowUp) {
        transform.translation.y += VERTICAL_SPEED * dt;
        if transform.translation.y > 500.0 {
            transform.translation.y = 500.0;
        } else {
            transform.rotation = Quat::from_rotation_z(-PI / 2.0 + 0.05);
        }
    }
    if buttons.pressed(KeyCode::KeyS) || buttons.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= VERTICAL_SPEED * dt;
        if transform.translation.y < -500.0 {
            transform.translation.y = -500.0;
        } else {
            transform.rotation = Quat::from_rotation_z(-PI / 2.0 - 0.05);
        }
    }
    if buttons.pressed(KeyCode::KeyA) || buttons.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= HORIZONTAL_SPEED * dt;
        if transform.translation.x < -1000.0 {
            transform.translation.x = -1000.0;
        } else {
            transform.rotation = Quat::from_rotation_z(-PI / 2.0);
        }
    }
    if buttons.pressed(KeyCode::KeyD) || buttons.pressed(KeyCode::ArrowRight) {
        transform.translation.x += HORIZONTAL_SPEED * dt;
        if transform.translation.x > 0.0 {
            transform.translation.x = 0.0;
        } else {
            transform.rotation = Quat::from_rotation_z(-PI / 2.0);
        }
    }

    // Also move with the level speed
    transform.translation.x -= LEVEL_SPEED_PX_PER_SEC * dt;
    if transform.translation.x < -1000.0 {
        transform.translation.x = -1000.0;
    }
}
