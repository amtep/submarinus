use bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
};

pub fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<SurfaceMaterial>::default())
        .add_systems(Startup, setup);
}

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
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SurfaceMaterial>>,
) {
    // Do the surface as a series of 100px segments
    let mesh = meshes.add(Rectangle::from_size(Vec2 { x: 100.0, y: 10.0 }));
    let material = materials.add(SurfaceMaterial {
        color: LinearRgba::BLUE,
    });

    for x in -13..=13 {
        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_xyz(x as f32 * 100.0, 500.0, 0.0),
        ));
    }
}
