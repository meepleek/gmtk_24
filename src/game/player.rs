use crate::prelude::*;
use bevy::ecs::{system::RunSystemOnce as _, world::Command};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>()
        .add_systems(Update, (process_typed_input, move_player));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Debug)]
pub struct SpawnPlayer {
    pub _max_speed: f32,
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_player);
    }
}

fn spawn_player(In(_config): In<SpawnPlayer>, mut cmd: Commands) {
    cmd.spawn((
        Name::new("Player"),
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(GRID_SIZE)),
                ..default()
            },
            ..Default::default()
        },
        Coordinate(IVec2::ZERO),
        StateScoped(Screen::Game),
    ));
}

fn process_typed_input(
    mut typed: ResMut<TypedInput>,
    mut player_q: Query<&mut Coordinate, With<Player>>,
) {
    if let Some(move_by) = match typed.as_str() {
        "a" => Some(IVec2::NEG_X),
        "d" => Some(IVec2::X),
        "w" => Some(IVec2::Y),
        "s" => Some(IVec2::NEG_Y),
        _ => {
            // todo?
            typed.clear();
            None
        }
    } {
        typed.clear();
        let mut coord = or_return!(player_q.get_single_mut());
        coord.0 += move_by;
    }
}

fn move_player(
    player_q: Query<(Entity, &Coordinate), (With<Player>, Changed<Coordinate>)>,
    mut cmd: Commands,
) {
    for (e, coord) in &player_q {
        cmd.tween_translation(e, coord.to_world(), 110, EaseFunction::QuadraticOut);
    }
}
