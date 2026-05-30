use bevy::prelude::*;

use crate::level::Sidescroll;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup).add_systems(Update, load);
}

const ENEMIES_PATH: &str = "enemies.png";

const COLOR_SHIP: [u8; 4] = [0xbe, 0x0b, 0x19, 0xff];

#[derive(Resource)]
struct EnemiesHandles {
    enemies: Handle<Image>,
    ship_mesh: Handle<Mesh>,
    ship_material: Handle<ColorMaterial>,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
struct Ship;

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
    let enemies = asset_server.load(ENEMIES_PATH);
    commands.insert_resource(EnemiesHandles {
        enemies,
        ship_mesh,
        ship_material,
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
