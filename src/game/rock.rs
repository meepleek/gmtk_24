use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<RockBundle>("Rock");
}

#[derive(Component, Default)]
pub(crate) struct Rock;

#[derive(Default, Bundle, LdtkEntity)]
struct RockBundle {
    rock: Rock,
    gravity: Gravity,
    grounded: Grounded,
    velocity: Velocity,
    #[grid_coords]
    grid_coords: GridCoords,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}
