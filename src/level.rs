use bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

use crate::constants::LEVEL_SPEED;

pub fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<SurfaceMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, load)
        .add_systems(FixedUpdate, move_level);
}

const LEVEL_PATH: &str = "level.png";

const COLOR_WATER: [u8; 4] = [0x00, 0x00, 0x00, 0xff];
const COLOR_ROCK: [u8; 4] = [0xa7, 0x64, 0x01, 0xff];
const COLOR_AIR: [u8; 4] = [0x74, 0xb9, 0xde, 0xff];

#[derive(Resource)]
struct LevelHandles {
    level: Handle<Image>,
    rectangle_mesh: Handle<Mesh>,
    triangle_lr_mesh: Handle<Mesh>,
    triangle_ll_mesh: Handle<Mesh>,
    surface_material: Handle<SurfaceMaterial>,
    rock_material: Handle<ColorMaterial>,
}

/// Marker for entities that should move with the terrain scroll speed
#[derive(Component, Default, Clone)]
pub struct Terrain;

/// Marker for water-air boundary
#[derive(Component, Default, Clone)]
pub struct Surface;

/// Marker for rock tile
#[derive(Component, Default, Clone)]
pub struct Rock;

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
    mut materials1: ResMut<Assets<SurfaceMaterial>>,
    mut materials2: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    let rectangle_mesh = meshes.add(Rectangle::default());
    let ll = Vec2::new(-0.5, -0.5);
    let lr = Vec2::new(0.5, -0.5);
    let ul = Vec2::new(-0.5, 0.5);
    let ur = Vec2::new(0.5, 0.5);
    let triangle_ll_mesh = meshes.add(Triangle2d::new(ll, ul, lr));
    let triangle_lr_mesh = meshes.add(Triangle2d::new(lr, ll, ur));
    let surface_material = materials1.add(SurfaceMaterial {
        color: LinearRgba::BLUE,
    });
    let rock_material = materials2.add(Color::srgb_u8(0xa7, 0x64, 0x01));
    let level = asset_server.load(LEVEL_PATH);
    commands.insert_resource(LevelHandles {
        level,
        rectangle_mesh,
        triangle_ll_mesh,
        triangle_lr_mesh,
        surface_material,
        rock_material,
    });
}

fn load(
    mut commands: Commands,
    mut done: Local<bool>,
    images: Res<Assets<Image>>,
    handles: Res<LevelHandles>,
) {
    if *done {
        return;
    }
    let Some(image) = images.get(handles.level.id()) else {
        return;
    };

    for x in 0..image.width() {
        let mut prev_color: &[u8] = &[];
        let translate_x = (x as f32 - 40.0) * 32.0;
        for y in 0..image.height() {
            let translate_y = (22.5 - y as f32) * 32.0;
            let color = image.pixel_bytes(UVec3::new(x, y, 0)).unwrap();
            let maybe_next = image.pixel_bytes(UVec3::new(x, y + 1, 0)).ok();
            let maybe_prev_column = if x == 0 {
                None
            } else {
                image.pixel_bytes(UVec3::new(x - 1, y, 0)).ok()
            };
            let maybe_next_column = image.pixel_bytes(UVec3::new(x + 1, y, 0)).ok();
            if color == COLOR_WATER {
                if prev_color == COLOR_AIR {
                    commands.spawn((
                        Terrain,
                        Surface,
                        Mesh2d(handles.rectangle_mesh.clone()),
                        MeshMaterial2d(handles.surface_material.clone()),
                        Transform::from_xyz(translate_x, translate_y + 16.0, 0.0)
                            .with_scale(Vec3::new(32.0, 16.0, 1.0)),
                    ));
                }
                if maybe_next.is_some_and(|c| c == COLOR_ROCK) {
                    if maybe_next_column.is_some_and(|c| c == COLOR_ROCK) {
                        commands.spawn((
                            Terrain,
                            Rock,
                            Mesh2d(handles.triangle_lr_mesh.clone()),
                            MeshMaterial2d(handles.rock_material.clone()),
                            Transform::from_xyz(translate_x, translate_y, 0.0)
                                .with_scale(Vec3::new(32.0, 32.0, 1.0)),
                        ));
                    }
                    if maybe_prev_column.is_some_and(|c| c == COLOR_ROCK) {
                        commands.spawn((
                            Terrain,
                            Rock,
                            Mesh2d(handles.triangle_ll_mesh.clone()),
                            MeshMaterial2d(handles.rock_material.clone()),
                            Transform::from_xyz(translate_x, translate_y, 0.0)
                                .with_scale(Vec3::new(32.0, 32.0, 1.0)),
                        ));
                    }
                }
            } else if color == COLOR_ROCK {
                commands.spawn((
                    Terrain,
                    Rock,
                    Mesh2d(handles.rectangle_mesh.clone()),
                    MeshMaterial2d(handles.rock_material.clone()),
                    Transform::from_xyz(translate_x, translate_y, 0.0)
                        .with_scale(Vec3::new(32.0, 32.0, 1.0)),
                ));
            }
            prev_color = color;
        }
    }

    *done = true;
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
