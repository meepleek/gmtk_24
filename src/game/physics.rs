use crate::prelude::*;

// todo: try to make rocks pushable to squish enemies?
// todo: also allow (some) enemies to push rocks too
pub(super) fn plugin(app: &mut App) {
    app.register_type::<Velocity>()
        .register_type::<Gravity>()
        .add_systems(
            FixedUpdate,
            (
                reset_grounded_on_tile_removed,
                apply_gravity,
                apply_velocity,
            )
                .chain()
                .run_if(level_ready),
        )
        .observe(reset_velocity_on_grounded);
}

#[derive(Component, Default, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub(crate) struct Velocity(Vec2);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct Gravity(f32);
impl Default for Gravity {
    fn default() -> Self {
        Self(-20.)
    }
}

#[derive(Component, Default)]
pub(crate) struct Grounded;

fn reset_grounded_on_tile_removed(
    mut word_tile_evr: EventReader<WordTileEvent>,
    lookup: Res<LevelEntityLookup>,
    rock_q: Query<(), With<Grounded>>,
    mut cmd: Commands,
) {
    for finished_tile_coords in word_tile_evr.read().filter_map(|ev| match ev.kind {
        WordTileEventKind::TileFinished { coords, .. } => Some(coords),
        _ => None,
    }) {
        let e = or_continue_quiet!(lookup.get(&finished_tile_coords.up()));
        or_continue_quiet!(rock_q.contains(*e));
        cmd.entity(*e).remove::<Grounded>();
    }
}

fn reset_velocity_on_grounded(
    trigger: Trigger<OnRemove, Grounded>,
    mut velocity_q: Query<&mut Velocity>,
) {
    let mut vel = or_return!(velocity_q.get_mut(trigger.entity()));
    vel.y = 0.0;
}

fn apply_gravity(
    mut gravity_q: Query<(&Gravity, &mut Velocity), Without<Grounded>>,
    time: Res<Time>,
) {
    for (gravity, mut vel) in &mut gravity_q {
        vel.y += gravity.0 * time.delta_seconds();
    }
}

// todo: extrapolation
fn apply_velocity(
    mut vel_q: Query<(Entity, &Velocity, &mut Transform, &mut GridCoords), Without<Grounded>>,
    mut lookup: ResMut<LevelEntityLookup>,
    collision_q: Query<(), Or<(With<Ground>, With<UnbreakableGround>)>>,
    mut cmd: Commands,
) {
    for (e, vel, mut t, mut coords) in &mut vel_q {
        let new_translation = t.translation + vel.0.extend(0.) - Vec3::Y * (TILE_SIZE as f32 / 2.);
        let new_coords = new_translation.to_grid_coords();
        if *coords != new_coords {
            let update;
            if let Some(coll_e) = lookup.get(&new_coords) {
                if collision_q.contains(*coll_e) {
                    cmd.entity(e).try_insert(Grounded);
                    t.translation = coords.to_world();
                    continue;
                } else {
                    update = true;
                }
            } else {
                update = true;
            }
            if update {
                lookup.remove(&*coords);
                lookup.insert(new_coords, e);
                *coords = new_coords;
                t.translation += vel.0.extend(0.);
            }
        } else {
            t.translation += vel.0.extend(0.);
        }
    }
}
