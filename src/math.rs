use bevy::prelude::*;

/// Convert a [`Quat`] to a [`Rot2`] on the assumption that the `Quat` is
/// only rotated around z.
// TODO: test what happens if a Quat is rotated 0 around z.
pub fn quat_to_rot2(q: &Quat) -> Rot2 {
    let (v, a) = q.to_axis_angle();
    Rot2::radians(a * v.z)
}

/// Get the triangles from a [`Mesh`] and transform them into place.
pub fn get_triangles2d(mesh: &Mesh, transform: &Transform) -> Vec<Triangle2d> {
    mesh.triangles()
        .expect("mesh not extracted yet")
        .map(|t3d| {
            Triangle2d::new(
                transform.transform_point(t3d.vertices[0]).xy(),
                transform.transform_point(t3d.vertices[1]).xy(),
                transform.transform_point(t3d.vertices[2]).xy(),
            )
        })
        .collect()
}

/// Check whether two sets of triangles have any overlap with each other.
pub fn triangles2d_overlap(triangles1: &[Triangle2d], triangles2: &[Triangle2d]) -> bool {
    for triangle1 in triangles1 {
        for triangle2 in triangles2 {
            if triangle2d_contains_point(triangle1, triangle2.vertices[0])
                || triangle2d_contains_point(triangle1, triangle2.vertices[1])
                || triangle2d_contains_point(triangle1, triangle2.vertices[2])
            {
                return true;
            }
        }
    }
    false
}

/// Check whether a point is inside a [`Triangle2d`]
pub fn triangle2d_contains_point(triangle: &Triangle2d, point: Vec2) -> bool {
    // Algorithm from https://stackoverflow.com/questions/2049582/how-to-determine-if-a-point-is-in-a-2d-triangle
    fn sign(p1: Vec2, p2: Vec2, p3: Vec2) -> f32 {
        (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
    }

    let d1 = sign(point, triangle.vertices[0], triangle.vertices[1]);
    let d2 = sign(point, triangle.vertices[1], triangle.vertices[2]);
    let d3 = sign(point, triangle.vertices[2], triangle.vertices[0]);

    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;

    !(has_neg && has_pos)
}
