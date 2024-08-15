// prelude module to simplify common imports
#![allow(unused_imports)]

pub(crate) use crate::tween::*;
pub(crate) use crate::{
    assets::{assets_exist, MusicAssets, SfxAssets, SpriteAssets},
    audio::{
        music::{MusicCommands, MusicTrack},
        sfx::{Sfx, SfxCommands},
    },
    ext::*,
    input::{PlayerAction, PlayerInput, UiAction, UiInput},
    math::*,
    screens::{transition::TransitionScreenCommandExt, Screen},
    theme::prelude::*,
    time::*,
    AppSet,
};
pub(crate) use bevy::{prelude::*, utils::HashMap};
pub(crate) use bevy_tweening::{
    asset_animator_system, component_animator_system, Animator, AssetAnimator, Ease, EaseFunction,
    TweenCompleted,
};
pub(crate) use rand::prelude::*;
pub(crate) use tiny_bail::prelude::{or_continue, or_continue_quiet, or_return, or_return_quiet};
