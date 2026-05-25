use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{constants::LEVEL_SPEED, level::Surface};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(FixedUpdate, keys);
}

const HORIZONTAL_SPEED: f32 = 128.0;
const VERTICAL_SPEED: f32 = 64.0;
const PLAYER_LEFT_BOUNDARY: f32 = -1000.0;
const PLAYER_RIGHT_BOUNDARY: f32 = 0.0;

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
    mut transform: Single<&mut Transform, (With<Player>, Without<Surface>)>,
    buttons: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
    surfaces: Query<&Transform, With<Surface>>,
) {
    let dt = time.delta_secs();
    let old_y = transform.translation.y;

    if buttons.pressed(KeyCode::KeyW) || buttons.pressed(KeyCode::ArrowUp) {
        transform.translation.y += VERTICAL_SPEED * dt;
        for surface in surfaces {
            if transform.translation.y < surface.translation.y {
                continue;
            }
            let bounds = (surface.translation.x - surface.scale.x / 2.0)
                ..=(surface.translation.x + surface.scale.x / 2.0);
            if bounds.contains(&transform.translation.x) {
                transform.translation.y = surface.translation.y;
                break;
            }
        }
    }
    if buttons.pressed(KeyCode::KeyS) || buttons.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= VERTICAL_SPEED * dt;
        if transform.translation.y < -500.0 {
            transform.translation.y = -500.0;
        }
    }
    if buttons.pressed(KeyCode::KeyA) || buttons.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= HORIZONTAL_SPEED * dt;
        if transform.translation.x < PLAYER_LEFT_BOUNDARY {
            transform.translation.x = PLAYER_LEFT_BOUNDARY;
        }
    }
    if buttons.pressed(KeyCode::KeyD) || buttons.pressed(KeyCode::ArrowRight) {
        transform.translation.x += HORIZONTAL_SPEED * dt;
        if transform.translation.x > PLAYER_RIGHT_BOUNDARY {
            transform.translation.x = PLAYER_RIGHT_BOUNDARY;
        }
    }

    // Turn the sub up or down depending on vertical movement
    if transform.translation.y > old_y {
        transform.rotation = Quat::from_rotation_z(-PI / 2.0 + 0.05);
    } else if transform.translation.y < old_y {
        transform.rotation = Quat::from_rotation_z(-PI / 2.0 - 0.05);
    } else {
        transform.rotation = Quat::from_rotation_z(-PI / 2.0);
    }

    // Also move with the level speed
    transform.translation.x -= LEVEL_SPEED * dt;
    if transform.translation.x < PLAYER_LEFT_BOUNDARY {
        transform.translation.x = PLAYER_LEFT_BOUNDARY;
    }
}
