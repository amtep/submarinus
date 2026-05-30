use bevy::prelude::*;
use rand::RngExt;

use crate::{
    colliders::{COLLIDE_LAYER_TERRAIN, Collided, Collider},
    level::{Sidescroll, Surface},
    random::RandomSource,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(FixedUpdate, float_bubbles);
}

const BUBBLE_SPEED: f32 = 50.0;

/// A bubble that wiggles upward to the surface
#[derive(Component, Default, Clone, Deref, DerefMut)]
struct Bubble(f32);

#[derive(Resource)]
pub struct BubbleHandles {
    mesh: Handle<Mesh>,
    color: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Ring::new(Circle::new(1.0), Circle::new(0.9)));
    let color = materials.add(Color::srgba(0.5, 0.5, 1.0, 1.0));
    commands.insert_resource(BubbleHandles { mesh, color });
    commands.run_system_cached_with(add_bubbles, (Vec2::splat(0.0), 8, 20.0, 10.0));
}

pub fn add_bubbles(
    In((pos, count, range, size)): In<(Vec2, usize, f32, f32)>,
    mut commands: Commands,
    mut rng: ResMut<RandomSource>,
    handles: Res<BubbleHandles>,
) {
    let circle = Circle::new(range);

    for _ in 0..count {
        let spawn_pos = pos + circle.sample_interior(&mut rng);
        commands
            .spawn((
                Sidescroll,
                Bubble(size),
                Collider(COLLIDE_LAYER_TERRAIN),
                Mesh2d(handles.mesh.clone()),
                MeshMaterial2d(handles.color.clone()),
                Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.0).with_scale(Vec3::splat(size)),
            ))
            .observe(|ev: On<Collided>, mut commands: Commands| {
                commands.entity(ev.entity).despawn();
            });
    }
}

fn float_bubbles(
    mut commands: Commands,
    mut rng: ResMut<RandomSource>,
    mut q: Populated<(Entity, &mut Bubble, &mut Transform), Without<Surface>>,
    surfaces: Query<&Transform, With<Surface>>,
    time: Res<Time<Fixed>>,
) {
    let dt = time.delta_secs();
    for (entity, mut bubble, mut transform) in &mut q {
        // TODO: make the randoms here depend on time delta

        // The bubble grows slowly
        let scale = 1.0 + (0.05 * dt);
        **bubble *= scale;
        transform.scale *= scale;

        // Random wiggle side to side
        match rng.random_range(0..5) {
            0 => transform.translation.x -= **bubble * dt,
            4 => transform.translation.x += **bubble * dt,
            _ => {}
        }

        // Bubble up
        transform.translation.y += BUBBLE_SPEED * dt;

        // Pop when breaking air
        for surface in surfaces {
            if transform.translation.y < surface.translation.y {
                continue;
            }
            let bounds = (surface.translation.x - surface.scale.x / 2.0)
                ..=(surface.translation.x + surface.scale.x / 2.0);
            if bounds.contains(&transform.translation.x) {
                commands.entity(entity).despawn();
                break;
            }
        }
    }
}
