use bevy::math::NormedVectorSpace;

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
                // cmd.entity(child_e).try_insert(sprite_color_anim(
                //     Color::NONE,
                //     fade.duration_ms,
                //     EaseFunction::QuadraticIn,
                // ));
            }
        }
    }
}

// todo: yeet after 0.15 lands
pub trait StableInterpolate: Clone {
    fn interpolate_stable(&self, other: &Self, t: f32) -> Self;
    fn interpolate_stable_assign(&mut self, other: &Self, t: f32) {
        *self = self.interpolate_stable(other, t);
    }

    /// Smoothly nudge this value towards the `target` at a given decay rate. The `decay_rate`
    /// parameter controls how fast the distance between `self` and `target` decays relative to
    /// the units of `delta`; the intended usage is for `decay_rate` to generally remain fixed,
    /// while `delta` is something like `delta_time` from an updating system. This produces a
    /// smooth following of the target that is independent of framerate.
    ///
    /// More specifically, when this is called repeatedly, the result is that the distance between
    /// `self` and a fixed `target` attenuates exponentially, with the rate of this exponential
    /// decay given by `decay_rate`.
    ///
    /// For example, at `decay_rate = 0.0`, this has no effect.
    /// At `decay_rate = f32::INFINITY`, `self` immediately snaps to `target`.
    /// In general, higher rates mean that `self` moves more quickly towards `target`.
    ///
    /// # Example
    /// ```
    /// # use bevy_math::{Vec3, StableInterpolate};
    /// # let delta_time: f32 = 1.0 / 60.0;
    /// let mut object_position: Vec3 = Vec3::ZERO;
    /// let target_position: Vec3 = Vec3::new(2.0, 3.0, 5.0);
    /// // Decay rate of ln(10) => after 1 second, remaining distance is 1/10th
    /// let decay_rate = f32::ln(10.0);
    /// // Calling this repeatedly will move `object_position` towards `target_position`:
    /// object_position.smooth_nudge(&target_position, decay_rate, delta_time);
    /// ```
    fn smooth_nudge(&mut self, target: &Self, decay_rate: f32, delta: f32) {
        self.interpolate_stable_assign(target, 1.0 - (-decay_rate * delta).exp());
    }
}
impl<V> StableInterpolate for V
where
    V: NormedVectorSpace,
{
    #[inline]
    fn interpolate_stable(&self, other: &Self, t: f32) -> Self {
        self.lerp(*other, t)
    }
}
