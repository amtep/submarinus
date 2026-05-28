use std::f32::consts::FRAC_PI_2;

use bevy::{
    math::bounding::{Bounded2d, IntersectsVolume},
    prelude::*,
};

use crate::{
    constants::{LEVEL_SPEED, SHOOT_COOLDOWN_SECS},
    level::{Rock, Surface},
    math::{get_triangles2d, quat_to_rot2, triangles2d_overlap},
    torpedoes::launch_torpedo,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(FixedUpdate, (keys, shoot, collisions, show_lives));
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

    commands.spawn((
        Player,
        Mesh2d(player_mesh.clone()),
        MeshMaterial2d(player_material.clone()),
        Transform::from_xyz(-1000.0, 0.0, 0.0).with_rotation(Quat::from_rotation_z(-FRAC_PI_2)),
    ));

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

fn collisions(
    mut commands: Commands,
    mut lives: ResMut<Lives>,
    transform: Single<(&Transform, &Mesh2d), (With<Player>, Without<Rock>)>,
    rocks: Query<(Entity, &Transform, &Mesh2d), With<Rock>>,
    meshes: Res<Assets<Mesh>>,
) {
    let mut hit = false;
    let (transform, mesh2d) = transform.into_inner();
    let mesh = meshes.get(mesh2d.id()).unwrap();
    let triangles = get_triangles2d(mesh, transform);

    let bounding_rough =
        Capsule2d::new(PLAYER_CAPSULE_RADIUS, PLAYER_INNER_WIDTH).aabb_2d(Isometry2d::new(
            transform.translation.xy(),
            quat_to_rot2(&transform.rotation),
        ));

    for (entity, rock_transform, rock_mesh2d) in rocks {
        let rock_rough = Rectangle::new(32.0, 32.0).aabb_2d(rock_transform.translation.xy());
        if bounding_rough.intersects(&rock_rough) {
            let rock_mesh = meshes.get(rock_mesh2d.id()).unwrap();
            let rock_triangles = get_triangles2d(rock_mesh, rock_transform);
            if triangles2d_overlap(&triangles, &rock_triangles) {
                hit = true;
                commands.entity(entity).despawn();
            }
        }
    }

    if hit {
        if **lives > 0 {
            **lives -= 1;
        } else {
            // TODO: game over
        }
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
