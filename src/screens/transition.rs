use bevy::ecs::system::RunSystemOnce;
use bevy_tweening::AnimTargetKind;

use crate::{camera::BACKGROUND_COLOR, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<ScreenTransition>()
        .add_systems(Startup, setup_transition_overlay)
        .add_systems(
            Update,
            (
                start_transition_out.run_if(state_changed::<ScreenTransition>),
                start_transition_in,
            ),
        )
        .insert_resource(TransitionSpeedFactor(if cfg!(feature = "dev") {
            0.5
        } else {
            1.0
        }));
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum ScreenTransition {
    #[default]
    Done,
    TransitioningOut(Screen),
    TransitioningIn,
}

#[derive(Debug)]
pub struct TransitionScreen(Screen);
impl Command for TransitionScreen {
    fn apply(self, world: &mut World) {
        world
            .run_system_once_with(
                |In(trans_screen): In<TransitionScreen>,
                 mut next: ResMut<NextState<ScreenTransition>>| {
                    next.set(ScreenTransition::TransitioningOut(trans_screen.0));
                },
                self,
            )
            .expect("transition screen");
    }
}

pub(crate) trait TransitionScreenCommandExt {
    fn transition_to_screen(&mut self, next_screen: Screen);
}

impl<'w, 's> TransitionScreenCommandExt for Commands<'w, 's> {
    fn transition_to_screen(&mut self, next_screen: Screen) {
        self.queue(TransitionScreen(next_screen));
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct TransitionSpeedFactor(pub f32);

impl TransitionSpeedFactor {
    pub fn duration(&self, base_duration: u64) -> u64 {
        (base_duration as f32 * self.0) as u64
    }
}

#[derive(Component)]
struct TransitionImage;

fn setup_transition_overlay(mut cmd: Commands, speed_factor: Res<TransitionSpeedFactor>) {
    cmd.spawn((
        Name::new("transition"),
        // ImageNode::new(),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Vw(100.),
            height: Val::Vw(100.),
            ..default()
        },
        BackgroundColor(BACKGROUND_COLOR.into()),
        TransitionImage,
    ))
    .tween_to(
        UiBgColorLensEnd::new(BACKGROUND_COLOR.with_alpha(0.0)),
        speed_factor.duration(800),
    )
    .delay_ms(speed_factor.duration(300))
    .spawn();
}

fn start_transition_out(
    next_transition_state: ResMut<State<ScreenTransition>>,
    mut cmd: Commands,
    transition_img_q: Query<Entity, With<TransitionImage>>,
    speed_factor: Res<TransitionSpeedFactor>,
) {
    if !matches!(
        next_transition_state.get(),
        ScreenTransition::TransitioningOut(_)
    ) {
        return;
    }

    let e = or_return!(transition_img_q.single());
    or_return!(cmd.tween_to(
        e,
        UiBgColorLensEnd(BACKGROUND_COLOR),
        speed_factor.duration(600),
    ))
    .spawn();
}

fn start_transition_in(
    screen_trans: Res<State<ScreenTransition>>,
    mut next_screen_trans: ResMut<NextState<ScreenTransition>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut cmd: Commands,
    mut tween_msg_r: MessageReader<AnimCompletedEvent>,
    transition_img_q: Query<Entity, With<TransitionImage>>,
    speed_factor: Res<TransitionSpeedFactor>,
) {
    if let ScreenTransition::TransitioningOut(screen) = screen_trans.get() {
        let e = or_return_quiet!(tween_msg_r.read().find_map(|ev| {
            if let AnimTargetKind::Component { entity } = ev.target
                && transition_img_q.contains(entity)
            {
                return Some(entity);
            }
            None
        }));

        next_screen_trans.set(ScreenTransition::TransitioningIn);
        next_screen.set(screen.clone());

        or_return!(cmd.tween_to(
            e,
            UiBgColorLensEnd(BACKGROUND_COLOR.with_alpha(0.0)),
            speed_factor.duration(600),
        ))
        .spawn();
    }
}
