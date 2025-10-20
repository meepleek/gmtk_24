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
            Sprite::from_image(sprites.player_sheet.clone()),
            Transform::from_translation(coords.to_world()),
            TextureAtlas {
                layout: sprites.idle_anim_layout.clone(),
                index: 0,
            },
            animation::PlayerAnimation::default(),
            animation::AnimationTimer(Timer::new(
                Duration::from_millis(animation::PlayerAnimation::Idle.frame_base_duration_ms(0)),
                TimerMode::Repeating,
            )),
            Gravity::default(),
            KinematicSensor {
                size: Vec2::new(18., 20.),
                ground_y_offset: 5.,
            },
            Grounded::airborne(0),
            HorizontalObstacleDetection::default(),
            Velocity::default(),
            MovementIntent::default(),
            MovementEasing::default(),
            GamePhysicsLayer::membership(GamePhysicsLayer::Player),
        ));
    }
}
