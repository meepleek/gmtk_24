use crate::prelude::*;
use bevy::ecs::world::Command;

pub(super) fn plugin(_app: &mut App) {
    // No setup required for this plugin.
    // It's still good to have a function here so that we can add some setup
    // later if needed.
}

pub(crate) enum Sfx {
    ButtonClick,
    ButtonHover,
}

impl SfxAssets {
    fn play(&self, sfx: Sfx, world: &mut World, settings: PlaybackSettings) {
        let source = match sfx {
            // can randomize for some SFX
            // let rng = &mut thread_rng();
            // let random_sfx = sfx_list.choose(rng).expect("Non-empty sfx list");
            Sfx::ButtonClick => self.button_click.clone_weak(),
            Sfx::ButtonHover => self.button_hover.clone_weak(),
        };

        world.spawn(AudioBundle { source, settings });
    }
}

/// A custom command used to play sound effects.
struct PlaySfx {
    sfx: Sfx,
    settings: PlaybackSettings,
}

impl Command for PlaySfx {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, sfx: Mut<SfxAssets>| {
            sfx.play(self.sfx, world, self.settings);
        });
    }
}

/// An extension trait with convenience methods for sound effect commands.
pub trait SfxCommands {
    fn play_sfx_with_settings(&mut self, sfx: Sfx, settings: PlaybackSettings);

    fn play_sfx(&mut self, sfx: Sfx) {
        self.play_sfx_with_settings(sfx, PlaybackSettings::DESPAWN);
    }
}

impl SfxCommands for Commands<'_, '_> {
    // By accepting an `Into<String>` here, we can be flexible about what we want to
    // accept: &str literals are better for prototyping and data-driven sound
    // effects, but enums are nicer for special-cased effects
    fn play_sfx_with_settings(&mut self, sfx: Sfx, settings: PlaybackSettings) {
        self.add(PlaySfx { sfx, settings });
    }
}
