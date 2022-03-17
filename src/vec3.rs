use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

pub fn cross_product(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3 {
        x: lhs.y * rhs.z - lhs.z * rhs.y,
        y: lhs.z * rhs.x - lhs.x * rhs.z,
        z: lhs.x * rhs.y - lhs.y * rhs.x,
    }
}

pub fn dot_product(lhs: Vec3, rhs: Vec3) -> f32 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

pub fn min_coor(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3 {
        x: lhs.x.min(rhs.x),
        y: lhs.y.min(rhs.y),
        z: lhs.z.min(rhs.z),
    }
}

pub fn max_coor(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3 {
        x: lhs.x.max(rhs.x),
        y: lhs.y.max(rhs.y),
        z: lhs.z.max(rhs.z),
    }
}
