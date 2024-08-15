//! A splash screen that plays briefly at startup.

use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use super::Screen;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Splash), spawn_splash);
    app.register_type::<SplashTimer>()
        .init_resource::<SplashTimer>();
    app.add_systems(
        Update,
        (
            tick_splash_timer.in_set(AppSet::TickTimers),
            check_splash_timer.in_set(AppSet::Update),
        )
            .run_if(in_state(Screen::Splash)),
    );
}

const SPLASH_DURATION_SECS: f32 = 1.8;

fn spawn_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .ui_root()
        .insert((Name::new("Splash screen"), StateScoped(Screen::Splash)))
        .with_children(|children| {
            children.spawn((
                Name::new("Splash image"),
                ImageBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        width: Val::Percent(70.0),
                        ..default()
                    },
                    image: UiImage::new(asset_server.load_with_settings(
                        // This should be an embedded asset for instant loading, but that is
                        // currently [broken on Windows Wasm builds](https://github.com/bevyengine/bevy/issues/14246).
                        "images/splash.png",
                        |settings: &mut ImageLoaderSettings| {
                            // Make an exception for the splash image in case
                            // `ImagePlugin::default_nearest()` is used for pixel art.
                            settings.sampler = ImageSampler::linear();
                        },
                    )),
                    ..default()
                },
            ));
        });
}

#[derive(Resource, Debug, Clone, PartialEq, Reflect)]
#[reflect(Resource)]
struct SplashTimer(Timer);

impl Default for SplashTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SPLASH_DURATION_SECS, TimerMode::Once))
    }
}

fn tick_splash_timer(time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    timer.0.tick(time.delta());
}

fn check_splash_timer(timer: ResMut<SplashTimer>, mut cmd: Commands) {
    if timer.0.just_finished() {
        cmd.transition_to_screen(Screen::Loading);
    }
}
