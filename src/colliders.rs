use bevy::{
    camera::primitives::Aabb,
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};
use throttled_tracing::info_every_n;

use crate::math::{get_triangles2d, quat_to_rot2, triangles2d_overlap};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, collisions);
}

pub const COLLIDE_LAYER_TERRAIN: u8 = 1 << 0;
pub const COLLIDE_LAYER_ENEMY: u8 = 1 << 1;
pub const COLLIDE_LAYER_SURFACE: u8 = 1 << 2;

/// Component to mark entities that can collide with other entities.
/// The field is a bitmask of `COLLIDE_LAYER_` values indicating which entities
/// this entity can collide with.
#[derive(Component)]
pub struct Collider(pub u8);

/// Component to mark entities that can be collided with.
/// The field is a `COLLIDE_LAYER_` value.
#[derive(Component)]
pub struct CollideLayer(pub u8);

/// An event triggered when a collision has been detected between a [`Collider`]
/// and an entity with a [`CollideLayer`] on the same level.
#[derive(EntityEvent)]
pub struct Collided {
    pub entity: Entity,
    pub with_entities: Vec<Entity>,
}

fn collisions(
    mut commands: Commands,
    colliders: Query<(Entity, &Collider, &Mesh2d, &Transform, &Aabb)>,
    collide_with: Query<(Entity, &CollideLayer, &Mesh2d, &Transform, &Aabb)>,
    meshes: Res<Assets<Mesh>>,
) {
    let mut comparisons = 0;
    let mut rough_intersects = 0;
    for (entity1, Collider(layers), mesh1, transform1, aabb1) in colliders {
        let mut collided = Vec::new();
        let rough1 = Aabb2d {
            min: aabb1.min().xy(),
            max: aabb1.max().xy(),
        }
        .transformed_by(
            transform1.translation.xy(),
            quat_to_rot2(&transform1.rotation),
        )
        .scale_around_center(transform1.scale.xy());
        // TODO: only do this work if a rough collision is detected
        let mesh1 = meshes.get(mesh1.id()).unwrap();
        let triangles1 = get_triangles2d(mesh1, transform1);

        for (entity2, CollideLayer(layer), mesh2, transform2, aabb2) in collide_with {
            if entity1 == entity2 || (layer & layers) == 0 {
                continue;
            }
            comparisons += 1;
            let rough2 = Aabb2d {
                min: aabb2.min().xy(),
                max: aabb2.max().xy(),
            }
            .transformed_by(
                transform2.translation.xy(),
                quat_to_rot2(&transform2.rotation),
            )
            .scale_around_center(transform2.scale.xy());
            info_once!("{rough1:?}, {rough2:?}");
            if rough1.intersects(&rough2) {
                rough_intersects += 1;
                // TODO: cache triangles2 maybe
                let mesh2 = meshes.get(mesh2.id()).unwrap();
                let triangles2 = get_triangles2d(mesh2, transform2);
                if triangles2d_overlap(&triangles1, &triangles2) {
                    collided.push(entity2);
                }
            }
        }

        if !collided.is_empty() {
            commands.entity(entity1).trigger(|entity| Collided {
                entity,
                with_entities: collided,
            });
        }
    }
    info_every_n!(64, "comparisons {comparisons}, rough {rough_intersects}");
}
