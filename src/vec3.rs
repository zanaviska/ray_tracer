use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const MAX: Vec3 = Vec3 {
        x: f32::MAX,
        y: f32::MAX,
        z: f32::MAX,
    };
    pub const MIN: Vec3 = Vec3 {
        x: f32::MIN,
        y: f32::MIN,
        z: f32::MIN,
    };
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

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x.lt(&other.x) {
            return Some(std::cmp::Ordering::Less);
        }
        if self.x.eq(&other.x) && self.y.lt(&other.y) {
            return Some(std::cmp::Ordering::Less);
        }
        if self.x.eq(&other.x) && self.y.eq(&other.y) && self.z.lt(&other.z) {
            return Some(std::cmp::Ordering::Less);
        }
        if self.x.eq(&other.x) && self.y.eq(&other.y) && self.z.eq(&other.z) {
            return Some(std::cmp::Ordering::Equal);
        }
        return Some(std::cmp::Ordering::Greater);
    }
    fn le(&self, other: &Self) -> bool {
        self.partial_cmp(other) != Some(std::cmp::Ordering::Greater)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y) && self.z.eq(&other.z)
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
