use crate::prelude::*;
use bevy::ecs::{system::RunSystemOnce as _, world::Command};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
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
                custom_size: Some(Vec2::splat(100.)),
                ..default()
            },
            ..Default::default()
        },
        StateScoped(Screen::Game),
    ));
}
