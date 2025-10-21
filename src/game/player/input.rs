use crate::prelude::*;
use bevy::input::keyboard::{Key, KeyboardInput};
use leafwing_input_manager::prelude::*;
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PlayerBindings>()
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .init_resource::<ActionState<PlayerAction>>()
        .add_plugins(InputManagerPlugin::<UiAction>::default())
        .init_resource::<ActionState<UiAction>>()
        .insert_resource(UiAction::input_map())
        .add_systems(
            Update,
            update_player_input_map.run_if(resource_changed::<PlayerBindings>),
        )
        .add_systems(
            FixedUpdate,
            (
                collect_intent.in_set(AppSet::CollectInput),
                process_text_input.in_set(AppSet::Update),
            )
                .run_if(level_ready),
        );
}

#[derive(Resource, Reflect)]
pub struct PlayerBindings {
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
}

impl Default for PlayerBindings {
    fn default() -> Self {
        Self {
            // left: "a".to_string(),
            // right: "d".to_string(),
            left: KeyCode::KeyA,
            right: KeyCode::KeyT,
            jump: KeyCode::KeyN,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Jump,
}

type PlayerInput<'a> = Res<'a, ActionState<PlayerAction>>;

// todo: migrate to actionlike when available
// https://github.com/Leafwing-Studios/leafwing-input-manager/blob/main/examples/axis_inputs.rs#L22
impl Actionlike for PlayerAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Move => InputControlKind::Axis,
            _ => InputControlKind::Button,
        }
    }
}

fn update_player_input_map(bindings: Res<PlayerBindings>, mut cmd: Commands) {
    let mut input_map: InputMap<PlayerAction> = InputMap::default();
    input_map
        .insert_axis(
            PlayerAction::Move,
            VirtualAxis::new(bindings.left, bindings.right),
        )
        .insert(PlayerAction::Jump, bindings.jump);
    cmd.insert_resource(input_map);
}

// todo: rotate player towards typed word if the current char is not found it the currently faced tile and the player hasn't started the tile yet (ignore when tile is not pristine?)
// todo: reset tiles when player moves away from a tile (or even rotates?)
fn process_text_input(
    mut msg_r_kbd: MessageReader<KeyboardInput>,
    player_q: Query<&GridCoords, With<Player>>,
    level_lookup: Res<LevelEntityLookup>,
    mut word_tile_q: Query<&mut WordTile>,
    mut word_tile_msg_w: MessageWriter<WordTileEvent>,
) {
    let player_coords = or_return!(player_q.single());
    let mut typed = String::new();
    for ev in msg_r_kbd.read() {
        if let Key::Character(input) = &ev.logical_key {
            or_continue_quiet!(
                ev.state == bevy::input::ButtonState::Released
                    || input.chars().any(|c| c.is_control())
            );
            typed.push_str(&input.to_lowercase());
        }
    }
    match typed.as_str() {
        "" => return,
        _ => {
            for neighbour_coords in player_coords.neighbours() {
                let neighbour_e = or_continue_quiet!(level_lookup.get(&neighbour_coords));
                let mut word_tile = or_continue_quiet!(word_tile_q.get_mut(*neighbour_e));
                if word_tile.remaining().starts_with(&typed) {
                    word_tile_msg_w.write(WordTileEvent {
                        e: *neighbour_e,
                        kind: word_tile.advance(typed.len(), neighbour_coords),
                    });
                }
                // todo: invalid input feedback
                // possibly reset the current word on error?
            }
        }
    }

    typed.clear();
}

#[derive(Default, Debug, Reflect)]
pub struct TimedButtonInput {
    pub state: ButtonState,
    pub last_pressed: Option<Duration>,
}

fn collect_intent(
    mut player_q: Query<&mut MovementIntent, With<Player>>,
    input: PlayerInput,
    time: Res<Time>,
) {
    let mut intent = or_return!(player_q.single_mut());
    let horizontal_movement = input.clamped_value(&PlayerAction::Move);
    let jump_btn_data = input
        .button_data(&PlayerAction::Jump)
        .expect("Jump mapped properly");
    *intent = MovementIntent {
        horizontal_movement,
        jump: TimedButtonInput {
            state: jump_btn_data.state,
            last_pressed: match jump_btn_data.state {
                ButtonState::JustPressed => Some(Duration::ZERO),
                _ => intent.jump.last_pressed.map(|last| last + time.delta()),
            },
        },
    };
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum UiAction {
    Move,
    Select,
    Reset,
    Back,
}

// todo: migrate to actionlike when available
// https://github.com/Leafwing-Studios/leafwing-input-manager/blob/main/examples/axis_inputs.rs#L22
impl Actionlike for UiAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Move => InputControlKind::Axis,
            _ => InputControlKind::Button,
        }
    }
}

impl UiAction {
    fn input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // MKB
        input_map
            .insert_axis(Self::Move, VirtualAxis::WS)
            .insert_axis(Self::Move, VirtualAxis::VERTICAL_ARROW_KEYS)
            .insert(Self::Select, KeyCode::Space)
            .insert(Self::Select, KeyCode::Enter)
            .insert(Self::Back, KeyCode::Escape)
            .insert(Self::Reset, KeyCode::F5);

        input_map
    }
}

#[allow(dead_code)]
pub(crate) type UiInput<'a> = Res<'a, ActionState<UiAction>>;
