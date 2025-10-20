use crate::prelude::*;

// todo: try to make rocks pushable to squish enemies?
// todo: also allow (some) enemies to push rocks too
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
    #[sprite_sheet]
    sprite: Sprite,
}
