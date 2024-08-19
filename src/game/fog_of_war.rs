use crate::{camera::FOG_OF_WAR_RENDER_LAYER, prelude::*};
use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<FogOfWarMaterial>::default())
        .register_type::<TileVisibility>()
        .add_systems(Update, (add_visibility_to_tile).run_if(in_game))
        .add_systems(Update, (update_tile_visibility).run_if(level_ready));
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
struct TileVisibility {
    visibility: f32,
    sprite_e: Entity,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(crate) struct FogOfWarMaterial {
    #[uniform(0)]
    pub blur: f32,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    pub mask_texture: Option<Handle<Image>>,
}

impl Material2d for FogOfWarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fog_of_war.wgsl".into()
    }
}

fn add_visibility_to_tile(visibility_q: Query<Entity, Added<GridCoords>>, mut cmd: Commands) {
    for e in &visibility_q {
        let sprite_e = cmd
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::splat(16.)),
                        ..default()
                    },
                    ..default()
                },
                FOG_OF_WAR_RENDER_LAYER,
            ))
            .id();
        cmd.entity(e)
            .try_insert(TileVisibility {
                visibility: 0.0,
                sprite_e,
            })
            .add_child(sprite_e);
    }
}

fn update_tile_visibility(
    player_q: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    mut visibility_q: Query<&mut TileVisibility>,
    level_lookup: Res<LevelEntityLookup>,
    mut cmd: Commands,
) {
    let radius = 3;
    let player_coords = or_return_quiet!(player_q.get_single());
    let radius_tile_pairs: Vec<_> = player_coords
        .radius(radius, false)
        .iter()
        .filter_map(|c| level_lookup.get(c).map(|e| (*c, *e)))
        .collect();

    for (tile_coords, tile_e) in radius_tile_pairs {
        let dist = tile_coords.distance(player_coords).floor();
        let new_visibility = 1.0 - ((dist - 1.0) / radius as f32) * 0.8;
        let mut tile_vis = or_continue!(visibility_q.get_mut(tile_e));
        let mut cmd_e = or_continue_quiet!(cmd.get_entity(tile_vis.sprite_e));
        if tile_vis.visibility <= new_visibility {
            tile_vis.visibility = new_visibility;
            cmd_e.tween_sprite_color(
                Color::linear_rgb(new_visibility, new_visibility, new_visibility),
                350,
                EaseFunction::QuadraticInOut,
            );
        }
    }
}
