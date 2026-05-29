use bevy::prelude::*;

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
        for vertex1 in 0..3 {
            let s1 = Segment2d::new(
                triangle1.vertices[vertex1],
                triangle1.vertices[(vertex1 + 1) % 3],
            );
            for triangle2 in triangles2 {
                for vertex2 in 0..3 {
                    let s2 = Segment2d::new(
                        triangle2.vertices[vertex2],
                        triangle2.vertices[(vertex2 + 1) % 3],
                    );
                    if segment2d_intersect(&s1, &s2) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Returns whether this [`Segment2d`] intersects another.
/// Adapted from bevy PR#20300
pub fn segment2d_intersect(seg1: &Segment2d, seg2: &Segment2d) -> bool {
    let p = seg1.point1();
    let q = seg2.point1();
    let r = seg1.scaled_direction();
    let s = seg2.scaled_direction();

    let pq = q - p;
    let pq_cross_r = pq.perp_dot(r);
    let pq_cross_s = pq.perp_dot(s);
    let r_cross_s = r.perp_dot(s);

    if r_cross_s != 0.0 {
        // non parallel
        let t = pq_cross_s / r_cross_s;
        let u = pq_cross_r / r_cross_s;
        let t_in_range = (0.0..=1.0).contains(&t);
        let u_in_range = (0.0..=1.0).contains(&u);
        t_in_range && u_in_range
    } else if pq_cross_r == 0.0 || pq_cross_s == 0.0 {
        // collinear
        let r_len2 = r.length_squared();
        let s_len2 = s.length_squared();
        match (r_len2 == 0.0, s_len2 == 0.0) {
            // point point
            (true, true) => pq.length_squared() == 0.0,
            // segment point
            (false, true) if pq_cross_r == 0.0 => {
                let t = pq.dot(r) / r_len2;
                (0.0..=1.0).contains(&t)
            }
            // point segment
            (true, false) if pq_cross_s == 0.0 => {
                let t = -pq.dot(s) / s_len2;
                (0.0..=1.0).contains(&t)
            }
            // segment segment
            (false, false) => {
                let t0 = pq.dot(r) / r_len2;
                let t1 = t0 + s.dot(r) / r_len2;
                let (t_min, t_max) = if t0 < t1 { (t0, t1) } else { (t1, t0) };
                t_max >= 0.0 && t_min <= 1.0
            }
            _ => false,
        }
    } else {
        // parallel
        false
    }
}
