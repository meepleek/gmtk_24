use crate::prelude::*;

// todo: try to make rocks pushable to squish enemies?
// todo: also allow (some) enemies to push rocks too
pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<RockBundle>("Rock")
        .add_systems(Update, apply_gravity_on_tile_removed.run_if(level_ready))
        .observe(apply_gravity);
}

#[derive(Component, Default)]
pub(crate) struct Rock;

#[derive(Default, Bundle, LdtkEntity)]
struct RockBundle {
    rock: Rock,
    #[with(movable_rock)]
    movable: Movable,
    #[grid_coords]
    grid_coords: GridCoords,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

fn movable_rock(_entity: &EntityInstance) -> Movable {
    Movable {
        tween_duration_ms: 240,
        ..default()
    }
}

fn apply_gravity_on_tile_removed(
    mut word_tile_evr: EventReader<WordTileEvent>,
    lookup: Res<LevelEntityLookup>,
    mut rock_q: Query<&mut GridCoords, With<Rock>>,
) {
    for finished_tile_coords in word_tile_evr.read().filter_map(|ev| match ev.kind {
        WordTileEventKind::TileFinished { coords, .. } => Some(coords),
        _ => None,
    }) {
        let e = or_continue_quiet!(lookup.get(&finished_tile_coords.up()));
        let mut rock_coords = or_continue_quiet!(rock_q.get_mut(*e));
        *rock_coords = rock_coords.down();
    }
}

fn apply_gravity(
    trigger: Trigger<OnRemove, Moving>,
    mut lookup: ResMut<LevelEntityLookup>,
    mut rock_q: Query<&mut GridCoords, With<Rock>>,
    collision_q: Query<(), Or<(With<Ground>, With<UnbreakableGround>)>>,
) {
    let e = trigger.entity();
    let mut rock_coords = or_return_quiet!(rock_q.get_mut(e));
    let update;
    let new_coords = rock_coords.down();

    if *rock_coords == new_coords {
        lookup.remove(&rock_coords.up());
        return;
    }

    if let Some(e) = lookup.get(&new_coords) {
        update = !collision_q.contains(*e);
    } else {
        update = true;
    }
    lookup.remove(&rock_coords.up());
    if update {
        *rock_coords = new_coords;
        lookup.insert(new_coords, e);
    }
}
