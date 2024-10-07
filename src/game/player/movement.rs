use crate::{
    anim::StableInterpolate,
    game::physics::{apply_gravity, check_horizontal_collisions},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementIntent>()
        .add_systems(
            FixedUpdate,
            (process_intent)
                .chain()
                .after(check_horizontal_collisions)
                .before(apply_gravity)
                .run_if(level_ready),
        )
        .add_systems(
            FixedUpdate,
            update_grid_coords
                .in_set(AppSet::UpdateCoords)
                .run_if(level_ready),
        );
}

pub const COYOTE_TIME_MS: usize = 90;
pub const JUMP_INPUT_BUFFER_MS: usize = 80;

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub(crate) struct MovementIntent {
    pub horizontal_movement: f32,
    pub jump: TimedButtonInput,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub(crate) struct MovementEasing {
    pub decay_ground: f32,
    pub decay_airborn: f32,
}
impl Default for MovementEasing {
    fn default() -> Self {
        Self {
            decay_ground: 0.05f32.recip(),
            decay_airborn: 0.1f32.recip(),
        }
    }
}

// todo: need (horizontal) collisions here
fn process_intent(
    mut movement_q: Query<(
        &mut Velocity,
        &Gravity,
        &mut MovementIntent,
        &mut Grounded,
        Option<&MovementEasing>,
        Option<&HorizontalObstacleDetection>,
    )>,
    time: Res<Time>,
) {
    let (mut velocity, gravity, mut intent, mut grounded, easing, horizontal_obstacles) =
        or_return!(movement_q.get_single_mut());
    // todo: need to lerp/interpolate to target velocity instead of directly snapping to it (or just using the ease timing as a factor)
    // todo: component/add to controller component?
    let speed = 150.;
    let target = speed * intent.horizontal_movement;
    match &easing {
        Some(easing) => velocity.x.smooth_nudge(
            &target,
            if grounded.is_grounded() {
                easing.decay_ground
            } else {
                easing.decay_airborn
            },
            time.delta_seconds(),
        ),
        None => velocity.x = target,
    };
    // wall-jump
    // todo: add leeway - store last collision time to treat it as coyote_time when there's no side collision
    if let (ButtonState::JustPressed, Some(horizontal)) = (intent.jump.state, horizontal_obstacles)
        && grounded.is_airborne()
        && horizontal.closest_sign().is_some()
    {
        // todo: store when the horizontal collision has changed or similar to prevent further jupming until the player re-enters the collision os is grounded (or smt similar?)
        if intent.horizontal_movement != 0.
            && intent.horizontal_movement.signum() == horizontal.closest_sign().unwrap()
        {
            velocity.0 = Vec2::new(speed * 1.5 * -horizontal.closest_sign().unwrap(), 4.5);
        } else {
            velocity.0 = Vec2::new(speed * 1.8 * -horizontal.closest_sign().unwrap(), 4.);
        }
    }
    // todo: wall-sliding (when holding horizontal direction towards wall without jumping)

    // jump
    else if let Some(last_pressed) = intent.jump.last_pressed
        && last_pressed.as_millis() as usize <= JUMP_INPUT_BUFFER_MS
        && grounded.can_jump(1, COYOTE_TIME_MS)
    {
        intent.jump.last_pressed = None;
        velocity.y = gravity.jump_velocity();
        match grounded.as_mut() {
            Grounded::Grounded => *grounded = Grounded::airborne(1),
            Grounded::Airborne {
                duration,
                jump_count,
            } => {
                *duration += time.delta();
                *jump_count += 1;
            }
        }
    }
    // variable jump height
    // fixme: does not work very well and the min jump seems to be way too low for e.g. min jump of 1 tile height
    else if let ButtonState::JustReleased | ButtonState::Released = intent.jump.state
        && velocity.y > gravity.min_jump_velocity()
    {
        velocity.y *= 0.65;
    }
}

fn update_grid_coords(
    mut kinematic_q: Query<
        (Entity, &mut Transform, &mut GridCoords),
        (With<KinematicSensor>, Changed<Transform>),
    >,
    mut lookup: ResMut<LevelEntityLookup>,
) {
    for (e, t, mut coords) in &mut kinematic_q {
        let new_coords = t.translation.to_grid_coords();
        if *coords != new_coords {
            lookup.upsert(e, &coords, new_coords);
            *coords = new_coords;
        }
    }
}
