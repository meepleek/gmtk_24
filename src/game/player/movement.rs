use crate::prelude::*;
use leafwing_input_manager::buttonlike::ButtonState;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementIntent>()
        .add_systems(FixedUpdate, process_intent.run_if(level_ready));
}

pub const COYOTE_TIME_MS: usize = 110;

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub(crate) struct MovementIntent {
    pub horizontal_direction: f32,
    pub jump: ButtonState,
}

fn process_intent(
    mut player_q: Query<(&mut Velocity, &Gravity, &MovementIntent, &mut Grounded), With<Player>>,
    time: Res<Time>,
) {
    let (mut velocity, gravity, intent, mut grounded) = or_return!(player_q.get_single_mut());
    velocity.x = 150.0 * intent.horizontal_direction * time.delta_seconds();
    match intent.jump {
        ButtonState::JustPressed if grounded.can_jump(1, COYOTE_TIME_MS) => {
            velocity.y = gravity.jump_velocity();
            match grounded.as_mut() {
                Grounded::Grounded => *grounded = Grounded::airborne(),
                Grounded::Airborne {
                    duration,
                    jump_count,
                } => {
                    *duration += time.delta();
                    *jump_count += 1;
                }
            }
        }
        // ButtonState::Pressed => todo!(),
        // ButtonState::JustReleased => todo!(),
        // ButtonState::Released => todo!(),
        // todo: variable jump height
        _ => {}
    }
}
