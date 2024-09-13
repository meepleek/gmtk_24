use crate::{assets::WordlistAssets, prelude::*};
use bevy::{color::palettes::tailwind, utils::HashSet};
use bevy_trauma_shake::Shakes;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<WordTile>()
        .add_event::<WordTileEvent>()
        .add_systems(OnExit(Screen::Loading), update_word_list)
        .add_systems(
            Update,
            update_word_list.run_if(assets_exist.and_then(resource_changed::<MovementBindings>)),
        )
        .add_systems(
            Update,
            (spawn_tile_words, update_ground_text_sections).run_if(in_game),
        )
        .add_systems(
            Update,
            (
                tween_ground_texts,
                tween_out_finished_tiles,
                shake_on_word_finished,
                play_word_sfx,
                spawn_cracks,
            )
                .run_if(level_ready),
        );
}

#[derive(Resource, Reflect, Debug, Deref, DerefMut)]
pub struct WordList {
    ground_words: Vec<String>,
    // other words for bosses etc
    // enemy_words: Vec<String>,
}

#[derive(Component, Reflect, Debug)]
pub(crate) struct WordTile {
    words: Vec<String>,
    word_i: usize,
    typed_char_len: usize,
    text_e: Entity,
}

#[derive(Debug, Reflect, PartialEq, Eq)]
pub(crate) enum WordTileEventKind {
    WordStarted,
    WordAdvanced,
    WordFinished(usize),
    TileFinished {
        word_count: usize,
        coords: GridCoords,
    },
}

#[derive(Event, Debug, Reflect)]
pub(crate) struct WordTileEvent {
    pub e: Entity,
    pub kind: WordTileEventKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum WordTileStatus {
    Pristine,
    Damaged,
    Finished,
}

impl WordTile {
    pub(crate) fn new(words: Vec<String>, text_e: Entity) -> Self {
        Self {
            words,
            word_i: 0,
            typed_char_len: 0,
            text_e,
        }
    }

    pub(crate) fn current_word(&self) -> &str {
        &self.words[self.word_i]
    }

    pub(crate) fn remaining(&self) -> String {
        self.current_word()
            .chars()
            .skip(self.typed_char_len)
            .collect()
    }

    pub(crate) fn advance(&mut self, count: usize, coords: GridCoords) -> WordTileEventKind {
        let typed_len_prev = self.typed_char_len;
        self.typed_char_len += count;
        if self.typed_char_len >= self.current_word().chars().count() {
            if self.word_i < (self.words.len() - 1) {
                self.word_i += 1;
                self.typed_char_len = 0;
                WordTileEventKind::WordFinished(self.word_i)
            } else {
                WordTileEventKind::TileFinished {
                    word_count: self.words.len(),
                    coords,
                }
            }
        } else if typed_len_prev == 0 {
            WordTileEventKind::WordStarted
        } else {
            WordTileEventKind::WordAdvanced
        }
    }

    pub(crate) fn status(&self) -> WordTileStatus {
        match (self.word_i, self.typed_char_len) {
            (0, 0) => WordTileStatus::Pristine,
            (word_i, typed)
                if word_i == (self.words.len() - 1) && typed >= self.words[word_i].len() =>
            {
                WordTileStatus::Finished
            }
            _ => WordTileStatus::Damaged,
        }
    }

    pub(crate) fn section(
        text: impl Into<String>,
        color: Color,
        font: Handle<Font>,
    ) -> TextSection {
        TextSection::new(
            text.into(),
            TextStyle {
                color,
                font_size: 36.0,
                font,
            },
        )
    }

    pub(crate) fn text_sections(&self, alpha: f32, font: Handle<Font>) -> Vec<TextSection> {
        tile_word_text_sections(
            &self.words,
            self.word_i,
            self.typed_char_len,
            self.status(),
            alpha,
            font,
        )
    }
}

fn update_word_list(
    wordlists: Res<Assets<WordListSource>>,
    wordlist_assets: Res<WordlistAssets>,
    bindings: Res<MovementBindings>,
    mut cmd: Commands,
) {
    let blacklist = [
        &bindings.up,
        &bindings.down,
        &bindings.left,
        &bindings.right,
    ];
    let source = or_return!(wordlists.get(&wordlist_assets.en));
    let mut words: Vec<_> = source
        .0
        .iter()
        .filter(|w| w.len() >= 3 && !blacklist.iter().any(|blacklisted| w.contains(*blacklisted)))
        .cloned()
        .collect();
    words.sort_unstable_by_key(|w| w.len());
    let split_i = or_return!(words.iter().enumerate().find_map(|(i, w)| if w.len() > 4 {
        Some(i)
    } else {
        None
    }));
    let _longer_words = words.split_off(split_i);

    cmd.insert_resource(WordList {
        ground_words: words,
    });
}

fn tile_word_text_sections(
    words: &[String],
    word_i: usize,
    typed_len: usize,
    status: WordTileStatus,
    alpha: f32,
    font: Handle<Font>,
) -> Vec<TextSection> {
    let mut res = Vec::with_capacity(4 + words.len());
    for (i, word) in words.iter().enumerate() {
        if i == word_i {
            if status != WordTileStatus::Pristine {
                res.push(WordTile::section(
                    word[..typed_len].to_string(),
                    tailwind::GRAY_700.with_alpha(alpha).into(),
                    font.clone_weak(),
                ));
            }
            if status != WordTileStatus::Finished {
                res.push(WordTile::section(
                    "|",
                    tailwind::GRAY_300.with_alpha(alpha).into(),
                    font.clone_weak(),
                ));
                let next_char_i = typed_len + 1;
                res.push(WordTile::section(
                    word[typed_len..next_char_i].to_string(),
                    tailwind::GREEN_200.with_alpha(alpha).into(),
                    font.clone_weak(),
                ));
                res.push(WordTile::section(
                    word[next_char_i..].to_string(),
                    tailwind::GRAY_200.with_alpha(alpha).into(),
                    font.clone_weak(),
                ));
            }
        } else {
            res.push(WordTile::section(
                word.to_string(),
                (if i < word_i {
                    tailwind::GRAY_700
                } else {
                    tailwind::GRAY_200
                })
                .with_alpha(alpha)
                .into(),
                font.clone_weak(),
            ));
        }

        if i < (words.len() - 1) {
            res.last_mut()
                .expect("At least 1 section has been added")
                .value += "\n";
        }
    }

    res
}

#[derive(Component, Default)]
struct TileWordVisible;

fn spawn_tile_words(
    ground_q: Query<Entity, Added<Ground>>,
    mut cmd: Commands,
    wordlist: Res<WordList>,
    fonts: Res<FontAssets>,
) {
    let mut rng = thread_rng();
    for e in &ground_q {
        let words: Vec<_> = wordlist
            .ground_words
            .choose_multiple(&mut rng, 3)
            .cloned()
            .collect();
        let mut text_e = None;
        let mut e_cmd = or_continue!(cmd.get_entity(e));
        e_cmd
            .with_children(|b| {
                text_e = Some(
                    b.spawn((
                        Text2dBundle {
                            text: Text::from_sections(tile_word_text_sections(
                                &words,
                                0,
                                0,
                                WordTileStatus::Pristine,
                                0.0,
                                fonts.tile.clone_weak(),
                            )),
                            transform: Transform::from_translation(Vec2::ZERO.extend(0.1))
                                .with_scale(Vec2::splat(0.25).extend(1.)),
                            ..default()
                        },
                        HIGH_RES_RENDER_LAYER,
                    ))
                    .id(),
                );
            })
            .try_insert(WordTile::new(words, text_e.unwrap()))
            .add_child(text_e.unwrap());
    }
}

fn update_ground_text_sections(
    mut word_tile_evr: EventReader<WordTileEvent>,
    word_q: Query<&WordTile>,
    mut text_q: Query<&mut Text>,
    fonts: Res<FontAssets>,
) {
    for ev in word_tile_evr.read() {
        let word = or_continue!(word_q.get(ev.e));
        let mut text = or_continue!(text_q.get_mut(word.text_e));
        text.sections = word.text_sections(1.0, fonts.tile.clone_weak());
    }
}

fn tween_out_finished_tiles(
    mut word_tile_evr: EventReader<WordTileEvent>,
    word_q: Query<&WordTile, Changed<WordTile>>,
    mut cmd: Commands,
) {
    for ev in word_tile_evr
        .read()
        .filter(|ev| matches!(ev.kind, WordTileEventKind::TileFinished { .. }))
    {
        let word = or_continue!(word_q.get(ev.e));
        let mut cmd_e = or_continue!(cmd.get_entity(ev.e));
        cmd_e.try_insert((
            DespawnOnTweenCompleted::Itself,
            FadeOutSpriteHiearchy { duration_ms: 150 },
        ));
        cmd.tween_tile_color(ev.e, Color::NONE, 150, EaseFunction::QuadraticIn);
        cmd.tween_text_alpha(word.text_e, 0.0, 110, EaseFunction::QuadraticIn);
    }
}

// tween text in/out as the player approaches/leaves
fn tween_ground_texts(
    player_q: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    word_q: Query<&WordTile>,
    visible_word_q: Query<Entity, With<TileWordVisible>>,
    level_lookup: Res<LevelEntityLookup>,
    mut cmd: Commands,
) {
    let player_coords = or_return_quiet!(player_q.get_single());
    let visible_tile_ids: HashSet<_> = visible_word_q.iter().collect();
    let radius_tile_ids: HashSet<_> = player_coords
        .neighbours()
        .iter()
        .filter_map(|c| level_lookup.get(c))
        .copied()
        .collect();

    // tween out when player has moved away
    for out_tile_e in visible_tile_ids.difference(&radius_tile_ids) {
        let word = or_continue_quiet!(word_q.get(*out_tile_e));
        if let Some(mut cmd_e) = cmd.get_entity(*out_tile_e) {
            cmd_e.remove::<TileWordVisible>();
            cmd.tween_text_alpha(word.text_e, 0.0, 110, EaseFunction::QuadraticOut);
        }
    }

    // tween in when player has moved in
    for tile_e in radius_tile_ids {
        let word = or_continue_quiet!(word_q.get(tile_e));
        if let Some(mut cmd_e) = cmd.get_entity(tile_e) {
            cmd_e.try_insert(TileWordVisible);
            cmd.tween_text_alpha(word.text_e, 1.0, 110, EaseFunction::QuadraticOut);
        }
    }
}

fn play_word_sfx(mut word_tile_evr: EventReader<WordTileEvent>, mut cmd: Commands) {
    if let Some(i) = word_tile_evr
        .read()
        .filter_map(|ev| match ev.kind {
            WordTileEventKind::WordFinished(i) => Some(i),
            WordTileEventKind::TileFinished { word_count, .. } => Some(word_count),
            _ => None,
        })
        .next()
    {
        word_tile_evr.clear();
        cmd.play_sfx(Sfx::FinishWord(i));
    };
}

fn spawn_cracks(
    mut word_tile_evr: EventReader<WordTileEvent>,
    mut cmd: Commands,
    sprites: Res<SpriteAssets>,
) {
    for (i, e) in word_tile_evr.read().filter_map(|ev| match ev.kind {
        WordTileEventKind::WordFinished(i) => Some((i - 1, ev.e)),
        WordTileEventKind::TileFinished { word_count, .. } => Some((word_count - 1, ev.e)),
        _ => None,
    }) {
        let mut e_cmd = or_continue_quiet!(cmd.get_entity(e));
        e_cmd.with_children(|b| {
            b.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::Z),
                    texture: sprites.tilemap.clone_weak(),
                    sprite: Sprite {
                        color: Color::NONE,
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    layout: sprites.tilemap_cracks_layout.clone_weak(),
                    index: i,
                },
                sprite_color_anim(Color::WHITE, 70, EaseFunction::QuadraticOut),
            ));
        });
    }
}

// todo: directional shake? (shake in the direction of the swing instead of just random)
fn shake_on_word_finished(mut word_tile_evr: EventReader<WordTileEvent>, mut shake: Shakes) {
    if word_tile_evr
        .read()
        .filter(|ev| {
            matches!(
                ev.kind,
                WordTileEventKind::WordFinished(_) | WordTileEventKind::TileFinished { .. }
            )
        })
        .next()
        .is_some()
    {
        shake.add_trauma(0.175);
    }
}

// // todo:
// fn flash_on_word_finished(
//     mut word_tile_evr: EventReader<WordTileEvent>,
//     word_q: Query<&WordTile, Changed<WordTile>>,
//     mut cmd: Commands,
// ) {
//     for ev in word_tile_evr
//         .read()
//         .filter(|ev| matches!(ev.kind, WordTileEventKind::WordFinished(_)))
//     {
//         let word_tile = or_continue!(word_q.get(ev.e));
//         let mut cmd_e = or_continue!(cmd.get_entity(ev.e));

//         cmd.tween_tile_color(ev.e, Color::NONE, 150, EaseFunction::QuadraticIn);
//     }
// }
