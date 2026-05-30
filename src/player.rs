use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use crate::{
    colliders::{COLLIDE_LAYER_ENEMY, COLLIDE_LAYER_TERRAIN, Collided, Collider},
    constants::{LEVEL_SPEED, SHOOT_COOLDOWN_SECS},
    level::Surface,
    torpedoes::launch_torpedo,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(FixedUpdate, (keys, shoot, show_lives));
}

const HORIZONTAL_SPEED: f32 = 128.0;
const VERTICAL_SPEED: f32 = 64.0;
const PLAYER_LEFT_BOUNDARY: f32 = -1000.0;
const PLAYER_RIGHT_BOUNDARY: f32 = 0.0;
const PLAYER_INNER_WIDTH: f32 = 50.0;
const PLAYER_CAPSULE_RADIUS: f32 = 10.0;

#[derive(Resource, Clone, Deref, DerefMut)]
struct ShootCooldown(Timer);

/// Marker struct for the player submarine
#[derive(Component, Clone, Default)]
struct Player;

#[derive(Resource)]
struct PlayerHandles {
    player_mesh: Handle<Mesh>,
    player_material: Handle<ColorMaterial>,
}

#[derive(Resource, Clone, Copy, Deref, DerefMut)]
struct Lives(u8);

/// Marker for displayed number of lives.
/// Contains the 1-based index of which life it's representing.
#[derive(Component, Clone, Copy, Deref, DerefMut)]
struct Life(u8);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_mesh = meshes.add(Capsule2d::new(PLAYER_CAPSULE_RADIUS, PLAYER_INNER_WIDTH));
    let player_material = materials.add(Color::srgba(0.5, 0.5, 0.5, 1.0));

    commands
        .spawn((
            Player,
            Collider(COLLIDE_LAYER_ENEMY | COLLIDE_LAYER_TERRAIN),
            Mesh2d(player_mesh.clone()),
            MeshMaterial2d(player_material.clone()),
            Transform::from_xyz(-1000.0, 0.0, 0.0).with_rotation(Quat::from_rotation_z(-FRAC_PI_2)),
        ))
        .observe(
            |ev: On<Collided>, mut commands: Commands, mut lives: ResMut<Lives>| {
                // TODO: hit animation and temporary invulnerability
                if **lives > 0 {
                    **lives -= 1;
                } else {
                    // TODO: game over
                }
                for entity in &ev.with_entities {
                    commands.entity(*entity).despawn();
                }
            },
        );

    commands.insert_resource(ShootCooldown(Timer::from_seconds(
        SHOOT_COOLDOWN_SECS,
        TimerMode::Once,
    )));

    commands.insert_resource(PlayerHandles {
        player_mesh,
        player_material,
    });

    commands.insert_resource(Lives(3));
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
        transform.rotation = Quat::from_rotation_z(-FRAC_PI_2 + 0.05);
    } else if transform.translation.y < old_y {
        transform.rotation = Quat::from_rotation_z(-FRAC_PI_2 - 0.05);
    } else {
        transform.rotation = Quat::from_rotation_z(-FRAC_PI_2);
    }

    // Also move with the level speed
    transform.translation.x -= LEVEL_SPEED * dt;
    if transform.translation.x < PLAYER_LEFT_BOUNDARY {
        transform.translation.x = PLAYER_LEFT_BOUNDARY;
    }
}

fn shoot(
    mut commands: Commands,
    transform: Single<&Transform, With<Player>>,
    buttons: Res<ButtonInput<KeyCode>>,
    mut cooldown: ResMut<ShootCooldown>,
    time: Res<Time<Fixed>>,
) {
    cooldown.tick(time.delta());
    if !cooldown.is_finished() {
        return;
    }

    if buttons.pressed(KeyCode::Space) {
        cooldown.reset();
        commands.run_system_cached_with(launch_torpedo, transform.translation.xy());
    }
}

fn show_lives(
    mut commands: Commands,
    lives: Res<Lives>,
    handles: Res<PlayerHandles>,
    q: Query<(Entity, &Life), With<Life>>,
) {
    if lives.is_changed() {
        let count = q.count();
        for i in (count + 1)..=(**lives as usize) {
            let x = -1280.0 + 50.0 + (i as f32) * 50.0;
            let y = 720.0 - 50.0;
            commands.spawn((
                Life(i as u8),
                Mesh2d(handles.player_mesh.clone()),
                MeshMaterial2d(handles.player_material.clone()),
                Transform::from_xyz(x, y, 0.1)
                    .with_scale(Vec3::splat(0.5))
                    .with_rotation(Quat::from_rotation_z(-FRAC_PI_2)),
            ));
        }
        for (entity, life) in q {
            if **life > **lives {
                commands.entity(entity).despawn();
            }
        }
    }
}
