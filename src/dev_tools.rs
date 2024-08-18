//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::prelude::*;
use bevy::{
    dev_tools::{states::log_transitions, ui_debug_overlay::DebugUiPlugin},
    input::common_conditions::input_toggle_active,
};

#[cfg(feature = "dev")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, log_transitions::<Screen>)
        .add_plugins(DebugUiPlugin)
        .add_systems(
            Update,
            draw_level_grid.run_if(input_toggle_active(false, MouseButton::Middle)),
        );

    #[cfg(feature = "dev")]
    app.add_plugins(
        WorldInspectorPlugin::new().run_if(input_toggle_active(false, MouseButton::Middle)),
    );
}

fn draw_level_grid(mut gizmos: Gizmos) {
    gizmos
        .grid_2d(
            Vec2::ZERO,
            0.0,
            UVec2::splat(64),
            Vec2::splat(TILE_SIZE as f32),
            // Dark gray
            LinearRgba::gray(0.1),
        )
        .outer_edges();
}
