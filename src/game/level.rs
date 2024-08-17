//! Spawn the main level.

use super::player::SpawnPlayer;
use crate::prelude::*;
use bevy::ecs::world::Command;

pub(crate) const GRID_SIZE: f32 = 64.;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, draw_level_grid.run_if(in_game));
}

#[derive(Component, Debug, Deref, DerefMut)]
pub(crate) struct Coordinate(pub IVec2);

impl Coordinate {
    pub(crate) fn to_world(&self) -> Vec3 {
        (self.0.as_vec2() * GRID_SIZE).extend(0.)
    }
}

#[derive(Debug)]
pub(crate) struct SpawnLevel;

impl Command for SpawnLevel {
    fn apply(self, world: &mut World) {
        // The only thing we have in our level is a player,
        // but add things like walls etc. here.
        world.commands().add(SpawnPlayer { _max_speed: 400.0 });

        // Flush the commands we just added so that they are
        // all executed now, as part of this command.
        world.flush_commands();
    }
}

fn draw_level_grid(mut gizmos: Gizmos) {
    gizmos
        .grid_2d(
            Vec2::ONE * (GRID_SIZE / 2.),
            0.0,
            UVec2::splat(64),
            Vec2::splat(GRID_SIZE),
            // Dark gray
            LinearRgba::gray(0.1),
        )
        .outer_edges();
}
