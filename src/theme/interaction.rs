use crate::prelude::*;

use super::widgets::{BUTTON_HEIGHT, BUTTON_WIDTH};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.add_systems(
        Update,
        (
            trigger_on_press,
            apply_interaction_palette,
            apply_button_interaction_size,
            trigger_interaction_sfx,
        ),
    );
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

/// Event triggered on a UI entity when the [`Interaction`] component on the same entity changes to
/// [`Interaction::Pressed`]. Observe this event to detect e.g. button presses.
#[derive(Event)]
pub struct OnPress;

fn trigger_on_press(
    interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (entity, interaction) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            commands.trigger_targets(OnPress, entity);
        }
    }
}

fn apply_interaction_palette(
    mut palette_query: Query<(Entity, &Interaction, &InteractionPalette), Changed<Interaction>>,
    mut cmd: Commands,
) {
    for (e, interaction, palette) in &mut palette_query {
        let color = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        };
        cmd.tween_ui_bg_color(e, color, 200, EaseFunction::QuadraticInOut);
    }
}

fn apply_button_interaction_size(
    mut palette_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<Button>)>,
    mut cmd: Commands,
) {
    for (e, interaction) in &mut palette_query {
        let scale = match interaction {
            Interaction::None => 1.0,
            Interaction::Hovered => 1.1,
            Interaction::Pressed => 1.25,
        };
        cmd.tween_style_size(
            e,
            Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT) * scale,
            350,
            EaseFunction::BackOut,
        );
    }
}

fn trigger_interaction_sfx(
    interaction_query: Query<&Interaction, Changed<Interaction>>,
    mut commands: Commands,
) {
    for interaction in &interaction_query {
        match interaction {
            Interaction::Hovered => commands.play_sfx(Sfx::ButtonHover),
            Interaction::Pressed => commands.play_sfx(Sfx::ButtonClick),
            _ => (),
        }
    }
}
