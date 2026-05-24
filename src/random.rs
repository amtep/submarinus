use bevy::prelude::*;
use rand::{make_rng, rngs::StdRng};

pub fn plugin(app: &mut App) {
    app.insert_resource(RandomSource(make_rng()));
}

#[derive(Resource, Deref, DerefMut)]
pub struct RandomSource(pub StdRng);
