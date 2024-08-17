use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>()
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_systems(Update, (process_typed_input, move_player));
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
) {
    if let Some(move_by) = match typed.as_str() {
        "a" => Some(GridCoords::new(-1, 0)),
        "d" => Some(GridCoords::new(1, 0)),
        "w" => Some(GridCoords::new(0, 1)),
        "s" => Some(GridCoords::new(0, -1)),
        _ => {
            // todo?
            typed.clear();
            None
        }
    } {
        typed.clear();
        let mut coords = or_return!(player_q.get_single_mut());
        let new_coords = *coords + move_by;
        *coords = new_coords;
    }
}

fn move_player(
    player_q: Query<(Entity, &GridCoords), (With<Player>, Changed<GridCoords>)>,
    mut cmd: Commands,
) {
    for (e, coord) in &player_q {
        cmd.tween_translation(e, coord.to_world(), 110, EaseFunction::QuadraticOut);
    }
}
