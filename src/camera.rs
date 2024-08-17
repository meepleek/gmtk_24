use crate::prelude::*;

pub(crate) const BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, spawn_camera);
}

fn spawn_camera(mut cmd: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.25;
    camera.transform.translation = Vec2::splat(1024.0 / 8.0).extend(0.0);
    cmd.spawn((
        Name::new("Camera"),
        camera,
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
    ));
}
