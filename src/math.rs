#![allow(dead_code)]

use std::ops::{Add, Mul};

pub fn inverse_lerp(a: f32, b: f32, progress_factor: f32) -> f32 {
    (progress_factor - a) / (b - a)
}

pub fn inverse_lerp_clamped(a: f32, b: f32, progress_factor: f32) -> f32 {
    inverse_lerp(a, b, progress_factor).clamp(0., 1.)
}

pub fn asymptotic_smoothing<
    T: Mul<f32> + From<<T as Mul<f32>>::Output> + Add<T> + From<<T as Add<T>>::Output>,
>(
    current_value: T,
    target_value: T,
    progress_factor: f32,
) -> T {
    T::from(
        T::from(current_value * (1.0 - progress_factor)) + T::from(target_value * progress_factor),
    )
}

pub fn asymptotic_smoothing_with_delta_time<
    T: Mul<f32> + From<<T as Mul<f32>>::Output> + Add<T> + From<<T as Add<T>>::Output>,
>(
    current_value: T,
    target_value: T,
    progress_factor: f32,
    delta_time_seconds: f32,
) -> T {
    let t = progress_factor * 60. * delta_time_seconds;
    asymptotic_smoothing(current_value, target_value, t)
}
