use crate::prelude::*;
use bevy::input::{
    keyboard::{Key, KeyboardInput},
    ButtonState,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<TypedInput>()
        .add_systems(Update, text_input);
}

#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub(crate) struct TypedInput(String);

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
                typed.push_str(input);
            }
            _ => {}
        }
    }
}
