use crate::prelude::*;

pub(crate) const TILE_SIZE: i32 = 16;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin)
        .register_ldtk_int_cell::<GroundBundle>(1)
        .register_ldtk_int_cell::<UnbreakableGroundBundle>(2)
        .insert_resource(LevelSelection::index(0))
        // .insert_resource(LevelSelection::index(1))
        .register_type::<LevelEntityLookup>()
        .add_systems(
            Update,
            (draw_level_grid.run_if(in_game), cache_level_entities),
        )
        .add_systems(OnEnter(Screen::Game), spawn_level)
        .add_systems(OnExit(Screen::Game), teardown_level);
}

#[derive(Resource, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub(crate) struct LevelEntityLookup(pub HashMap<GridCoords, Entity>);

// todo: make this an enum & use the intgrid value to determine the variant
#[derive(Component, Default)]
pub(crate) struct Ground;

#[derive(Default, Bundle, LdtkIntCell)]
struct GroundBundle {
    ground: Ground,
}

#[derive(Default, Component)]
pub(crate) struct UnbreakableGround;

#[derive(Default, Bundle, LdtkIntCell)]
struct UnbreakableGroundBundle {
    unbreakable_ground: UnbreakableGround,
}

pub(crate) fn level_ready(lookup: Option<Res<LevelEntityLookup>>) -> bool {
    lookup.is_some()
}

fn spawn_level(ass: Res<AssetServer>, mut cmd: Commands) {
    cmd.spawn((
        Name::new("ldtk_world"),
        LdtkWorldBundle {
            ldtk_handle: ass.load("levels.ldtk"),
            ..Default::default()
        },
    ));
}

fn teardown_level(mut cmd: Commands) {
    cmd.remove_resource::<LevelEntityLookup>();
}

fn cache_level_entities(
    // mut level_entities: Option<ResMut<LevelEntityLookup>>,
    mut level_evr: EventReader<LevelEvent>,
    // ldtk_project_entities: Query<&Handle<LdtkProject>>,
    // ldtk_project_assets: Res<Assets<LdtkProject>>,
    tilemap_id_q: Query<(&GridCoords, Entity)>,
    mut cmd: Commands,
) {
    for level_event in level_evr.read() {
        if let LevelEvent::Transformed(_level_iid) = level_event {
            // let ldtk_project = ldtk_project_assets
            //     .get(ldtk_project_entities.single())
            //     .expect("LdtkProject should be loaded when level is spawned");
            // let level = ldtk_project
            //     .get_raw_level_by_iid(level_iid.get())
            //     .expect("spawned level should exist in project");

            let coords_entity_lookup = tilemap_id_q
                .iter()
                .map(|(coords, e)| (*coords, e))
                .collect();
            cmd.insert_resource(LevelEntityLookup(coords_entity_lookup));
        }
    }
}

fn draw_level_grid(mut gizmos: Gizmos) {
    gizmos
        .grid_2d(
            Vec2::ZERO,
            0.0,
            UVec2::splat(64),
            Vec2::splat(TILE_SIZE as f32),
            // Dark gray
            LinearRgba::gray(0.1),
        )
        .outer_edges();
}
