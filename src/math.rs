use bevy::prelude::*;

pub fn quat_to_rot2(q: &Quat) -> Rot2 {
    let (v, a) = q.to_axis_angle();
    Rot2::radians(a * v.z)
}
