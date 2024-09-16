use crate::prelude::*;
use avian2d::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(avian2d::PhysicsPlugins::default())
        .register_type::<Velocity>()
        .register_type::<Gravity>()
        .add_systems(Update, (tick_cooldown::<Gravity>, add_tile_collider))
        .add_systems(
            FixedUpdate,
            (
                reset_grounded_on_tile_removed,
                apply_gravity,
                apply_horizontal_velocity,
                apply_vertical_velocity,
            )
                .chain()
                .run_if(level_ready),
        )
        .observe(reset_velocity_on_grounded);
}

#[derive(PhysicsLayer)]
pub(crate) enum GamePhysicsLayer {
    Player,
    Obstacle,
}

impl GamePhysicsLayer {
    pub fn membership(member: GamePhysicsLayer) -> CollisionLayers {
        Self::memberships([member])
    }

    pub fn memberships<const N: usize>(members: [GamePhysicsLayer; N]) -> CollisionLayers {
        CollisionLayers::new(members, LayerMask::ALL)
    }

    pub fn obstacle_collision_layers() -> CollisionLayers {
        Self::membership(Self::Obstacle)
    }
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub(crate) struct TileCollider;

#[derive(Component, Default, Deref, DerefMut, Reflect, Debug)]
#[reflect(Component)]
pub(crate) struct Velocity(Vec2);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct Gravity {
    pub y: f32,
    pub delay_ms: Option<u64>,
}
impl Default for Gravity {
    fn default() -> Self {
        Self {
            y: -7.,
            // delay_ms: Some(350),
            delay_ms: None,
        }
    }
}

#[derive(Component, Default)]
pub(crate) struct Grounded;

fn add_tile_collider(grounded_q: Query<Entity, Added<TileCollider>>, mut cmd: Commands) {
    for e in &grounded_q {
        cmd.entity(e)
            .try_insert(Collider::rectangle(TILE_SIZE as f32, TILE_SIZE as f32));
    }
}

fn reset_grounded_on_tile_removed(
    mut word_tile_evr: EventReader<WordTileEvent>,
    lookup: Res<LevelEntityLookup>,
    grounded_q: Query<&Gravity, With<Grounded>>,
    mut cmd: Commands,
) {
    for finished_tile_coords in word_tile_evr.read().filter_map(|ev| match ev.kind {
        WordTileEventKind::TileFinished { coords, .. } => Some(coords),
        _ => None,
    }) {
        let e = or_continue_quiet!(lookup.get(&finished_tile_coords.up()));
        let gravity = or_continue_quiet!(grounded_q.get(*e));
        let mut e_cmd = cmd.entity(*e);
        e_cmd.remove::<Grounded>();
        if let Some(delay) = gravity.delay_ms {
            e_cmd.try_insert(Cooldown::<Gravity>::new(delay));
        }
    }
}

fn reset_velocity_on_grounded(
    trigger: Trigger<OnAdd, Grounded>,
    mut velocity_q: Query<&mut Velocity>,
) {
    let mut vel = or_return!(velocity_q.get_mut(trigger.entity()));
    vel.y = 0.0;
}

fn apply_gravity(
    mut gravity_q: Query<
        (&Gravity, &mut Velocity),
        (Without<Grounded>, Without<Cooldown<Gravity>>),
    >,
    time: Res<Time>,
) {
    for (gravity, mut vel) in &mut gravity_q {
        vel.y += gravity.y * time.delta_seconds();
    }
}

// todo: extrapolation
fn apply_vertical_velocity(
    mut vel_q: Query<
        (Entity, &Velocity, &mut Transform, &mut GridCoords),
        (Without<Grounded>, Without<Cooldown<Gravity>>),
    >,
    mut lookup: ResMut<LevelEntityLookup>,
    collision_q: Query<(), Or<(With<Ground>, With<UnbreakableGround>)>>,
    mut cmd: Commands,
) {
    for (e, vel, mut t, mut coords) in &mut vel_q {
        let new_y = t.translation.y + vel.y;
        let new_y_btm = new_y - TILE_SIZE as f32 / 2.;
        let new_coords = Vec3::new(t.translation.x, new_y_btm, 0.0).to_grid_coords();
        let mut should_ground = false;
        let mut update_coords = false;
        if new_y_btm < 0. {
            // transition to grounded at bounds
            should_ground = true;
        } else if *coords != new_coords {
            if let Some(coll_e) = lookup.get(&new_coords) {
                if collision_q.contains(*coll_e) {
                    // transition to grounded on collision
                    should_ground = true;
                } else {
                    // update coords on no collision
                    update_coords = true;
                }
            } else {
                // update coords on no collision
                update_coords = true;
            }
        } else {
            // no coords change, just update translation
            t.translation.y = new_y;
        }

        if should_ground {
            cmd.entity(e).try_insert(Grounded);
            t.translation.y = coords.to_world().y;
        } else if update_coords {
            lookup.remove(&*coords);
            lookup.insert(new_coords, e);
            *coords = new_coords;
            t.translation.y = new_y;
        }
    }
}

fn apply_horizontal_velocity(mut vel_q: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut t) in &mut vel_q {
        let new_x = t.translation.x + vel.x;
        // todo: handle collisions here?
        t.translation.x = new_x;
        if vel.x != 0.0 {
            t.scale.x = vel.x.signum();
        }
    }
}
