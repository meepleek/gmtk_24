use std::time::Duration;

use crate::prelude::*;
use avian2d::prelude::*;
use bevy::color::palettes::tailwind;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(avian2d::PhysicsPlugins::default())
        .register_type::<Velocity>()
        .register_type::<Gravity>()
        .add_systems(Update, (tick_cooldown::<Gravity>, add_tile_collider))
        .add_systems(
            FixedUpdate,
            (
                check_grounded,
                apply_gravity,
                apply_horizontal_velocity,
                apply_vertical_velocity,
            )
                .chain()
                .run_if(level_ready),
        )
        .observe(reset_velocity_on_grounded);
}

pub const SKIN_WIDTH: f32 = 1.0;

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
    pub ground_width: f32,
}
impl Default for Gravity {
    fn default() -> Self {
        Self {
            y: -7.,
            // delay_ms: Some(350),
            delay_ms: None,
            ground_width: TILE_SIZE as f32,
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct GroundSensor {
    pub width: f32,
    pub y: f32,
}
impl Default for GroundSensor {
    fn default() -> Self {
        Self {
            width: TILE_SIZE as f32,
            y: (TILE_SIZE / 2) as f32,
        }
    }
}

#[derive(Component, Default)]
pub(crate) struct Grounded {
    duration: Duration,
}

fn add_tile_collider(grounded_q: Query<Entity, Added<TileCollider>>, mut cmd: Commands) {
    for e in &grounded_q {
        cmd.entity(e)
            .try_insert(Collider::rectangle(TILE_SIZE as f32, TILE_SIZE as f32));
    }
}

fn check_grounded(
    mut grounded_q: Query<(Entity, &GroundSensor, &Transform, Option<&mut Grounded>)>,
    mut cmd: Commands,
    cast: SpatialQuery,
    mut gizmos: Gizmos,
    time: Res<Time>,
) {
    for (e, ground_sensor, t, grounded) in &mut grounded_q {
        let origin = Vec2::new(t.translation.x, t.translation.y + ground_sensor.y);
        gizmos.rect_2d(
            origin - Vec2::NEG_Y * SKIN_WIDTH,
            0.,
            Vec2::new(ground_sensor.width, SKIN_WIDTH),
            tailwind::GREEN_400,
        );
        // let size = Vec2::new(ground_sensor.width, 10.);
        if cast
            .shape_hits(
                // todo: get shape from caster? or just use a custom components with dimentions for ground checks etc.
                &Collider::segment(
                    Vec2::new(-ground_sensor.width / 2., -SKIN_WIDTH),
                    Vec2::new(ground_sensor.width / 2., -SKIN_WIDTH),
                ),
                origin,
                0.,
                // todo:
                Dir2::new(Vec2::NEG_Y).unwrap(),
                // todo:
                SKIN_WIDTH,
                u32::MAX,
                false,
                SpatialQueryFilter {
                    mask: GamePhysicsLayer::Obstacle.into(),
                    excluded_entities: [e].into(),
                },
            )
            .into_iter()
            .any(|hit| hit.normal1.y > 0.)
        {
            if let Some(mut grounded) = grounded {
                grounded.duration += time.delta();
            } else {
                cmd.entity(e).insert(Grounded::default());
            }
        } else if grounded.is_some() {
            cmd.entity(e).remove::<Grounded>();
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

// todo: interpolation - possibly using one of the interpolation crates
fn apply_vertical_velocity(
    mut vel_q: Query<
        (Entity, &Velocity, &mut Transform, &mut GridCoords),
        (Without<Grounded>, Without<Cooldown<Gravity>>),
    >,
    mut lookup: ResMut<LevelEntityLookup>,
    collision_q: Query<(), Or<(With<Ground>, With<UnbreakableGround>)>>,
) {
    for (e, vel, mut t, mut coords) in &mut vel_q {
        let new_y = t.translation.y + vel.y;
        let new_y_btm = new_y - TILE_SIZE as f32 / 2.;
        let new_coords = Vec3::new(t.translation.x, new_y_btm, 0.0).to_grid_coords();
        let mut update_y = None;

        if *coords != new_coords {
            if let Some(coll_e) = lookup.get(&new_coords) {
                if collision_q.contains(*coll_e) {
                    // snap to ground on collision
                    update_y = Some(coords.to_world().y);
                } else {
                    update_y = Some(new_y);
                }
            } else {
                // update coords on no collision
                update_y = Some(new_y);
            }
        } else {
            // no coords change, just update translation
            t.translation.y = new_y;
        }

        if let Some(y) = update_y {
            lookup.remove(&*coords);
            lookup.insert(new_coords, e);
            *coords = new_coords;
            t.translation.y = y;
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
