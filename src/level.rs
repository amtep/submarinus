use bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

use crate::constants::{LEVEL_SPEED, SURFACE_Y};

pub fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<SurfaceMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_level);
}

/// Marker for entities that should move with the terrain scroll speed
#[derive(Component, Default, Clone)]
pub struct Terrain;

/// Marker for water-air boundary
#[derive(Component, Default, Clone)]
pub struct Surface;

const SURFACE_SHADER_PATH: &str = "shaders/surface.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct SurfaceMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for SurfaceMaterial {
    fn fragment_shader() -> ShaderRef {
        SURFACE_SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SurfaceMaterial>>,
) {
    let mesh = meshes.add(Rectangle::default());
    let material = materials.add(SurfaceMaterial {
        color: LinearRgba::BLUE,
    });

    for x in -13..=13 {
        commands.spawn((
            Terrain,
            Surface,
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_xyz(x as f32 * 100.0, SURFACE_Y, 0.0)
                .with_scale(Vec3::new(100.0, 10.0, 1.0)),
        ));
    }
}

fn move_level(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform), With<Terrain>>,
    time: Res<Time<Fixed>>,
) {
    for (entity, mut transform) in &mut q {
        transform.translation.x -= LEVEL_SPEED * time.delta_secs();
        // If the x is offscreen with a generous margin, despawn it
        if transform.translation.x < -1280.0 - 200.0 {
            commands.entity(entity).despawn();
        }
    }
}
