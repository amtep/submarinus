use bevy::{
    math::bounding::{Bounded2d, IntersectsVolume},
    prelude::*,
};
use rand::RngExt;

use crate::{
    bubbles::add_bubbles,
    constants::LEVEL_SPEED,
    level::{Rock, Terrain},
    math::{get_triangles2d, quat_to_rot2, triangles2d_overlap},
    random::RandomSource,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(FixedUpdate, (torpedoes, torpedo_hit));
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

fn torpedo_hit(
    mut commands: Commands,
    torpedoes: Query<(Entity, &Transform, &Mesh2d), (With<Torpedo>, Without<Rock>)>,
    rocks: Query<(Entity, &Transform, &Mesh2d), With<Rock>>,
    meshes: Res<Assets<Mesh>>,
) {
    for (torpedo, transform, mesh2d) in torpedoes {
        let mut hit = false;
        let mesh = meshes.get(mesh2d.id()).unwrap();
        let triangles = get_triangles2d(mesh, transform);

        let bounding_rough = Rectangle::new(10.0, 4.0).aabb_2d(Isometry2d::new(
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
            commands.entity(torpedo).despawn();
        }
    }
}
