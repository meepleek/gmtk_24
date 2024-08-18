#![allow(dead_code)]

use bevy::utils::HashSet;

use crate::prelude::*;

pub trait Vec2Ext {
    fn to_quat(self) -> Quat;
    fn to_rot2(self) -> Rot2;
}

impl Vec2Ext for Vec2 {
    fn to_quat(self) -> Quat {
        match Dir2::new(self) {
            Ok(dir) => Quat::from_rotation_z(dir.to_angle()),
            Err(_) => Quat::IDENTITY,
        }
    }

    fn to_rot2(self) -> Rot2 {
        Rot2::radians(self.to_angle())
    }
}

pub trait QuatExt {
    fn to_rot2(self) -> Rot2;
    fn z_angle_rad(&self) -> f32;
}

impl QuatExt for Quat {
    fn to_rot2(self) -> Rot2 {
        Rot2::radians(self.z_angle_rad())
    }

    fn z_angle_rad(&self) -> f32 {
        self.to_scaled_axis().z
    }
}

pub trait Rot2Ext {
    fn to_quat(self) -> Quat;
}

impl Rot2Ext for Rot2 {
    fn to_quat(self) -> Quat {
        Quat::from_rotation_z(self.as_radians())
    }
}

pub trait Dir2Ext {
    fn to_quat(self) -> Quat;
}

impl Rot2Ext for Dir2 {
    fn to_quat(self) -> Quat {
        Quat::from_rotation_z(self.to_angle())
    }
}

pub trait TransExt {
    fn zero_scale_2d() -> Transform;
}

impl TransExt for Transform {
    fn zero_scale_2d() -> Transform {
        Transform::from_scale(Vec2::ZERO.extend(1.))
    }
}

pub trait RandExt {
    fn rotation(&mut self) -> Rot2;
    fn rotation_range_degrees(&mut self, degrees: f32) -> Rot2;
    fn direction(&mut self) -> Dir2;
}

impl RandExt for ThreadRng {
    fn rotation(&mut self) -> Rot2 {
        self.rotation_range_degrees(360.0)
    }

    fn rotation_range_degrees(&mut self, degrees: f32) -> Rot2 {
        Rot2::degrees(self.gen_range(-degrees..degrees))
    }

    fn direction(&mut self) -> Dir2 {
        Dir2::new(self.rotation() * Vec2::X).expect("Non-zero direction")
    }
}

pub trait EventReaderExt<T> {
    fn read_only_last(&mut self) -> Option<&T>;
    fn clear_any(&mut self) -> bool;
}

impl<'w, 's, T: Event> EventReaderExt<T> for EventReader<'w, 's, T> {
    fn read_only_last(&mut self) -> Option<&T> {
        let mut res = None;
        for ev in self.read() {
            res = Some(ev)
        }
        res
    }

    fn clear_any(&mut self) -> bool {
        if !self.is_empty() {
            self.clear();
            true
        } else {
            false
        }
    }
}

pub trait GridCoordsExt {
    fn to_world(&self) -> Vec3;
    fn to_world_with_z(&self, z: f32) -> Vec3;
    fn x() -> Self;
    fn neg_x() -> Self;
    fn y() -> Self;
    fn neg_y() -> Self;
    fn up(&self) -> Self;
    fn down(&self) -> Self;
    fn left(&self) -> Self;
    fn right(&self) -> Self;
    fn neighbours(&self) -> Vec<GridCoords>;
}

impl GridCoordsExt for GridCoords {
    fn to_world(&self) -> Vec3 {
        self.to_world_with_z(0.)
    }

    fn to_world_with_z(&self, z: f32) -> Vec3 {
        bevy_ecs_ldtk::utils::grid_coords_to_translation(*self, IVec2::splat(TILE_SIZE)).extend(z)
    }

    fn y() -> Self {
        Self::new(0, 1)
    }

    fn neg_y() -> Self {
        Self::new(0, -1)
    }

    fn x() -> Self {
        Self::new(1, 0)
    }

    fn neg_x() -> Self {
        Self::new(-1, 0)
    }

    fn up(&self) -> Self {
        Self::new(self.x, (self.y - 1).max(0))
    }

    fn down(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    fn left(&self) -> Self {
        Self::new((self.x + 1).max(0), self.y)
    }

    fn right(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }

    fn neighbours(&self) -> Vec<GridCoords> {
        let res = HashSet::from([self.up(), self.down(), self.left(), self.right()]);
        res.into_iter().collect()
    }
}
