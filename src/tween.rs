// #![allow(dead_code)]

// use bevy::{ecs::system::EntityCommands, prelude::*};
// use bevy_ecs_tilemap::tiles::TileColor;
// use bevy_tweening::*;
// use std::{marker::PhantomData, time::Duration};

// #[derive(Component)]
// pub enum DespawnOnTweenCompleted {
//     Itself,
//     Entity(Entity),
// }

// #[derive(Component)]
// #[component(storage = "SparseSet")]
// pub struct TweenFactor<T: Send + Sync + 'static> {
//     timer: Timer,
//     delay: Option<Timer>,
//     ease: EaseFunction,
//     _phantom: PhantomData<T>,
// }

// impl<T: Send + Sync> TweenFactor<T> {
//     pub fn new(duration_ms: u64, ease: EaseFunction) -> Self {
//         Self {
//             timer: Timer::new(Duration::from_millis(duration_ms), TimerMode::Once),
//             delay: None,
//             ease,
//             _phantom: default(),
//         }
//     }

//     pub fn with_delay(mut self, delay_ms: u64) -> Self {
//         self.delay = Some(Timer::new(Duration::from_millis(delay_ms), TimerMode::Once));
//         self
//     }

//     pub fn factor(&self) -> f32 {
//         self.timer.fraction().calc(self.ease)
//     }
// }

// pub fn tween_factor<T: Send + Sync>(mut factor_q: Query<&mut TweenFactor<T>>, time: Res<Time>) {
//     for mut factor in &mut factor_q {
//         if let Some(delay) = factor.delay.as_mut() {
//             delay.tick(time.delta());
//             if delay.just_finished() {
//                 factor.delay.take();
//             }
//         } else if !factor.timer.finished() {
//             factor.timer.tick(time.delta());
//         }
//     }
// }

// pub(super) fn plugin(app: &mut App) {
//     app.add_plugins(TweeningPlugin).add_systems(
//         Update,
//         (
//             component_animator_system::<BackgroundColor>,
//             component_animator_system::<TileColor>,
//             despawn_after_tween,
//         ),
//     );
// }

// fn despawn_after_tween(
//     mut cmd: Commands,
//     mut ev_r: EventReader<TweenCompleted>,
//     despawn_q: Query<&DespawnOnTweenCompleted>,
// ) {
//     for ev in ev_r.read() {
//         if let Ok(despawn) = despawn_q.get(ev.entity) {
//             let e = match despawn {
//                 DespawnOnTweenCompleted::Itself => ev.entity,
//                 DespawnOnTweenCompleted::Entity(e) => *e,
//             };
//             cmd.entity(e).despawn_recursive();
//         }
//     }
// }

// pub fn delay_tween<T: 'static>(tween: Tween<T>, delay_ms: u64) -> Sequence<T> {
//     if delay_ms > 0 {
//         Delay::new(Duration::from_millis(delay_ms)).then(tween)
//     } else {
//         Sequence::new([tween])
//     }
// }

// relative_tween_impl!(
//     translation,
//     Animator,
//     Transform,
//     TransformRelativePositionLens,
//     Vec3,
//     Vec3
// );

// relative_tween_impl!(
//     scale,
//     Animator,
//     Transform,
//     TransformRelativeScaleLens,
//     Vec3,
//     Vec3
// );

// relative_tween_impl!(
//     rotation,
//     Animator,
//     Transform,
//     TransformRelativeRotationLens,
//     Quat,
//     Quat
// );

// relative_tween_impl!(
//     style_size,
//     Animator,
//     Style,
//     StyleRelativeSizeLens,
//     Vec2,
//     Vec2
// );

// relative_tween_impl!(
//     text_color,
//     Animator,
//     Text,
//     TextRelativeColorLens,
//     Vec<Color>,
//     Color
// );

// relative_tween_impl!(
//     text_alpha,
//     Animator,
//     Text,
//     TextRelativeAlphaLens,
//     Vec<f32>,
//     f32
// );

// relative_tween_impl!(
//     ui_bg_color,
//     Animator,
//     BackgroundColor,
//     UiBackgroundColorLens,
//     Color,
//     Color
// );

// relative_tween_impl!(
//     ui_image_color,
//     Animator,
//     UiImage,
//     UiImageColorLens,
//     Color,
//     Color
// );

// relative_tween_impl!(
//     color_material_color,
//     AssetAnimator,
//     ColorMaterial,
//     ColorMaterialRelativeColorLens,
//     Color,
//     Color
// );

// relative_tween_impl!(tile_color, Animator, TileColor, TileColorLens, Color, Color);

// relative_lens!(Transform, Vec3, TransformRelativeScaleLens, scale);
// relative_lens!(Transform, Vec3, TransformRelativePositionLens, translation);
// relative_lens!(Transform, Quat, TransformRelativeRotationLens, rotation);

// #[derive(Default)]
// pub struct TransformRelativeByPositionLens {
//     start: Vec3,
//     end: Vec3,
//     move_by: Vec3,
// }

// impl TransformRelativeByPositionLens {
//     pub fn new(move_by: Vec3) -> Self {
//         Self {
//             move_by,
//             start: Vec3::ZERO,
//             end: Vec3::ZERO,
//         }
//     }
// }

// impl Lens<Transform> for TransformRelativeByPositionLens {
//     fn lerp(&mut self, target: &mut dyn Targetable<Transform>, ratio: f32) {
//         let value = self.start + (self.end - self.start) * ratio;
//         target.translation = value;
//     }

//     fn update_on_tween_start(
//         &mut self,
//         target: &mut dyn Targetable<Transform>,
//         _direction: TweeningDirection,
//         _times_completed: i32,
//     ) {
//         self.start = target.translation;
//         self.end = target.translation + self.move_by;
//     }
// }

// #[derive(Default)]
// pub struct TextRelativeColorLens {
//     pub start: Option<Vec<Color>>,
//     pub end: Color,
// }

// impl TextRelativeColorLens {
//     pub fn relative(end: Color) -> Self {
//         Self { start: None, end }
//     }
// }

// impl Lens<Text> for TextRelativeColorLens {
//     fn lerp(&mut self, target: &mut dyn Targetable<Text>, ratio: f32) {
//         for i in 0..target.sections.len() {
//             if let Some(col) = self.start.as_ref().unwrap().get(i) {
//                 target.sections[i].style.color = lerp_color(*col, self.end, ratio);
//             }
//         }
//     }

//     fn update_on_tween_start(
//         &mut self,
//         target: &mut dyn Targetable<Text>,
//         _direction: TweeningDirection,
//         _times_completed: i32,
//     ) {
//         self.start
//             .get_or_insert_with(|| target.sections.iter().map(|s| s.style.color).collect());
//     }
// }

// #[derive(Default)]
// pub struct TextRelativeAlphaLens {
//     pub start: Option<Vec<f32>>,
//     pub end: f32,
// }

// impl TextRelativeAlphaLens {
//     pub fn relative(end: f32) -> Self {
//         Self { start: None, end }
//     }
// }

// impl Lens<Text> for TextRelativeAlphaLens {
//     fn lerp(&mut self, target: &mut dyn Targetable<Text>, ratio: f32) {
//         for i in 0..target.sections.len() {
//             if let Some(alpha) = self.start.as_ref().unwrap().get(i) {
//                 target.sections[i]
//                     .style
//                     .color
//                     .set_alpha((*alpha).lerp(self.end, ratio));
//             }
//         }
//     }

//     fn update_on_tween_start(
//         &mut self,
//         target: &mut dyn Targetable<Text>,
//         _direction: TweeningDirection,
//         _times_completed: i32,
//     ) {
//         self.start.get_or_insert_with(|| {
//             target
//                 .sections
//                 .iter()
//                 .map(|s| s.style.color.alpha())
//                 .collect()
//         });
//     }
// }

// relative_lens_struct!(StyleRelativeSizeLens, Vec2);
// impl Lens<Style> for StyleRelativeSizeLens {
//     fn lerp(&mut self, target: &mut dyn Targetable<Style>, ratio: f32) {
//         if let Some(start) = self.start {
//             let size = start + (self.end - start) * ratio;
//             target.width = Val::Px(size.x);
//             target.height = Val::Px(size.y);
//         }
//     }

//     fn update_on_tween_start(
//         &mut self,
//         target: &mut dyn Targetable<Style>,
//         _direction: TweeningDirection,
//         _times_completed: i32,
//     ) {
//         if self.start.is_none() {
//             if let (Val::Px(w), Val::Px(h)) = (target.width, target.height) {
//                 self.start = Some(Vec2::new(w, h));
//             }
//         }
//     }
// }

// color_lens!(Sprite, SpriteRelativeColorLens, color);
// relative_tween_impl!(
//     sprite_color,
//     Animator,
//     Sprite,
//     SpriteRelativeColorLens,
//     Color,
//     Color
// );
// color_lens!(BackgroundColor, UiBackgroundColorLens, 0);
// color_lens!(UiImage, UiImageColorLens, color);
// color_lens!(ColorMaterial, ColorMaterialRelativeColorLens, color);
// color_lens!(TileColor, TileColorLens, 0);

// pub fn lerp_color(from: Color, to: Color, ratio: f32) -> Color {
//     let start = from.to_linear().to_vec4();
//     let end: Vec4 = to.to_linear().to_vec4();
//     let lerped = start.lerp(end, ratio);
//     Color::LinearRgba(LinearRgba::from_vec4(lerped))
// }

// macro_rules! relative_lens_struct {
//     ($lens:ident, $value:ty) => {
//         #[derive(Default)]
//         pub struct $lens {
//             pub(super) start: Option<$value>,
//             pub(super) end: $value,
//         }

//         impl $lens {
//             pub fn relative(end: $value) -> Self {
//                 Self { start: None, end }
//             }

//             pub fn new(start: $value, end: $value) -> Self {
//                 Self {
//                     start: Some(start),
//                     end,
//                 }
//             }
//         }
//     };
// }

// pub(super) use relative_lens_struct;

// macro_rules! color_lens {
//     ($component:ty, $lens:ident, $field:tt) => {
//         relative_lens_struct!($lens, Color);

//         impl Lens<$component> for $lens {
//             fn lerp(&mut self, target: &mut dyn Targetable<$component>, ratio: f32) {
//                 target.$field = lerp_color(
//                     self.start
//                         .expect("Lerping has started so initial values should have been set"),
//                     self.end,
//                     ratio,
//                 );
//             }

//             fn update_on_tween_start(
//                 &mut self,
//                 target: &mut dyn Targetable<$component>,
//                 _direction: TweeningDirection,
//                 _times_completed: i32,
//             ) {
//                 self.start.get_or_insert_with(|| target.$field);
//             }
//         }
//     };
// }

// pub(super) use color_lens;

// macro_rules! relative_lens {
//     ($component:ty, $value:ty, $lens:ident, $field:tt) => {
//         relative_lens_struct!($lens, $value);

//         impl Lens<$component> for $lens {
//             fn lerp(&mut self, target: &mut dyn Targetable<$component>, ratio: f32) {
//                 let start = self.start.unwrap();
//                 let value = start + (self.end - start) * ratio;
//                 target.$field = value;
//             }

//             fn update_on_tween_start(
//                 &mut self,
//                 target: &mut dyn Targetable<$component>,
//                 _direction: TweeningDirection,
//                 _times_completed: i32,
//             ) {
//                 self.start.get_or_insert_with(|| target.$field);
//             }
//         }
//     };
// }

// pub(super) use relative_lens;

// macro_rules! relative_tween_impl {
//     ($name:ident, $animator: ty, $component:ty, $lens:ty, $value_start:ty, $value_end:ty) => {
//         paste::paste! {
//             pub fn [<$name _absolute_tween>](
//                 end: $value_end,
//                 duration_ms: u64,
//                 ease: EaseFunction,
//             ) -> Tween<$component> {
//                 [<$name _tween_impl>](
//                     None,
//                     end,
//                     duration_ms,
//                     ease,
//                 )
//             }

//             pub fn [<$name _tween>](
//                 end: $value_end,
//                 duration_ms: u64,
//                 ease: EaseFunction,
//             ) -> Tween<$component> {
//                 [<$name _tween_impl>](
//                     None,
//                     end,
//                     duration_ms,
//                     ease
//                 )
//             }

//             pub fn [<$name _anim>](
//                 end: $value_end,
//                 duration_ms: u64,
//                 ease: EaseFunction,
//             ) -> $animator<$component> {
//                 $animator::new([<$name _tween_impl>](
//                     None,
//                     end,
//                     duration_ms,
//                     ease,
//                 ))
//             }

//             pub fn [<$name _absolute_anim>](
//                 start: $value_start,
//                 end: $value_end,
//                 duration_ms: u64,
//                 ease: EaseFunction,
//             ) -> $animator<$component> {
//                 $animator::new([<$name _tween_impl>](
//                     Some(start),
//                     end,
//                     duration_ms,
//                     ease,
//                 ))
//             }

//             fn [<$name _tween_impl>](
//                 start: Option<$value_start>,
//                 end: $value_end,
//                 duration_ms: u64,
//                 ease: EaseFunction,
//             ) -> Tween<$component> {
//                 Tween::new(
//                     ease,
//                     Duration::from_millis(duration_ms),
//                     $lens {
//                         start,
//                         end,
//                     },
//                 ).with_completed_event(0)
//             }

//             #[allow(non_camel_case_types)]
//             pub trait [<Tween_ $name TweenCommands>] {
//                 fn [<tween_ $name>](
//                     &mut self,
//                     entity: Entity,
//                     end: $value_end,
//                     duration_ms: u64,
//                     ease: EaseFunction,
//                 );

//                 fn [<tween_ $name _absolute>](
//                     &mut self,
//                     entity: Entity,
//                     start: $value_start,
//                     end: $value_end,
//                     duration_ms: u64,
//                     ease: EaseFunction,
//                 );
//             }

//             impl<'w, 's> [<Tween_ $name TweenCommands>] for Commands<'w, 's> {
//                 fn [<tween_ $name>](
//                     &mut self,
//                     entity: Entity,
//                     end: $value_end,
//                     duration_ms: u64,
//                     ease: EaseFunction,
//                 ) {
//                     if let Some(mut e_cmd) = self.get_entity(entity) {
//                         e_cmd.try_insert([<$name _anim>](end, duration_ms, ease));
//                     }
//                 }

//                 fn [<tween_ $name _absolute>](
//                     &mut self,
//                     entity: Entity,
//                     start: $value_start,
//                     end: $value_end,
//                     duration_ms: u64,
//                     ease: EaseFunction,
//                 ) {
//                     if let Some(mut e_cmd) = self.get_entity(entity) {
//                         e_cmd.try_insert([<$name _absolute_anim>](start, end, duration_ms, ease));
//                     }
//                 }
//             }

//             #[allow(non_camel_case_types)]
//             pub trait [<Tween_ $name TweenEntityCommands>] {
//                 fn [<tween_ $name>](
//                     &mut self,
//                     end: $value_end,
//                     duration_ms: u64,
//                     ease: EaseFunction,
//                 );

//                 fn [<tween_ $name _absolute>](
//                     &mut self,
//                     start: $value_start,
//                     end: $value_end,
//                     duration_ms: u64,
//                     ease: EaseFunction,
//                 );
//             }

//             impl<'w> [<Tween_ $name TweenEntityCommands>] for EntityCommands<'w> {
//                 fn [<tween_ $name>](
//                     &mut self,
//                     end: $value_end,
//                     duration_ms: u64,
//                     ease: EaseFunction,
//                 ) {
//                     self.try_insert([<$name _anim>](end, duration_ms, ease));
//                 }

//                 fn [<tween_ $name _absolute>](
//                     &mut self,
//                     start: $value_start,
//                     end: $value_end,
//                     duration_ms: u64,
//                     ease: EaseFunction,
//                 ) {
//                     self.try_insert([<$name _absolute_anim>](start, end, duration_ms, ease));
//                 }
//             }

//         }
//     };
// }

// pub(super) use relative_tween_impl;
