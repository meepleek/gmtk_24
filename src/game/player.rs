use std::time::Duration;

use super::word::WordTile;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>()
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_systems(
            Update,
            (
                process_typed_input,
                process_movement_input,
                on_player_spawned,
                animate_player,
            )
                .run_if(level_ready),
        );
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Debug, Default, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub enum PlayerAnimation {
    #[default]
    Idle,
    SwingAnticipation,
    SwingAnticipationIdle,
    Swing,
    SwingFast,
}

impl PlayerAnimation {
    fn len(&self) -> usize {
        match self {
            PlayerAnimation::Idle => 5,
            PlayerAnimation::SwingAnticipation => 3,
            PlayerAnimation::SwingAnticipationIdle => 4,
            PlayerAnimation::Swing => 5,
            PlayerAnimation::SwingFast => 3,
        }
    }

    fn frame_base_duration_ms(&self, frame: usize) -> u64 {
        match self {
            PlayerAnimation::Swing => match frame {
                0 => 90,
                _ => 60,
            },
            PlayerAnimation::SwingFast => 80,
            _ => 100,
        }
    }

    fn is_idle(&self) -> bool {
        matches!(
            self,
            PlayerAnimation::Idle | PlayerAnimation::SwingAnticipationIdle
        )
    }
}

// todo: move
#[derive(Component, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct AnimationTimer(Timer);

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[grid_coords]
    grid_coords: GridCoords,
}

fn on_player_spawned(
    player_q: Query<(Entity, &GridCoords), Added<Player>>,
    mut cmd: Commands,
    sprites: Res<SpriteAssets>,
) {
    for (e, coords) in &player_q {
        cmd.entity(e).try_insert((
            SpriteBundle {
                texture: sprites.player_sheet.clone_weak(),
                transform: Transform::from_translation(coords.to_world()),
                ..default()
            },
            TextureAtlas {
                layout: sprites.idle_anim_layout.clone_weak(),
                index: 0,
            },
            PlayerAnimation::default(),
            AnimationTimer(Timer::new(
                Duration::from_millis(PlayerAnimation::Idle.frame_base_duration_ms(0)),
                TimerMode::Repeating,
            )),
        ));
    }
}

// todo: rotate player towards typed word if the current char is not found it the currently faced tile and the player hasn't started the tile yet (ignore when tile is not pristine?)
// todo: reset tiles when player moves away from a tile (or even rotates?)
fn process_typed_input(
    mut typed: ResMut<TypedInput>,
    player_q: Query<&GridCoords, With<Player>>,
    level_lookup: Res<LevelEntityLookup>,
    mut word_tile_q: Query<&mut WordTile>,
    mut word_tile_evw: EventWriter<WordTileEvent>,
) {
    let player_coords = or_return!(player_q.get_single());
    match typed.as_str() {
        "" => return,
        _ => {
            for neighbour_coords in player_coords.neighbours() {
                let neighbour_e = or_continue_quiet!(level_lookup.get(&neighbour_coords));
                let mut word_tile = or_continue_quiet!(word_tile_q.get_mut(*neighbour_e));
                if word_tile.remaining().starts_with(&typed.0) {
                    word_tile_evw.send(WordTileEvent {
                        e: *neighbour_e,
                        kind: word_tile.advance(typed.len(), neighbour_coords),
                    });
                }
                // todo: invalid input feedback
                // possibly reset the current word on error?
            }
        }
    }

    typed.clear();
}

// todo: just store intent
// but move in fixed time?
// todo: migrate to avian to handle collisions
// todo: track coords - possibly in a different general system that tracks by  transform
// todo: fix initial position - incorrect no matter the initial grid coords
fn process_movement_input(
    mut player_q: Query<&mut Transform, With<Player>>,
    // obstacle_q: Query<(), Or<(With<Ground>, With<UnbreakableGround>, With<Rock>)>>,
    bindings: Res<MovementBindings>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let mut player_t = or_return!(player_q.get_single_mut());
    let mut move_by = 0f32;
    if kb_input.pressed(bindings.left) {
        move_by += -1.0
    }
    if kb_input.pressed(bindings.right) {
        move_by += 1.0
    }
    if move_by != 0.0 {
        player_t.scale.x = move_by.signum();
        player_t.translation.x += move_by * 300.0 * time.delta_seconds();
        // let new_coords = *player_coords + move_by;
        // if let Some(e) = level_lookup.get(&new_coords) {
        //     if obstacle_q.contains(*e) {
        //         // todo: hit wall feedback
        //         return;
        //     }
        // }

        // *player_coords = new_coords;
    }
}

fn animate_player(
    time: Res<Time>,
    mut player_q: Query<
        (&mut AnimationTimer, &mut PlayerAnimation, &mut TextureAtlas),
        With<Player>,
    >,
    mut word_tile_evr: EventReader<WordTileEvent>,
    sprites: Res<SpriteAssets>,
) {
    let (mut timer, mut player_anim, mut atlas) = or_return!(player_q.get_single_mut());

    for ev in word_tile_evr.read() {
        if match ev.kind {
            WordTileEventKind::WordStarted => {
                atlas.layout = sprites.swing_anticipation_anim_layout.clone_weak();
                *player_anim = PlayerAnimation::SwingAnticipation;
                true
            }
            WordTileEventKind::WordFinished(_) | WordTileEventKind::TileFinished { .. } => {
                atlas.layout = sprites.swing_anim_layout.clone_weak();
                *player_anim = PlayerAnimation::Swing;
                true
            }
            _ => false,
        } {
            atlas.index = 0;
            timer.set_duration(Duration::from_millis(
                player_anim.frame_base_duration_ms(atlas.index),
            ));
            timer.reset();
            break;
        }
    }
    if word_tile_evr
        .read()
        .any(|ev| ev.kind == WordTileEventKind::WordStarted)
    {
        atlas.layout = sprites.swing_anticipation_anim_layout.clone_weak();
        *player_anim = PlayerAnimation::SwingAnticipation;
        atlas.index = 0;
        timer.set_duration(Duration::from_millis(
            player_anim.frame_base_duration_ms(atlas.index),
        ));
        timer.reset();
    }

    timer.tick(time.delta());
    if timer.just_finished() {
        atlas.index = (atlas.index + 1) % player_anim.len();
        if atlas.index == 0 && !player_anim.is_idle() {
            // todo: busy anticipation when the current anim is swing anticipation
            if *player_anim == PlayerAnimation::SwingAnticipation {
                atlas.layout = sprites.swing_anticipation_idle_anim_layout.clone_weak();
                *player_anim = PlayerAnimation::SwingAnticipationIdle;
            } else {
                atlas.layout = sprites.idle_anim_layout.clone_weak();
                *player_anim = PlayerAnimation::Idle;
            }
        }
        timer.set_duration(Duration::from_millis(
            player_anim.frame_base_duration_ms(atlas.index),
        ));
    }
}
