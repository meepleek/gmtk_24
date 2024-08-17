use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>()
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_systems(Update, (process_typed_input, move_player));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

fn process_typed_input(
    mut typed: ResMut<TypedInput>,
    mut player_q: Query<&mut Coordinate, With<Player>>,
) {
    if let Some(move_by) = match typed.as_str() {
        "a" => Some(IVec2::NEG_X),
        "d" => Some(IVec2::X),
        "w" => Some(IVec2::Y),
        "s" => Some(IVec2::NEG_Y),
        _ => {
            // todo?
            typed.clear();
            None
        }
    } {
        typed.clear();
        let mut coord = or_return!(player_q.get_single_mut());
        coord.0 += move_by;
    }
}

fn move_player(
    player_q: Query<(Entity, &Coordinate), (With<Player>, Changed<Coordinate>)>,
    mut cmd: Commands,
) {
    for (e, coord) in &player_q {
        cmd.tween_translation(e, coord.to_world(), 110, EaseFunction::QuadraticOut);
    }
}
