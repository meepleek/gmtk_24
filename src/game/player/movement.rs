use crate::{
    game::physics::{apply_gravity, check_grounded},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementIntent>()
        .add_systems(
            FixedUpdate,
            (horizontal_velocity_easing, process_intent)
                .chain()
                .after(check_grounded)
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

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub(crate) struct MovementEasing {
    ease_factor: f32,
    last_direction: f32,
    duration_in_s: f32,
    duration_out_s: f32,
}
impl MovementEasing {
    pub fn new(duration_in_s: f32, duration_out_s: f32) -> Self {
        Self {
            duration_in_s,
            duration_out_s,
            ease_factor: 0.,
            last_direction: 0.,
        }
    }
}

fn process_intent(
    mut movement_q: Query<
        (
            &mut Velocity,
            &Gravity,
            &mut MovementIntent,
            &mut Grounded,
            Option<&MovementEasing>,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    let (mut velocity, gravity, mut intent, mut grounded, easing) =
        or_return!(movement_q.get_single_mut());
    velocity.x = 150.0
        * easing.map_or(intent.horizontal_movement, |easing| {
            easing.last_direction * easing.ease_factor
        })
        * time.delta_seconds();
    match (intent.jump.state, intent.jump.last_pressed) {
        (_, Some(last_pressed))
            if last_pressed.as_millis() as usize <= JUMP_INPUT_BUFFER_MS
                && grounded.can_jump(1, COYOTE_TIME_MS) =>
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
        // fixme: does not work very well and the min jump seems to be way too low for e.g. min jump of 1 tile height
        (ButtonState::JustReleased | ButtonState::Released, _)
            if velocity.y > gravity.min_jump_velocity() =>
        {
            velocity.y *= 0.65;
        }
        // todo
        (_, _) => {} // ButtonState::Pressed => todo!(),
                     // ButtonState::JustReleased => todo!(),
                     // ButtonState::Released => todo!(),
                     // todo: variable jump height
    }
}

// todo: handle gamepad analog sticks?
fn horizontal_velocity_easing(
    mut easing_q: Query<(&MovementIntent, &mut MovementEasing), With<Player>>,
    time: Res<Time>,
) {
    for (intent, mut easing) in &mut easing_q {
        if intent.horizontal_movement != 0. {
            easing.last_direction = intent.horizontal_movement.signum();
        }
        easing.ease_factor = (easing.ease_factor
            + if intent.horizontal_movement == 0. {
                -time.delta_seconds() / easing.duration_out_s
            } else {
                time.delta_seconds() / easing.duration_in_s
            })
        .clamp(0., 1.);
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
