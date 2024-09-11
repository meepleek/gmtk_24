use crate::prelude::*;

pub(crate) const TILE_SIZE: u32 = 32;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin)
        .register_ldtk_int_cell::<UnbreakableGroundBundle>(1)
        .register_ldtk_int_cell::<GroundBundle>(2)
        .insert_resource(LevelSelection::index(0))
        .register_type::<LevelEntityLookup>()
        .add_systems(
            Update,
            (cache_level_entities, tween_entity_movement).run_if(in_game),
        )
        .add_systems(Last, (remove_tile_from_cache,).run_if(level_ready))
        .add_systems(Update, process_cooldown::<Moving>)
        .add_systems(OnEnter(Screen::Game), spawn_level)
        .add_systems(OnExit(Screen::Game), teardown_level);
}

pub(crate) fn level_ready(lookup: Option<Res<LevelEntityLookup>>) -> bool {
    lookup.is_some()
}

#[derive(Resource, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub(crate) struct LevelEntityLookup(pub HashMap<GridCoords, Entity>);

#[derive(Component, Debug)]
pub struct Movable {
    pub tween_duration_ms: u64,
    pub easing: Option<EaseFunction>,
}

impl Default for Movable {
    fn default() -> Self {
        Self {
            tween_duration_ms: 110,
            easing: None,
        }
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Moving;

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

fn spawn_level(ass: Res<AssetServer>, mut cmd: Commands) {
    cmd.spawn((
        Name::new("ldtk_world"),
        LdtkWorldBundle {
            ldtk_handle: ass.load("levels.ldtk"),
            ..Default::default()
        },
        StateScoped(Screen::Game),
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

fn remove_tile_from_cache(
    mut word_tile_evr: EventReader<WordTileEvent>,
    mut lookup: ResMut<LevelEntityLookup>,
) {
    for finished_tile_coords in word_tile_evr.read().filter_map(|ev| match ev.kind {
        WordTileEventKind::TileFinished { coords, .. } => Some(coords),
        _ => None,
    }) {
        warn!("rmv tile from cache");
        lookup.remove(&finished_tile_coords);
    }
}

fn tween_entity_movement(
    player_q: Query<(Entity, &GridCoords, &Movable), Changed<GridCoords>>,
    mut cmd: Commands,
) {
    for (e, coord, movable) in &player_q {
        cmd.tween_translation(
            e,
            coord.to_world(),
            movable.tween_duration_ms,
            movable.easing.unwrap_or(EaseFunction::QuadraticInOut),
        );
        cmd.entity(e).try_insert((
            Moving,
            Cooldown::<Moving>::new(movable.tween_duration_ms).remove_component(),
        ));
    }
}
