//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::prelude::*;
use bevy::{
    color::palettes::tailwind, dev_tools::states::log_transitions,
    input::common_conditions::input_toggle_active,
};

#[cfg(feature = "dev")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, log_transitions::<Screen>)
        .add_systems(
            Update,
            draw_level_grid.run_if(input_toggle_active(false, MouseButton::Right)),
        );

    #[cfg(feature = "dev")]
    {
        app.add_plugins((
            WorldInspectorPlugin::new().run_if(input_toggle_active(false, MouseButton::Right)),
            avian2d::debug_render::PhysicsDebugPlugin::default(),
        ));
        app.add_systems(FixedUpdate, draw_kinematic_sensor_gizmos);
    }
}

fn draw_level_grid(mut gizmos: Gizmos) {
    gizmos
        .grid_2d(
            Vec2::ZERO,
            UVec2::splat(64),
            Vec2::splat(TILE_SIZE as f32),
            // Dark gray
            LinearRgba::gray(0.1),
        )
        .outer_edges();
}

fn draw_kinematic_sensor_gizmos(
    sensor_q: Query<(&KinematicSensor, &Transform, &Grounded)>,
    mut gizmos: Gizmos,
) {
    for (sensor, t, grounded) in &sensor_q {
        gizmos.rect_2d(
            sensor.translation(t.translation),
            sensor.size,
            if grounded.is_grounded() {
                tailwind::BLUE_400
            } else {
                tailwind::GREEN_400
            },
        );
    }
}
