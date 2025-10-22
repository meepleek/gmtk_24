// prelude module to simplify common imports
#![allow(unused_imports)]

pub(crate) use crate::tween::*;
pub(crate) use crate::{
    AppSet,
    anim::FadeOutSpriteHiearchy,
    assets::{FontAssets, MusicAssets, SfxAssets, SpriteAssets, assets_exist},
    audio::{
        music::{MusicCommands, MusicTrack},
        sfx::{Sfx, SfxCommands},
    },
    camera::HIGH_RES_RENDER_LAYER,
    ext::*,
    game::{
        level::{
            Ground, LevelEntityLookup, Movable, Moving, TILE_SIZE, UnbreakableGround, level_ready,
        },
        physics::{
            GamePhysicsLayer, Gravity, Grounded, HorizontalObstacleDetection, KinematicSensor,
            TileCollider, Velocity,
        },
        player::{
            Player,
            input::{PlayerBindings, TimedButtonInput, UiAction},
            movement::{MovementEasing, MovementIntent},
        },
        rock::Rock,
        word::{WordTile, WordTileEvent, WordTileEventKind, WordTileStatus},
    },
    math::*,
    screens::{Screen, in_game, transition::TransitionScreenCommandExt},
    theme::prelude::*,
    time::*,
    word_loader::WordListSource,
};
pub(crate) use avian2d::prelude::{Collider, CollisionLayers};
pub(crate) use bevy::prelude::*;
pub(crate) use bevy_platform::collections::{HashMap, HashSet};

pub(crate) use bevy_ecs_ldtk::prelude::*;
pub(crate) use bevy_tweening::AnimCompletedEvent;
pub(crate) use leafwing_input_manager::buttonlike::ButtonState;
pub(crate) use rand::prelude::*;
pub(crate) use tiny_bail::prelude::{or_continue, or_continue_quiet, or_return, or_return_quiet};
