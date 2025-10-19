#![feature(let_chains)]

mod anim;
mod assets;
mod audio;
mod camera;
#[cfg(feature = "dev")]
mod dev_tools;
mod ext;
mod game;
mod math;
mod prelude;
mod screens;
mod theme;
mod time;
mod tween;
mod word_loader;

use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
};
use prelude::*;

const GAME_NAME: &str = "GMTK 2024";

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            FixedUpdate,
            (
                AppSet::TickTimers,
                AppSet::CollectInput,
                AppSet::Update,
                AppSet::UpdateCoords,
            )
                .chain(),
        )
        .configure_sets(
            Update,
            (
                AppSet::TickTimers,
                AppSet::CollectInput,
                AppSet::Update,
                AppSet::UpdateCoords,
            )
                .chain(),
        );

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: GAME_NAME.to_string(),
                        resolution: Vec2::splat(1024.).into(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );

        // Add other plugins.
        app.add_plugins((
            word_loader::plugin,
            game::plugin,
            screens::plugin,
            theme::plugin,
            assets::plugin,
            audio::plugin,
            tween::plugin,
            anim::plugin,
            camera::plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    TickTimers,
    CollectInput,
    Update,
    UpdateCoords,
}
