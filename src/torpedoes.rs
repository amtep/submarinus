use bevy::prelude::*;
use rand::RngExt;

use crate::{bubbles::add_bubbles, constants::LEVEL_SPEED, level::Terrain, random::RandomSource};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(FixedUpdate, torpedoes);
}

const TORPEDO_MAX_SPEED: f32 = 200.0;
const TORPEDO_ACCELERATION: f32 = 75.0;
const TORPEDO_AVG_BUBBLES_PER_SEC: f32 = 10.0;

#[derive(Resource)]
pub struct TorpedoHandles {
    torpedo_tip_mesh: Handle<Mesh>,
    torpedo_body_mesh: Handle<Mesh>,
    torpedo_tip_material: Handle<ColorMaterial>,
    torpedo_body_material: Handle<ColorMaterial>,
}

/// Marker component for torpedoes. It carries the forward speed information.
#[derive(Component, Default, Clone, Copy, Deref, DerefMut)]
struct Torpedo(f32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let torpedo_tip_mesh = meshes.add(Ellipse::new(5.0, 2.0));
    let torpedo_body_mesh = meshes.add(Rectangle::new(10.0, 4.0));
    let torpedo_tip_material = materials.add(Color::srgb_u8(0xff, 0x0, 0x0));
    let torpedo_body_material = materials.add(Color::srgb_u8(0x60, 0x60, 0x60));
    commands.insert_resource(TorpedoHandles {
        torpedo_tip_mesh,
        torpedo_body_mesh,
        torpedo_tip_material,
        torpedo_body_material,
    });
}

pub fn launch_torpedo(In(tip_pos): In<Vec2>, mut commands: Commands, handles: Res<TorpedoHandles>) {
    let mut body_center = tip_pos;
    body_center.x -= 10.0;
    let mut body_back = body_center;
    body_back.x -= 5.0;

    commands.spawn((
        Terrain,
        Torpedo(LEVEL_SPEED),
        Mesh2d(handles.torpedo_body_mesh.clone()),
        MeshMaterial2d(handles.torpedo_body_material.clone()),
        Transform::from_translation(body_center.extend(1.0)),
        children![(
            Mesh2d(handles.torpedo_tip_mesh.clone()),
            MeshMaterial2d(handles.torpedo_tip_material.clone()),
            Transform::from_xyz(5.0, 0.0, -0.1),
        )],
    ));
    commands.run_system_cached_with(add_bubbles, (body_back, 8, 5.0, 2.0));
}

/// Move and accelerate all torpedoes, emitting some bubbles along the way.
fn torpedoes(
    mut commands: Commands,
    mut torps: Query<(&mut Torpedo, &mut Transform)>,
    time: Res<Time<Fixed>>,
    mut rng: ResMut<RandomSource>,
) {
    let dt = time.delta_secs();

    for (mut speed, mut transform) in &mut torps {
        transform.translation.x += **speed * dt;
        let new_speed = (**speed + TORPEDO_ACCELERATION * dt).min(TORPEDO_MAX_SPEED);
        if **speed != new_speed {
            **speed = new_speed;
        }
        if rng.0.random::<f32>() < TORPEDO_AVG_BUBBLES_PER_SEC * dt {
            commands.run_system_cached_with(add_bubbles, (transform.translation.xy(), 1, 5.0, 2.0));
        }
    }
}
