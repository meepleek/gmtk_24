// prelude module to simplify common imports
#![allow(unused_imports)]

pub(crate) use crate::tween::*;
pub(crate) use crate::{
    anim::FadeOutSpriteHiearchy,
    assets::{assets_exist, FontAssets, MusicAssets, SfxAssets, SpriteAssets},
    audio::{
        music::{MusicCommands, MusicTrack},
        sfx::{Sfx, SfxCommands},
    },
    camera::HIGH_RES_RENDER_LAYER,
    ext::*,
    game::{
        level::{
            level_ready, Ground, LevelEntityLookup, Movable, Moving, UnbreakableGround, TILE_SIZE,
        },
        physics::{GamePhysicsLayer, Gravity, Grounded, TileCollider, Velocity},
        player::Player,
        rock::Rock,
        word::{WordTileEvent, WordTileEventKind, WordTileStatus},
    },
    input::{MovementBindings, TypedInput, UiAction, UiInput},
    math::*,
    screens::{in_game, transition::TransitionScreenCommandExt, Screen},
    theme::prelude::*,
    time::*,
    word_loader::WordListSource,
    AppSet,
};
pub(crate) use avian2d::prelude::{Collider, CollisionLayers};
pub(crate) use bevy::{prelude::*, utils::HashMap};
pub(crate) use bevy_ecs_ldtk::prelude::*;
pub(crate) use bevy_tweening::{
    asset_animator_system, component_animator_system, Animator, AssetAnimator, Ease, EaseFunction,
    TweenCompleted,
};
pub(crate) use rand::prelude::*;
pub(crate) use tiny_bail::prelude::{or_continue, or_continue_quiet, or_return, or_return_quiet};
