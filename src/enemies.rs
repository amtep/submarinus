use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use rand::RngExt;

use crate::{
    colliders::{COLLIDE_LAYER_ENEMY, COLLIDE_LAYER_TERRAIN, CollideLayer, Collided, Collider},
    level::Sidescroll,
    random::RandomSource,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, (load, ships_drop_bombs, bombs_fall));
}

const ENEMIES_PATH: &str = "enemies.png";
const SHIP_AVG_BOMBS_PER_SEC: f32 = 0.05;
const BOMB_DROP_SPEED: f32 = 20.0;
const COLOR_SHIP: [u8; 4] = [0xbe, 0x0b, 0x19, 0xff];

#[derive(Resource)]
struct EnemiesHandles {
    enemies: Handle<Image>,
    ship_mesh: Handle<Mesh>,
    bomb_mesh: Handle<Mesh>,
    ship_material: Handle<ColorMaterial>,
    bomb_material: Handle<ColorMaterial>,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
struct Ship;

/// A marker component for a depth bomb dropped from a [`Ship`].
/// It contains the y value at which it should explode.
#[derive(Component)]
struct DepthBomb(f32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let ship_mesh = meshes.add(
        ConvexPolygon::new(vec![
            vec2(-1.0, -0.5),
            vec2(1.0, -0.5),
            vec2(1.5, 0.5),
            vec2(-1.5, 0.5),
        ])
        .unwrap(),
    );
    let ship_material = materials.add(Color::srgb_u8(0xbe, 0x0b, 0x19));
    let bomb_mesh = meshes.add(Capsule2d::new(0.5, 0.5));
    let bomb_material = materials.add(Color::srgb(0.7, 0.7, 0.7));
    let enemies = asset_server.load(ENEMIES_PATH);
    commands.insert_resource(EnemiesHandles {
        enemies,
        ship_mesh,
        bomb_mesh,
        ship_material,
        bomb_material,
    });
}

fn load(
    mut commands: Commands,
    mut done: Local<bool>,
    images: Res<Assets<Image>>,
    handles: Res<EnemiesHandles>,
) {
    if *done {
        return;
    }
    let Some(image) = images.get(handles.enemies.id()) else {
        return;
    };

    for x in 0..image.width() {
        let translate_x = (x as f32 - 40.0) * 32.0;
        for y in 0..image.height() {
            let translate_y = (22.5 - y as f32) * 32.0;
            let color = image.pixel_bytes(UVec3::new(x, y, 0)).unwrap();
            if color == COLOR_SHIP {
                commands.spawn((
                    Sidescroll,
                    Enemy,
                    CollideLayer(COLLIDE_LAYER_ENEMY),
                    Ship,
                    Mesh2d(handles.ship_mesh.clone()),
                    MeshMaterial2d(handles.ship_material.clone()),
                    Transform::from_xyz(translate_x, translate_y - 10.0, 0.0)
                        .with_scale(vec3(32.0, 32.0, 1.0)),
                ));
            }
        }
    }

    *done = true;
}

fn ships_drop_bombs(
    mut commands: Commands,
    ships: Query<&Transform, With<Ship>>,
    time: Res<Time<Fixed>>,
    handles: Res<EnemiesHandles>,
    mut rng: ResMut<RandomSource>,
) {
    let dt = time.delta_secs();

    for transform in ships {
        if rng.random::<f32>() < SHIP_AVG_BOMBS_PER_SEC * dt {
            // TODO: make this sensitive to terrain
            let explode_depth = rng.random_range(-500.0..400.0);
            commands
                .spawn((
                    Enemy,
                    DepthBomb(explode_depth),
                    CollideLayer(COLLIDE_LAYER_ENEMY),
                    Collider(COLLIDE_LAYER_TERRAIN),
                    Sidescroll,
                    Mesh2d(handles.bomb_mesh.clone()),
                    MeshMaterial2d(handles.bomb_material.clone()),
                    Transform::from_xyz(transform.translation.x, transform.translation.y, 0.1)
                        .with_scale(vec3(10.0, 10.0, 1.0))
                        .with_rotation(Quat::from_rotation_z(FRAC_PI_2 + 0.2)),
                ))
                .observe(|ev: On<Collided>, mut commands: Commands| {
                    commands.entity(ev.entity).despawn();
                });
        }
    }
}

fn bombs_fall(
    mut commands: Commands,
    mut bombs: Populated<(Entity, &DepthBomb, &mut Transform), With<DepthBomb>>,
    time: Res<Time<Fixed>>,
) {
    let dt = time.delta_secs();

    for (entity, DepthBomb(explode_depth), mut transform) in &mut bombs {
        transform.translation.y -= BOMB_DROP_SPEED * dt;
        if transform.translation.y < *explode_depth {
            // TODO: explode
            commands.entity(entity).despawn();
        }
    }
}
