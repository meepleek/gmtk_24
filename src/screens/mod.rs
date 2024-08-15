//! The game's main screen states and transitions between them.

mod credits;
mod game;
mod loading;
mod main_menu;
mod settings;
mod splash;
pub(crate) mod transition;
mod tutorial;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        main_menu::plugin,
        credits::plugin,
        game::plugin,
        settings::plugin,
        tutorial::plugin,
        transition::plugin,
    ));
}

#[allow(dead_code)]
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[cfg_attr(not(feature = "dev"), default)]
    Splash,
    #[cfg_attr(feature = "dev", default)]
    Loading,
    Loaded,
    MainMenu,
    Settings,
    Tutorial,
    Credits,
    Game,
    RestartGame,
    Score,
    Quit,
}

macro_rules! transition_system {
    ($name: ident, $screen:tt) => {
        paste::paste! {
            pub(crate) fn [<trigger_transition_to_ $name>](_trigger: Trigger<OnPress>, mut cmd: Commands) {
                cmd.transition_to_screen(Screen::$screen);
            }
        }
    };
}

transition_system!(main_menu, MainMenu);
transition_system!(game, Game);
transition_system!(tutorial, Tutorial);
transition_system!(settings, Settings);
transition_system!(credits, Credits);
