use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<FadeOutSpriteHiearchy>()
        .add_systems(Update, fade_out_sprite_hiearchy);
}

#[derive(Component, Debug, Reflect)]
#[component(storage = "SparseSet")]
pub struct FadeOutSpriteHiearchy {
    pub duration_ms: u64,
}

fn fade_out_sprite_hiearchy(
    fade_q: Query<(Entity, &FadeOutSpriteHiearchy), Added<FadeOutSpriteHiearchy>>,
    child_q: Query<&Children>,
    sprite_q: Query<&Sprite>,
    mut cmd: Commands,
) {
    for (e, fade) in &fade_q {
        for child_e in child_q.iter_descendants(e) {
            if sprite_q.contains(child_e) {
                cmd.entity(child_e).try_insert(sprite_color_anim(
                    Color::NONE,
                    fade.duration_ms,
                    EaseFunction::QuadraticIn,
                ));
            }
        }
    }
}
