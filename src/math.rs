use bevy::prelude::*;

/// Convert a [`Quat`] to a [`Rot2`] on the assumption that the `Quat` is
/// only rotated around z.
// TODO: test what happens if a Quat is rotated 0 around z.
pub fn quat_to_rot2(q: &Quat) -> Rot2 {
    let (v, a) = q.to_axis_angle();
    Rot2::radians(a * v.z)
}
