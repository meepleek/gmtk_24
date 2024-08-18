use crate::prelude::*;

use super::level::TileWord;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>()
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_systems(
            Update,
            (process_typed_input, tween_player_movement).run_if(level_ready),
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
