use crate::{camera::FOG_OF_WAR_RENDER_LAYER, prelude::*};
use bevy::{
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
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
                Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::splat(TILE_SIZE as f32)),
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
    wall_q: Query<&UnbreakableGround>,
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
        if grid_line((*player_coords).into(), tile_coords.into())
            .iter()
            .skip(1) // skip the initial/player coord
            .any(|c| level_lookup.get(c).map_or(true, |e| wall_q.contains(*e)))
        {
            continue;
        }
        let dist = tile_coords.distance(player_coords).floor();
        let new_visibility = 1.0 - ((dist - 1.0) / radius as f32) * 0.8;
        let mut tile_vis = or_continue_quiet!(visibility_q.get_mut(tile_e));
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

pub(crate) fn grid_line(a: IVec2, b: IVec2) -> Vec<GridCoords> {
    if a == b {
        return vec![a.into()];
    }

    let dir = b - a;
    let dir_abs = dir.abs();
    let sign = IVec2::new(
        if dir.x > 0 { 1 } else { -1 },
        if dir.y > 0 { 1 } else { -1 },
    );

    let mut points = Vec::with_capacity(dir_abs.max_element() as usize);
    let mut point = a;
    points.push(point.into());
    let mut i = IVec2::ZERO;
    loop {
        let what = (i.as_vec2() + Vec2::splat(0.5)) / dir_abs.as_vec2();
        if what.x < what.y {
            // horizontal step
            point.x += sign.x;
            i.x += 1;
        } else {
            // vertical step
            point.y += sign.y;
            i.y += 1;
        }

        points.push(point.into());
        if i.x >= dir_abs.x && i.y >= dir_abs.y {
            break;
        }
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use tracing_test::traced_test;

    #[traced_test]
    #[test_case(IVec2::ZERO, IVec2::ZERO => vec![GridCoords::new(0, 0)])]
    #[test_case(IVec2::ZERO, IVec2::new(3, 0) =>
        vec![
            GridCoords::new(0, 0),
            GridCoords::new(1, 0),
            GridCoords::new(2, 0),
            GridCoords::new(3, 0),
        ]
    )]
    #[test_case(IVec2::ZERO, IVec2::new(0, 3) =>
        vec![
            GridCoords::new(0, 0),
            GridCoords::new(0, 1),
            GridCoords::new(0, 2),
            GridCoords::new(0, 3),
        ]
    )]
    #[test_case(IVec2::ZERO, IVec2::new(2, 2) =>
        vec![
            GridCoords::new(0, 0),
            GridCoords::new(0, 1),
            GridCoords::new(1, 1),
            GridCoords::new(1, 2),
            GridCoords::new(2, 2),
        ]
    )]
    #[test_case(IVec2::new(1, 2), IVec2::new(4, 3) =>
        vec![
            GridCoords::new(1, 2),
            GridCoords::new(2, 2),
            GridCoords::new(2, 3),
            GridCoords::new(3, 3),
            GridCoords::new(4, 3),
        ]
    )]
    fn grid_line_coords(a: IVec2, b: IVec2) -> Vec<GridCoords> {
        grid_line(a, b)
    }
}
