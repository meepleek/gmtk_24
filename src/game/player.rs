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
            )
                .run_if(level_ready),
        );
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

fn process_typed_input(
    mut typed: ResMut<TypedInput>,
    mut player_q: Query<&mut GridCoords, With<Player>>,
    level_lookup: Res<LevelEntityLookup>,
    ground_q: Query<(), Or<(With<Ground>, With<UnbreakableGround>)>>,
    mut word_q: Query<&mut TileWord>,
    mut word_advanced_evw: EventWriter<WordAdvancedEvent>,
    mut word_finished_evw: EventWriter<WordFinishedEvent>,
) {
    let mut coords = or_return!(player_q.get_single_mut());

    if let Some(move_by) = match typed.as_str() {
        "" => return,
        // "a" => Some(GridCoords::neg_x()),
        // "d" => Some(GridCoords::x()),
        "n" => Some(GridCoords::neg_x()),
        "o" => Some(GridCoords::x()),
        // "w" => Some(GridCoords::y()),
        // "s" => Some(GridCoords::neg_y()),
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
    if word_finished_evr.len() != 1 {
        word_finished_evr.clear();
        return;
    }

    let ev = word_finished_evr.read().next().unwrap();
    let tile_coords = or_return!(coords_q.get(ev.0));
    let mut player_coords = or_return!(player_q.get_single_mut());
    *player_coords = *tile_coords;
}
