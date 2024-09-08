use crate::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .continue_to_state(if cfg!(feature = "dev") {
                Screen::Game
            } else {
                Screen::MainMenu
            })
            .load_collection::<SpriteAssets>()
            .load_collection::<FontAssets>()
            .load_collection::<SfxAssets>()
            .load_collection::<MusicAssets>()
            .load_collection::<WordlistAssets>(),
    );
    // app.add_systems(Startup, setup_particles);
}

#[allow(dead_code)]
pub fn assets_exist(
    sprites: Option<Res<SpriteAssets>>,
    fonts: Option<Res<WordlistAssets>>,
    sfx: Option<Res<SfxAssets>>,
    music: Option<Res<MusicAssets>>,
    wordlists: Option<Res<FontAssets>>,
    // particles: Option<Res<ParticleAssets>>,
) -> bool {
    sprites.is_some() && fonts.is_some() && sfx.is_some() && music.is_some() && wordlists.is_some()
    /*&& particles.is_some()*/
}

#[derive(AssetCollection, Resource)]
pub struct WordlistAssets {
    #[asset(path = "en.words.txt")]
    pub en: Handle<WordListSource>,
}

// https://github.com/NiklasEi/bevy_asset_loader?tab=readme-ov-file#supported-asset-fields
#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 5, rows = 1))]
    pub idle_anim_layout: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 32,
        tile_size_y = 32,
        columns = 3,
        rows = 1,
        offset_y = 32
    ))]
    pub swing_anticipation_anim_layout: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 32,
        tile_size_y = 32,
        columns = 4,
        rows = 1,
        offset_y = 64
    ))]
    pub swing_anticipation_idle_anim_layout: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 32,
        tile_size_y = 32,
        columns = 5,
        rows = 1,
        offset_y = 96
    ))]
    pub swing_anim_layout: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 32,
        tile_size_y = 32,
        columns = 3,
        rows = 1,
        offset_y = 128
    ))]
    #[allow(dead_code)]
    pub swing_fast_anim_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "images/player.png")]
    pub player_sheet: Handle<Image>,
    #[asset(texture_atlas_layout(
        tile_size_x = 32,
        tile_size_y = 32,
        columns = 3,
        rows = 1,
        offset_y = 480
    ))]
    pub tilemap_cracks_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "images/tilemap.png")]
    pub tilemap: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/m5x7.ttf")]
    pub tile: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct SfxAssets {
    #[asset(path = "audio/sfx/button_hover.ogg")]
    pub button_hover: Handle<AudioSource>,
    #[asset(path = "audio/sfx/button_press.ogg")]
    pub button_click: Handle<AudioSource>,
    #[asset(
        paths(
            "audio/sfx/hit_1_1.ogg",
            "audio/sfx/hit_1_1.ogg",
            "audio/sfx/hit_1_3.ogg"
        ),
        collection(typed)
    )]
    pub hit_1: Vec<Handle<AudioSource>>,
    #[asset(
        paths(
            "audio/sfx/hit_2_1.ogg",
            "audio/sfx/hit_2_1.ogg",
            "audio/sfx/hit_2_3.ogg"
        ),
        collection(typed)
    )]
    pub hit_2: Vec<Handle<AudioSource>>,
    #[asset(
        paths(
            "audio/sfx/hit_3_1.ogg",
            "audio/sfx/hit_3_1.ogg",
            "audio/sfx/hit_3_3.ogg"
        ),
        collection(typed)
    )]
    pub hit_3: Vec<Handle<AudioSource>>,
}

#[derive(AssetCollection, Resource)]
pub struct MusicAssets {
    #[asset(path = "audio/music/main_menu.ogg")]
    pub main_menu: Handle<AudioSource>,
    #[asset(path = "audio/music/game.ogg")]
    pub game: Handle<AudioSource>,
}

// todo: use asset_loader for particles too
// #[derive(AssetCollection, Resource)]
// pub struct ParticleAssets {
//     #[asset(path = "particles/circle.png")]
//     pub circle_mat: Handle<SpriteParticle2dMaterial>,
//     #[asset(path = "particles/gun.particle.ron")]
//     pub gun: Handle<Particle2dEffect>,
//     #[asset(path = "particles/enemy.particle.ron")]
//     pub enemy: Handle<Particle2dEffect>,
//     #[asset(path = "particles/reflection.particle.ron")]
//     pub reflection: Handle<Particle2dEffect>,
//     #[asset(path = "particles/core.particle.ron")]
//     pub core: Handle<Particle2dEffect>,
// }

// #[derive(Resource, Reflect)]
// #[reflect(Resource)]
// pub struct ParticleAssets {
//     pub circle_mat: Handle<SpriteParticle2dMaterial>,
//     pub gun: Handle<Particle2dEffect>,
//     pub enemy: Handle<Particle2dEffect>,
//     pub reflection: Handle<Particle2dEffect>,
//     pub core: Handle<Particle2dEffect>,
//     pub core_clear: Handle<Particle2dEffect>,
//     pub bg: Handle<Particle2dEffect>,
//     pub ball: Handle<Particle2dEffect>,
// }

// impl ParticleAssets {
//     pub fn square_particle_spawner(
//         &self,
//         effect: Handle<Particle2dEffect>,
//         transform: Transform,
//     ) -> ParticleSpawnerBundle<ColorParticle2dMaterial> {
//         ParticleSpawnerBundle {
//             effect,
//             material: DEFAULT_MATERIAL,
//             transform,
//             ..default()
//         }
//     }

//     pub fn particle_spawner(
//         &self,
//         effect: Handle<Particle2dEffect>,
//         transform: Transform,
//     ) -> ParticleSpawnerBundle<SpriteParticle2dMaterial> {
//         ParticleSpawnerBundle {
//             effect,
//             material: self.circle_mat.clone(),
//             transform,
//             ..default()
//         }
//     }
// }

// fn setup_particles(
//     ass: Res<AssetServer>,
//     mut materials: ResMut<Assets<SpriteParticle2dMaterial>>,
//     mut cmd: Commands,
// ) {
//     cmd.insert_resource(ParticleAssets {
//         circle_mat: materials.add(
//             // hframes and vframes define how the sprite sheet is divided for animations,
//             // if you just want to bind a single texture, leave both at 1.
//             SpriteParticle2dMaterial::new(ass.load("particles/circle.png"), 1, 1),
//         ),
//         gun: ass.load("particles/gun.particle.ron"),
//         enemy: ass.load("particles/enemy.particle.ron"),
//         reflection: ass.load("particles/reflection.particle.ron"),
//         core: ass.load("particles/core.particle.ron"),
//         core_clear: ass.load("particles/core_clear.particle.ron"),
//         bg: ass.load("particles/bg.particle.ron"),
//         ball: ass.load("particles/ball.particle.ron"),
//     });
// }
