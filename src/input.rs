use crate::prelude::*;
use bevy::input::{
    keyboard::{Key, KeyboardInput},
    ButtonState,
};
use leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .init_resource::<ActionState<PlayerAction>>()
        .insert_resource(PlayerAction::input_map())
        .add_plugins(InputManagerPlugin::<UiAction>::default())
        .init_resource::<ActionState<UiAction>>()
        .insert_resource(UiAction::input_map())
        .init_resource::<TypedInput>()
        .add_systems(Update, (text_input).in_set(AppSet::RecordInput));
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Jump,
}

impl Actionlike for PlayerAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            PlayerAction::Move => InputControlKind::DualAxis,
            _ => InputControlKind::Button,
        }
    }
}

impl PlayerAction {
    fn input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // gamepad
        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);
        input_map.insert_dual_axis(Self::Move, GamepadVirtualDPad::DPAD);
        input_map.insert(Self::Jump, GamepadButtonType::South);

        // MKB
        input_map.insert_dual_axis(Self::Move, KeyboardVirtualDPad::WASD);
        input_map.insert_dual_axis(Self::Move, KeyboardVirtualDPad::ARROW_KEYS);
        input_map.insert(Self::Jump, MouseButton::Left);

        input_map
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum UiAction {
    Move,
    Select,
    Reset,
    Back,
}

impl Actionlike for UiAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            UiAction::Move => InputControlKind::Axis,
            _ => InputControlKind::Button,
        }
    }
}

impl UiAction {
    fn input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // gamepad
        input_map
            .insert_axis(
                Self::Move,
                GamepadControlAxis::RIGHT_Y.with_deadzone_symmetric(0.25),
            )
            .insert_axis(
                Self::Move,
                GamepadControlAxis::LEFT_Y.with_deadzone_symmetric(0.25),
            )
            .insert_axis(Self::Move, GamepadVirtualAxis::DPAD_Y)
            .insert(Self::Select, GamepadButtonType::South)
            .insert(Self::Back, GamepadButtonType::East)
            .insert(Self::Reset, GamepadButtonType::Start);

        // MKB
        input_map
            .insert_axis(Self::Move, KeyboardVirtualAxis::WS)
            .insert_axis(Self::Move, KeyboardVirtualAxis::VERTICAL_ARROW_KEYS)
            .insert(Self::Select, KeyCode::Space)
            .insert(Self::Select, KeyCode::Enter)
            .insert(Self::Back, KeyCode::Escape)
            .insert(Self::Reset, KeyCode::F5);

        input_map
    }
}

#[allow(dead_code)]
pub(crate) type PlayerInput<'a> = Res<'a, ActionState<PlayerAction>>;
#[allow(dead_code)]
pub(crate) type UiInput<'a> = Res<'a, ActionState<UiAction>>;

#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub(crate) struct TypedInput(pub(crate) String);

fn text_input(mut evr_kbd: EventReader<KeyboardInput>, mut typed: ResMut<TypedInput>) {
    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Released {
            continue;
        }
        match &ev.logical_key {
            Key::Enter => {
                println!("Text input: {}", typed.0);
            }
            // todo: handle DEL too?
            Key::Backspace => {
                typed.pop();
            }
            Key::Character(input) => {
                if input.chars().any(|c| c.is_control()) {
                    continue;
                }
                typed.push_str(&input.to_lowercase());
            }
            _ => {}
        }
    }
}
