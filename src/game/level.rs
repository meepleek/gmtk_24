use bevy::color::palettes::tailwind;

use crate::prelude::*;

pub(crate) const TILE_SIZE: i32 = 16;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin)
        .register_ldtk_int_cell::<UnbreakableGroundBundle>(1)
        .register_ldtk_int_cell::<GroundBundle>(2)
        .insert_resource(LevelSelection::index(0))
        // .insert_resource(LevelSelection::index(1))
        .register_type::<LevelEntityLookup>()
        .register_type::<TileWord>()
        .add_systems(
            Update,
            (
                draw_level_grid,
                cache_level_entities,
                spawn_tile_words,
                update_ground_text_sections,
            )
                .run_if(in_game),
        )
        .add_systems(OnEnter(Screen::Game), spawn_level)
        .add_systems(OnExit(Screen::Game), teardown_level);
}

pub(crate) fn level_ready(lookup: Option<Res<LevelEntityLookup>>) -> bool {
    lookup.is_some()
}

#[derive(Resource, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub(crate) struct LevelEntityLookup(pub HashMap<GridCoords, Entity>);

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

#[derive(Component, Reflect, Debug)]
pub(crate) struct TileWord {
    text: String,
    typed_char_len: usize,
    text_e: Entity,
}

impl TileWord {
    pub(crate) fn new(text: impl Into<String>, text_e: Entity) -> Self {
        Self {
            text: text.into(),
            typed_char_len: 0,
            text_e,
        }
    }

    pub(crate) fn remaining(&self) -> String {
        self.text.chars().skip(self.typed_char_len).collect()
    }

    pub(crate) fn advance(&mut self, count: usize) {
        self.typed_char_len += count;
    }

    pub(crate) fn done(&self) -> bool {
        self.text.len() <= self.typed_char_len
    }

    pub(crate) fn section(text: impl Into<String>, color: Color) -> TextSection {
        TextSection::new(
            text.into(),
            TextStyle {
                color,
                font_size: 24.0,
                ..default()
            },
        )
    }

    pub(crate) fn done_section(text: impl Into<String>) -> TextSection {
        Self::section(text, tailwind::GRAY_400.into())
    }

    pub(crate) fn text_sections(&self) -> Vec<TextSection> {
        let mut res = Vec::with_capacity(4);
        if self.typed_char_len > 0 {
            res.push(Self::done_section(
                self.text[..self.typed_char_len].to_string(),
            ));
        }
        if !self.done() {
            res.push(Self::section("|", tailwind::GRAY_950.into()));
            let next_char_i = self.typed_char_len + 1;
            res.push(Self::section(
                self.text[self.typed_char_len..next_char_i].to_string(),
                tailwind::GREEN_700.into(),
            ));
            res.push(Self::section(
                self.text[next_char_i..].to_string(),
                tailwind::GRAY_950.into(),
            ));
        }

        res
    }
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

const WORDS: &[&str] = &["bar", "baz", "test", "dig"];

fn spawn_tile_words(ground_q: Query<Entity, Added<Ground>>, mut cmd: Commands) {
    let mut rng = thread_rng();
    for e in &ground_q {
        let word = *WORDS.choose(&mut rng).expect("random word picked");
        let mut text_e = None;
        let mut e_cmd = or_continue!(cmd.get_entity(e));
        e_cmd
            .with_children(|b| {
                text_e = Some(
                    b.spawn(Text2dBundle {
                        text: Text::from_sections(vec![TileWord::done_section(word)]),
                        transform: Transform::from_translation(Vec2::ZERO.extend(0.1))
                            .with_scale(Vec2::splat(0.25).extend(1.)),
                        ..default()
                    })
                    .id(),
                );
            })
            .try_insert(TileWord::new(word, text_e.unwrap()))
            .add_child(text_e.unwrap());
    }
}

fn update_ground_text_sections(
    word_q: Query<&TileWord, Changed<TileWord>>,
    mut text_q: Query<&mut Text>,
) {
    for word in &word_q {
        let mut text = or_continue!(text_q.get_mut(word.text_e));
        text.sections = word.text_sections();
    }
}

// tween text in/out as the player approaches/leaves
fn tween_ground_texts(
    player_q: Query<(Entity, &GridCoords), (With<Player>, Changed<GridCoords>)>,
    mut cmd: Commands,
) {
    for (e, coord) in &player_q {
        cmd.tween_translation(e, coord.to_world(), 110, EaseFunction::QuadraticOut);
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
