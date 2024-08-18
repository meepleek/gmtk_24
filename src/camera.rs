use bevy::render::{
    camera::RenderTarget,
    render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    view::RenderLayers,
};

use crate::prelude::*;

const DOWNSCALE_RES: u32 = 512;
/// Render layers for high-resolution rendering.
pub const HIGH_RES_RENDER_LAYER: RenderLayers = RenderLayers::layer(1);

pub(crate) const BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, spawn_camera);
}

/// Low-resolution texture that contains the pixel-perfect world.
/// Canvas itself is rendered to the high-resolution world.
#[derive(Component)]
struct Canvas;

/// Camera that renders the pixel-perfect world to the [`Canvas`].
#[derive(Component)]
struct PixelPerfectCamera;

/// Camera that renders the [`Canvas`] (and other graphics on [`HIGH_RES_LAYERS`]) to the screen.
#[derive(Component)]
struct HighResCamera;

fn spawn_camera(mut cmd: Commands, mut images: ResMut<Assets<Image>>) {
    let canvas_size = Extent3d {
        width: DOWNSCALE_RES,
        height: DOWNSCALE_RES,
        ..default()
    };

    // this Image serves as a canvas representing the low-resolution game screen
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
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
    canvas.resize(canvas_size);
    let image_handle = images.add(canvas);
    cmd.spawn((
        Camera2dBundle {
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        PixelPerfectCamera,
        IsDefaultUiCamera,
    ));

    // spawn the canvas
    cmd.spawn((
        SpriteBundle {
            texture: image_handle,
            ..default()
        },
        Canvas,
        HIGH_RES_RENDER_LAYER,
    ));

    let mut screen_camera = Camera2dBundle::default();
    screen_camera.projection.scale = 0.25;
    screen_camera.transform.translation = Vec2::splat(1024.0 / 8.0).extend(0.0);
    cmd.spawn((screen_camera, HighResCamera, HIGH_RES_RENDER_LAYER));

    // let mut camera = Camera2dBundle::default();
    // camera.projection.scale = 0.25;
    // camera.transform.translation = Vec2::splat(1024.0 / 8.0).extend(0.0);
    // cmd.spawn((Name::new("Camera"), camera, IsDefaultUiCamera));
}
