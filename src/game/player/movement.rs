use crate::{
    game::physics::{apply_gravity, check_grounded},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementIntent>().add_systems(
        FixedUpdate,
        process_intent
            .after(check_grounded)
            .before(apply_gravity)
            .run_if(level_ready),
    );
}

pub const COYOTE_TIME_MS: usize = 90;
pub const JUMP_INPUT_BUFFER_MS: usize = 80;

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub(crate) struct MovementIntent {
    pub horizontal_direction: f32,
    pub jump: TimedButtonInput,
}

fn process_intent(
    mut player_q: Query<
        (&mut Velocity, &Gravity, &mut MovementIntent, &mut Grounded),
        With<Player>,
    >,
    time: Res<Time>,
) {
    let (mut velocity, gravity, mut intent, mut grounded) = or_return!(player_q.get_single_mut());
    velocity.x = 150.0 * intent.horizontal_direction * time.delta_seconds();
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
        // todo
        (_, _) => {} // ButtonState::Pressed => todo!(),
                     // ButtonState::JustReleased => todo!(),
                     // ButtonState::Released => todo!(),
                     // todo: variable jump height
    }
}
