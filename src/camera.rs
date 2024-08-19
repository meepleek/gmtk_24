use bevy::{
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
};

use crate::{game::fog_of_war::FogOfWarMaterial, prelude::*};

const DOWNSCALE_RES: u32 = 512;
/// Render layers for high-resolution rendering.
pub const HIGH_RES_RENDER_LAYER: RenderLayers = RenderLayers::layer(1);
pub const FOG_OF_WAR_RENDER_LAYER: RenderLayers = RenderLayers::layer(2);

pub(crate) const BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, spawn_camera);
}

/// Low-resolution texture that contains the pixel-perfect world.
/// Canvas itself is rendered to the high-resolution world.
#[derive(Component)]
struct LowResCanvas;

/// Camera that renders the pixel-perfect world to the [`Canvas`].
#[derive(Component)]
struct PixelPerfectCamera;

/// Camera that renders the [`Canvas`] (and other graphics on [`HIGH_RES_LAYERS`]) to the screen.
#[derive(Component)]
struct HighResCamera;

fn spawn_camera(
    mut cmd: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fog_of_war_mats: ResMut<Assets<FogOfWarMaterial>>,
) {
    // fog of war
    let fog_of_war_mask_handle = images.add(render_texture_image(DOWNSCALE_RES, DOWNSCALE_RES));
    cmd.spawn((
        Name::new("fog_of_war_cam"),
        Camera2dBundle {
            camera: Camera {
                order: -2,
                target: RenderTarget::Image(fog_of_war_mask_handle.clone()),
                ..default()
            },
            ..default()
        },
        FOG_OF_WAR_RENDER_LAYER,
    ));

    // // debug mask
    // cmd.spawn((
    //     Name::new("fog_of_war_canvas"),
    //     SpriteBundle {
    //         texture: mask_handle.clone(),
    //         sprite: Sprite {
    //             // color: Color::WHITE.with_alpha(0.25),
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     HIGH_RES_RENDER_LAYER,
    // ));

    // pixel perfect render
    let pixel_perfect_canvas_handle =
        images.add(render_texture_image(DOWNSCALE_RES, DOWNSCALE_RES));
    cmd.spawn((
        Name::new("pixel_perfect_cam"),
        Camera2dBundle {
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(pixel_perfect_canvas_handle.clone()),
                ..default()
            },
            ..default()
        },
        PixelPerfectCamera,
        IsDefaultUiCamera,
    ));

    // spawn the canvas
    cmd.spawn((
        Name::new("pixel_canvas"),
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform::from_scale(Vec2::splat(512.0).extend(1.)),
            material: fog_of_war_mats.add(FogOfWarMaterial {
                texture: pixel_perfect_canvas_handle,
                mask_texture: Some(fog_of_war_mask_handle),
                blur: 1.0,
            }),
            ..default()
        },
        LowResCanvas,
        HIGH_RES_RENDER_LAYER,
    ));

    let mut screen_camera = Camera2dBundle::default();
    screen_camera.projection.scale = 0.25;
    screen_camera.transform.translation = Vec2::splat(1024.0 / 8.0).extend(0.0);
    cmd.spawn((
        Name::new("screen_cam"),
        screen_camera,
        HighResCamera,
        HIGH_RES_RENDER_LAYER,
    ));
}

fn render_texture_image(width: u32, height: u32) -> Image {
    let size = Extent3d {
        width,
        height,
        ..default()
    };
    let mut img = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    // resizes by zero-ing out the buffer
    img.resize(size);
    img
}
