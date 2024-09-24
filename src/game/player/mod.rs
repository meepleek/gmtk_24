use crate::prelude::*;
use std::time::Duration;

mod animation;
pub mod input;
pub mod movement;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((animation::plugin, input::plugin, movement::plugin))
        .register_type::<Player>()
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_systems(Update, on_player_spawned.run_if(assets_exist));
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[grid_coords]
    grid_coords: GridCoords,
}

fn on_player_spawned(
    player_q: Query<(Entity, &GridCoords), Added<Player>>,
    mut cmd: Commands,
    sprites: Res<SpriteAssets>,
) {
    for (e, coords) in &player_q {
        cmd.entity(e).try_insert((
            SpriteBundle {
                texture: sprites.player_sheet.clone_weak(),
                transform: Transform::from_translation(coords.to_world()),
                ..default()
            },
            TextureAtlas {
                layout: sprites.idle_anim_layout.clone_weak(),
                index: 0,
            },
            animation::PlayerAnimation::default(),
            animation::AnimationTimer(Timer::new(
                Duration::from_millis(animation::PlayerAnimation::Idle.frame_base_duration_ms(0)),
                TimerMode::Repeating,
            )),
            Gravity::default(),
            GroundSensor {
                width: 18.,
                y: -(TILE_SIZE as f32 / 2.),
            },
            Velocity::default(),
            MovementIntent::default(),
            GamePhysicsLayer::membership(GamePhysicsLayer::Player),
        ));
    }
}
