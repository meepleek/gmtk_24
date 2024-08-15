use crate::prelude::*;
use bevy::{
    audio::PlaybackMode,
    ecs::{system::RunSystemOnce as _, world::Command},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MusicSource>();
}

/// Marker component for the soundtrack entity so we can find it later.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct MusicSource;

/// A custom command used to play soundtracks.
#[derive(Debug)]
enum PlayMusic {
    Track(MusicTrack),
    Disable,
}

#[derive(Debug)]
pub(crate) enum MusicTrack {
    MainMenu,
    Game,
}

impl Command for PlayMusic {
    /// This command will despawn the current soundtrack, then spawn a new one
    /// if necessary.
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, play_music);
    }
}

// todo: don't restart the same track if it's already playing
fn play_music(
    In(config): In<PlayMusic>,
    mut commands: Commands,
    music_query: Query<Entity, With<MusicSource>>,
    music: Res<MusicAssets>,
) {
    for entity in music_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let track = match config {
        PlayMusic::Track(key) => key,
        PlayMusic::Disable => return,
    };

    commands.spawn((
        AudioSourceBundle {
            source: match track {
                MusicTrack::MainMenu => music.main_menu.clone_weak(),
                MusicTrack::Game => music.game.clone_weak(),
            },
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        MusicSource,
    ));
}

/// An extension trait with convenience methods for music commands.
pub trait MusicCommands {
    /// Play a track, replacing the current one.
    /// music will loop.
    fn play_music(&mut self, track: MusicTrack);

    /// Stop the current soundtrack.
    fn stop_music(&mut self);
}

impl MusicCommands for Commands<'_, '_> {
    fn play_music(&mut self, track: MusicTrack) {
        self.add(PlayMusic::Track(track));
    }

    fn stop_music(&mut self) {
        self.add(PlayMusic::Disable);
    }
}
