use crate::prelude::*;
use leafwing_input_manager::buttonlike::ButtonState;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementIntent>()
        .add_systems(FixedUpdate, process_intent.run_if(level_ready));
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub(crate) struct MovementIntent {
    pub horizontal_direction: f32,
    pub jump: ButtonState,
}

fn process_intent(
    mut player_q: Query<
        (
            Entity,
            &mut Velocity,
            &Gravity,
            &MovementIntent,
            Has<Grounded>,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    mut cmd: Commands,
) {
    let (e, mut velocity, gravity, intent, is_grounded) = or_return!(player_q.get_single_mut());
    velocity.x = 150.0 * intent.horizontal_direction * time.delta_seconds();
    match intent.jump {
        ButtonState::JustPressed if is_grounded => {
            velocity.y = gravity.jump_velocity();
            cmd.entity(e).remove::<Grounded>();
        }
        // ButtonState::Pressed => todo!(),
        // ButtonState::JustReleased => todo!(),
        // ButtonState::Released => todo!(),
        // todo: variable jump height
        _ => {}
    }
}
