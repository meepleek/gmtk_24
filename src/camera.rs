use crate::prelude::*;

pub(crate) const BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, spawn_camera);
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn((
        Name::new("Camera"),
        Camera2dBundle::default(),
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
    ));
}
