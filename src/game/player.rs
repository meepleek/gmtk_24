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
                tween_player_movement,
                move_player_to_finished_word_cell,
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
    player_q: Query<Entity, Added<Player>>,
    mut cmd: Commands,
    sprites: Res<SpriteAssets>,
) {
    for e in &player_q {
        cmd.entity(e).try_insert((
            SpriteBundle {
                texture: sprites.player_sheet.clone_weak(),
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

fn process_typed_input(
    mut typed: ResMut<TypedInput>,
    mut player_q: Query<(&mut GridCoords, &mut Transform), With<Player>>,
    level_lookup: Res<LevelEntityLookup>,
    ground_q: Query<(), Or<(With<Ground>, With<UnbreakableGround>)>>,
    mut word_tile_q: Query<&mut WordTile>,
    mut word_tile_evw: EventWriter<WordTileEvent>,
    bindings: Res<MovementBindings>,
) {
    let (mut player_coords, mut player_t) = or_return!(player_q.get_single_mut());

    if let Some(move_by) = match typed.as_str() {
        "" => return,
        c if c == bindings.left => Some(GridCoords::neg_x()),
        c if c == bindings.right => Some(GridCoords::x()),
        c if c == bindings.up => Some(GridCoords::y()),
        c if c == bindings.down => Some(GridCoords::neg_y()),
        _ => {
            for neighbour_coords in player_coords.neighbours() {
                let neighbour_e = or_continue_quiet!(level_lookup.get(&neighbour_coords));
                let mut word_tile = or_continue_quiet!(word_tile_q.get_mut(*neighbour_e));
                if word_tile.remaining().starts_with(&typed.0) {
                    word_tile_evw.send(WordTileEvent {
                        e: *neighbour_e,
                        kind: word_tile.advance(typed.len()),
                    });
                }
                // todo: invalid input feedback
                // possibly reset the current word on error?
            }

            typed.clear();
            None
        }
    } {
        typed.clear();

        if move_by.x != 0 {
            player_t.scale.x = move_by.x.signum() as f32;
        }
        let new_coords = *player_coords + move_by;
        if let Some(e) = level_lookup.get(&new_coords) {
            if ground_q.contains(*e) {
                // todo: hit wall feedback
                return;
            }
        }

        *player_coords = new_coords;
    }
}

fn tween_player_movement(
    player_q: Query<(Entity, &GridCoords), (With<Player>, Changed<GridCoords>)>,
    mut cmd: Commands,
) {
    for (e, coord) in &player_q {
        cmd.tween_translation(e, coord.to_world(), 110, EaseFunction::QuadraticOut);
    }
}

fn move_player_to_finished_word_cell(
    mut word_tile_evr: EventReader<WordTileEvent>,
    mut player_q: Query<&mut GridCoords, With<Player>>,
    coords_q: Query<&GridCoords, Without<Player>>,
) {
    // only move when there's an exactly ONE finished word
    // todo: instead of doing that, move only in the facing/last move direction when there's more than 1 finished word
    // or maybe move in that direction only when the word is followed by pressing space
    let Some(ev) = word_tile_evr
        .read()
        .find(|ev| ev.kind == WordTileEventKind::TileFinished)
    else {
        return;
    };

    let tile_coords = or_return!(coords_q.get(ev.e));
    let mut player_coords = or_return!(player_q.get_single_mut());
    *player_coords = *tile_coords;
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
            WordTileEventKind::WordFinished | WordTileEventKind::TileFinished => {
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
