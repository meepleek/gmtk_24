use crate::prelude::*;
use avian2d::prelude::*;
use std::{ops::RangeInclusive, time::Duration};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(avian2d::PhysicsPlugins::default())
        .register_type::<Velocity>()
        .register_type::<Gravity>()
        .register_type::<Grounded>()
        .add_systems(Update, add_tile_collider)
        .add_systems(
            FixedUpdate,
            (
                check_grounded,
                check_horizontal_collisions,
                apply_gravity,
                apply_horizontal_velocity,
                apply_vertical_velocity,
            )
                .in_set(AppSet::Update)
                .chain()
                .run_if(level_ready),
        );
}

pub const FIXED_UPDATE_FPS: f32 = 64.0;
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
pub(crate) struct Velocity(pub Vec2);
impl Velocity {
    #[expect(dead_code)]
    pub fn falling(&self) -> bool {
        self.y < 0.
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct Gravity {
    gravity: f32,
    jump_velocity: f32,
    min_jump_velocity: f32,
    max_fall_velocity: f32,
    ground_width: f32,
}
impl Default for Gravity {
    fn default() -> Self {
        Self::new(0.5..=1.27, 0.3, TILE_SIZE as f32)
    }
}
impl Gravity {
    pub fn new(
        jump_height: RangeInclusive<f32>,
        jump_to_apex_duration_sec: f32,
        ground_width: f32,
    ) -> Self {
        let tile_unit_size = TILE_SIZE as f32 / FIXED_UPDATE_FPS;
        let accel = (2.0 * jump_height.end() * tile_unit_size) / jump_to_apex_duration_sec.powi(2);
        let jump_velocity = accel * jump_to_apex_duration_sec;
        Self {
            gravity: -accel,
            jump_velocity,
            min_jump_velocity: jump_velocity * (jump_height.start() / jump_height.end()).sqrt(),
            max_fall_velocity: -(accel * TILE_SIZE as f32) / FIXED_UPDATE_FPS,
            ground_width,
        }
    }

    pub fn jump_velocity(&self) -> f32 {
        self.jump_velocity
    }

    pub fn min_jump_velocity(&self) -> f32 {
        self.min_jump_velocity
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct KinematicSensor {
    pub size: Vec2,
    pub ground_y_offset: f32,
}
impl Default for KinematicSensor {
    fn default() -> Self {
        Self {
            size: Vec2::splat(TILE_SIZE as f32 / 2.),
            ground_y_offset: 0.,
        }
    }
}
impl KinematicSensor {
    pub fn translation(&self, transform_translation: Vec3) -> Vec2 {
        transform_translation.truncate() - Vec2::Y * self.ground_y_offset
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub(crate) enum Grounded {
    #[default]
    Grounded,
    Airborne {
        duration: Duration,
        jump_count: u8,
    },
}
impl Grounded {
    pub fn airborne(jump_count: u8) -> Self {
        Grounded::Airborne {
            duration: Duration::default(),
            jump_count,
        }
    }

    pub fn is_grounded(&self) -> bool {
        matches!(self, Grounded::Grounded)
    }

    pub fn is_airborne(&self) -> bool {
        matches!(self, Grounded::Airborne { .. })
    }

    pub fn can_jump(&self, max_jump_count: u8, coyote_time_ms: usize) -> bool {
        match self {
            Grounded::Grounded => true,
            Grounded::Airborne {
                duration,
                jump_count,
            } => *jump_count < max_jump_count && duration.as_millis() as usize <= coyote_time_ms,
        }
    }
}

#[derive(Reflect, Debug)]
pub(crate) enum ClosestHorizontalCollision {
    Left(f32),
    Right(f32),
}

#[derive(Component, Default, Reflect, Debug, Deref, DerefMut)]
#[reflect(Component)]
pub(crate) struct HorizontalObstacleDetection(Option<ClosestHorizontalCollision>);
impl HorizontalObstacleDetection {
    pub fn new(distance_left: Option<f32>, distance_right: Option<f32>) -> Self {
        use ClosestHorizontalCollision::{Left, Right};
        match (distance_left, distance_right) {
            (Some(left), None) => Self(Some(Left(left))),
            (None, Some(right)) => Self(Some(Right(right))),
            (Some(left), Some(right)) => {
                if left < right {
                    Self(Some(Left(left)))
                } else {
                    Self(Some(Right(right)))
                }
            }
            (None, None) => Self(None),
        }
    }

    pub fn closest_sign(&self) -> Option<f32> {
        match self.0 {
            Some(ClosestHorizontalCollision::Left(_)) => Some(-1.),
            Some(ClosestHorizontalCollision::Right(_)) => Some(1.),
            _ => None,
        }
    }
}

fn add_tile_collider(grounded_q: Query<Entity, Added<TileCollider>>, mut cmd: Commands) {
    for e in &grounded_q {
        cmd.entity(e)
            .try_insert(Collider::rectangle(TILE_SIZE as f32, TILE_SIZE as f32));
    }
}

pub(crate) fn check_grounded(
    mut grounded_q: Query<(Entity, &KinematicSensor, &Transform, &mut Grounded)>,
    cast: SpatialQuery,
    time: Res<Time>,
) {
    for (e, sensor, t, mut grounded) in &mut grounded_q {
        let sensor_half_size = sensor.size / 2. - Vec2::splat(SKIN_WIDTH);
        let origin = Vec2::new(
            t.translation.x,
            t.translation.y - sensor_half_size.y - sensor.ground_y_offset,
        );
        if cast
            .shape_hits(
                &Collider::segment(
                    Vec2::new(-sensor_half_size.x, 0.),
                    Vec2::new(sensor_half_size.x, 0.),
                ),
                origin,
                0.,
                Dir2::new(Vec2::NEG_Y).unwrap(),
                SKIN_WIDTH,
                u32::MAX,
                false,
                SpatialQueryFilter {
                    mask: GamePhysicsLayer::Obstacle.into(),
                    excluded_entities: [e].into(),
                },
            )
            .into_iter()
            // hit coming from below
            .any(|hit| hit.normal1.y > 0.)
        {
            *grounded = Grounded::Grounded;
        } else {
            match grounded.as_mut() {
                Grounded::Grounded => {
                    *grounded = Grounded::airborne(0);
                }
                Grounded::Airborne { duration, .. } => {
                    *duration += time.delta();
                }
            }
        }
    }
}

pub(crate) fn check_horizontal_collisions(
    mut grounded_q: Query<(
        Entity,
        &KinematicSensor,
        &Transform,
        &mut HorizontalObstacleDetection,
    )>,
    cast: SpatialQuery,
) {
    for (e, sensor, t, mut coll) in &mut grounded_q {
        let distance = |sign: f32| {
            let sensor_half_size = sensor.size / 2. - Vec2::splat(SKIN_WIDTH);
            let origin = Vec2::new(t.translation.x + sensor_half_size.x * sign, t.translation.y);
            cast.shape_hits(
                &Collider::segment(
                    Vec2::new(0., -sensor_half_size.y),
                    Vec2::new(0., sensor_half_size.y),
                ),
                origin,
                0.,
                Dir2::new(Vec2::X * sign).expect("Valid direction"),
                TILE_SIZE as f32 * 0.25 + SKIN_WIDTH,
                u32::MAX,
                false,
                SpatialQueryFilter {
                    mask: GamePhysicsLayer::Obstacle.into(),
                    excluded_entities: [e].into(),
                },
            )
            .into_iter()
            // horizontal hit
            .filter(|hit| hit.normal1.x != 0.)
            .min_by(|hit1, hit2| {
                hit1.time_of_impact
                    .partial_cmp(&hit2.time_of_impact)
                    .expect("Valid TOI")
            })
            .map(|h| h.time_of_impact - SKIN_WIDTH)
        };
        *coll = HorizontalObstacleDetection::new(distance(-1.), distance(1.));
    }
}

pub(crate) fn apply_gravity(
    mut gravity_q: Query<(&Gravity, &mut Velocity, &Grounded, &MovementIntent)>,
    time: Res<Time>,
) {
    for (gravity, mut vel, grounded, movement_intent) in &mut gravity_q {
        vel.y = if grounded.is_grounded() {
            0.
        } else {
            let gravity_factor = if movement_intent.jump.state.pressed()
                && vel.y.abs() <= (gravity.jump_velocity() * 0.125)
            {
                // lower gravity at jump apex
                0.5
            } else {
                1.
            };
            (vel.y + gravity.gravity * gravity_factor * time.delta_seconds())
                .max(gravity.max_fall_velocity)
        };
    }
}

// todo: should I use verlet integration instead of euler even when using fixed schedule?
// todo: interpolation - possibly using one of the interpolation crates
fn apply_vertical_velocity(
    mut vel_q: Query<(
        Entity,
        &Grounded,
        &mut Velocity,
        &mut Transform,
        &KinematicSensor,
    )>,
    cast: SpatialQuery,
) {
    for (e, _, mut vel, mut t, sensor) in &mut vel_q
        .iter_mut()
        .filter(|(_, grounded, ..)| !grounded.is_grounded())
    {
        if vel.y == 0. {
            continue;
        }

        let move_by_y = match cast
            .shape_hits(
                // todo: get shape from caster? or just use a custom components with dimentions for ground checks etc.
                &Collider::rectangle(
                    sensor.size.x - SKIN_WIDTH * 2.,
                    sensor.size.y - SKIN_WIDTH * 2.,
                ),
                sensor.translation(t.translation),
                0.,
                Dir2::new(Vec2::Y * vel.y).expect("Non-zero y velocity"),
                vel.y.abs() + SKIN_WIDTH,
                u32::MAX,
                false,
                SpatialQueryFilter {
                    mask: GamePhysicsLayer::Obstacle.into(),
                    excluded_entities: [e].into(),
                },
            )
            .into_iter()
            .filter(|hit| hit.normal1.y != 0.)
            .min_by(|hit1, hit2| {
                hit1.time_of_impact
                    .partial_cmp(&hit2.time_of_impact)
                    .expect("Valid TOI")
            }) {
            Some(hit) => {
                if vel.y > 0. && hit.normal1.y < 0. {
                    // reset vertical velocity when hitting a ceiling
                    vel.y = 0.;
                }

                (hit.time_of_impact - SKIN_WIDTH).max(0.) * vel.y.signum()
            }
            None => vel.y,
        };

        if move_by_y != 0. {
            t.translation.y += move_by_y;
        }
    }
}

fn apply_horizontal_velocity(
    mut vel_q: Query<(Entity, &Velocity, &KinematicSensor, &mut Transform)>,
    cast: SpatialQuery,
    time: Res<Time>,
) {
    for (e, vel, sensor, mut t) in &mut vel_q {
        let x = vel.x * time.delta_seconds();
        if x != 0.0 {
            t.scale.x = x.signum();
        } else {
            continue;
        }

        let move_by_x = match cast
            .shape_hits(
                // todo: get shape from caster? or just use a custom components with dimentions for ground checks etc.
                &Collider::rectangle(
                    sensor.size.x - SKIN_WIDTH * 2.,
                    sensor.size.y - SKIN_WIDTH * 2.,
                ),
                sensor.translation(t.translation),
                0.,
                Dir2::new(Vec2::X * x.signum()).expect("Non-zero y velocity"),
                x.abs() + SKIN_WIDTH,
                u32::MAX,
                false,
                SpatialQueryFilter {
                    mask: GamePhysicsLayer::Obstacle.into(),
                    excluded_entities: [e].into(),
                },
            )
            .into_iter()
            .filter(|hit| hit.normal1.x != 0.)
            .min_by(|hit1, hit2| {
                hit1.time_of_impact
                    .partial_cmp(&hit2.time_of_impact)
                    .expect("Valid TOI")
            }) {
            Some(hit) => (hit.time_of_impact - SKIN_WIDTH).max(0.) * x.signum(),
            None => x,
        };

        if move_by_x != 0. {
            t.translation.x += move_by_x;
        }
    }
}
