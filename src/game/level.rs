use crate::prelude::*;

pub(crate) const TILE_SIZE: f32 = 16.;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
        .add_systems(Update, draw_level_grid.run_if(in_game))
        .add_systems(OnEnter(Screen::Game), spawn_level);
}

#[derive(Component, Debug, Deref, DerefMut)]
pub(crate) struct Coordinate(pub IVec2);

impl Coordinate {
    pub(crate) fn to_world(&self) -> Vec3 {
        (self.0.as_vec2() * TILE_SIZE).extend(0.)
    }
}

fn spawn_level(ass: Res<AssetServer>, mut cmd: Commands) {
    cmd.spawn(LdtkWorldBundle {
        ldtk_handle: ass.load("levels.ldtk"),
        ..Default::default()
    });
}

fn draw_level_grid(mut gizmos: Gizmos) {
    gizmos
        .grid_2d(
            Vec2::ZERO,
            0.0,
            UVec2::splat(64),
            Vec2::splat(TILE_SIZE),
            // Dark gray
            LinearRgba::gray(0.1),
        )
        .outer_edges();
}
