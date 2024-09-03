use std::time::Duration;

use super::word::TileWord;
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

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub enum PlayerAnimation {
    #[default]
    Idle,
    Mine,
    MineFast,
}

impl PlayerAnimation {
    fn len(&self) -> usize {
        match self {
            PlayerAnimation::Idle => 5,
            PlayerAnimation::Mine => 8,
            PlayerAnimation::MineFast => 3,
        }
    }

    fn frame_base_duration_ms(&self, frame: usize) -> u64 {
        match self {
            PlayerAnimation::Idle => 100,
            PlayerAnimation::Mine => match frame {
                0..=2 => 110,
                3 => 90,
                _ => 60,
            },
            PlayerAnimation::MineFast => 80,
        }
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
    mut player_q: Query<&mut GridCoords, With<Player>>,
    level_lookup: Res<LevelEntityLookup>,
    ground_q: Query<(), Or<(With<Ground>, With<UnbreakableGround>)>>,
    mut word_q: Query<&mut TileWord>,
    mut word_advanced_evw: EventWriter<WordAdvancedEvent>,
    mut word_finished_evw: EventWriter<WordFinishedEvent>,
    bindings: Res<MovementBindings>,
) {
    let mut coords = or_return!(player_q.get_single_mut());

    if let Some(move_by) = match typed.as_str() {
        "" => return,
        c if c == bindings.left => Some(GridCoords::neg_x()),
        c if c == bindings.right => Some(GridCoords::x()),
        c if c == bindings.up => Some(GridCoords::y()),
        c if c == bindings.down => Some(GridCoords::neg_y()),
        _ => {
            for neighbour_coords in coords.neighbours() {
                let neighbour_e = or_continue_quiet!(level_lookup.get(&neighbour_coords));
                let mut word = or_continue_quiet!(word_q.get_mut(*neighbour_e));
                if word.remaining().starts_with(&typed.0) {
                    word.advance(typed.len());
                    if word.status() == WordStatus::Finished {
                        word_finished_evw.send(WordFinishedEvent(*neighbour_e));
                    } else {
                        word_advanced_evw.send(WordAdvancedEvent(*neighbour_e));
                    }
                }
            }

            typed.clear();
            None
        }
    } {
        typed.clear();
        let new_coords = *coords + move_by;

        if let Some(e) = level_lookup.get(&new_coords) {
            if ground_q.contains(*e) {
                // todo: hit wall feedback
                return;
            }
        }

        *coords = new_coords;
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
    mut word_finished_evr: EventReader<WordFinishedEvent>,
    mut player_q: Query<&mut GridCoords, With<Player>>,
    coords_q: Query<&GridCoords, Without<Player>>,
) {
    // only move when there's an exactly ONE finished word
    // todo: instead of doing that, move only in the facing/last move direction when there's more than 1 finished word
    // or maybe move in that direction only when the word is followed by pressing space
    if word_finished_evr.len() != 1 {
        word_finished_evr.clear();
        return;
    }

    let ev = word_finished_evr.read().next().unwrap();
    let tile_coords = or_return!(coords_q.get(ev.0));
    let mut player_coords = or_return!(player_q.get_single_mut());
    *player_coords = *tile_coords;
}

fn animate_player(
    time: Res<Time>,
    mut player_q: Query<
        (&mut AnimationTimer, &mut PlayerAnimation, &mut TextureAtlas),
        With<Player>,
    >,
    mut word_advanced_evr: EventReader<WordAdvancedEvent>,
    mut word_finished_evr: EventReader<WordFinishedEvent>,
    sprites: Res<SpriteAssets>,
) {
    let (mut timer, mut player_anim, mut atlas) = or_return!(player_q.get_single_mut());

    if word_advanced_evr.clear_any() || word_finished_evr.clear_any() {
        atlas.layout = sprites.mine_anim_layout.clone_weak();
        *player_anim = PlayerAnimation::Mine;
        atlas.index = 3;
        timer.set_duration(Duration::from_millis(
            player_anim.frame_base_duration_ms(atlas.index),
        ));
        timer.reset();
    }

    timer.tick(time.delta());
    if timer.just_finished() {
        atlas.index = (atlas.index + 1) % player_anim.len();
        if atlas.index == 0 {
            atlas.layout = sprites.idle_anim_layout.clone_weak();
            *player_anim = PlayerAnimation::Idle;
        }
        timer.set_duration(Duration::from_millis(
            player_anim.frame_base_duration_ms(atlas.index),
        ));
    }
}
