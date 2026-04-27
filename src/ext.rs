#![allow(dead_code)]

use crate::prelude::*;

pub trait Vec3Ext {
    fn to_grid_coords(&self) -> GridCoords;
}

impl Vec3Ext for Vec3 {
    fn to_grid_coords(&self) -> GridCoords {
        bevy_ecs_ldtk::utils::translation_to_grid_coords(
            self.truncate(),
            IVec2::splat(TILE_SIZE as i32),
        )
    }
}

pub trait GridCoordsExt {
    fn to_world(&self) -> Vec3;
    fn to_world_with_z(&self, z: f32) -> Vec3;
    fn to_vec2(&self) -> Vec2;
    fn distance(&self, rhl: &Self) -> f32;
    fn x() -> Self;
    fn neg_x() -> Self;
    fn y() -> Self;
    fn neg_y() -> Self;
    fn up(&self) -> Self;
    fn down(&self) -> Self;
    fn left(&self) -> Self;
    fn right(&self) -> Self;
    fn neighbours(&self) -> Vec<GridCoords>;
    fn radius(&self, radius: u32, skip_center: bool) -> Vec<GridCoords>;
}

impl GridCoordsExt for GridCoords {
    fn to_world(&self) -> Vec3 {
        self.to_world_with_z(0.)
    }

    fn to_world_with_z(&self, z: f32) -> Vec3 {
        bevy_ecs_ldtk::utils::grid_coords_to_translation(*self, IVec2::splat(TILE_SIZE as i32))
            .extend(z)
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
        Self::new(self.x, self.y + 1)
    }

    fn down(&self) -> Self {
        Self::new(self.x, (self.y - 1).max(0))
    }

    fn left(&self) -> Self {
        Self::new((self.x - 1).max(0), self.y)
    }

    fn right(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }

    fn neighbours(&self) -> Vec<GridCoords> {
        let res = HashSet::from([self.up(), self.down(), self.left(), self.right()]);
        res.into_iter().collect()
    }

    fn radius(&self, radius: u32, skip_center: bool) -> Vec<GridCoords> {
        let mut res = vec![];
        let radius = radius as i32;
        let x_min = (self.x - radius).max(0);
        let x_max = self.x + radius;
        let y_min = (self.y - radius).max(0);
        let y_max = self.y + radius;
        let radius_sq = radius.pow(2) as f32;

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                if skip_center && x == self.x && y == self.y {
                    continue;
                }

                let distance_sq = Vec2::new(x as f32, y as f32).distance_squared(self.to_vec2());
                if distance_sq <= radius_sq {
                    res.push(GridCoords::new(x, y));
                }
            }
        }

        res
    }

    fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    fn distance(&self, rhl: &Self) -> f32 {
        self.to_vec2().distance(rhl.to_vec2())
    }
}
