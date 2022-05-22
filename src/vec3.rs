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
    pub fn rotate(&mut self, alfa: f32, beta: f32) {
        let alfa_sin = alfa.sin();
        let alfa_cos = alfa.cos();
        let beta_sin = beta.sin();
        let beta_cos = beta.cos();
        let temp = Vec3 {
            x: self.x * alfa_cos - self.z * alfa_sin,
            y: self.y,
            z: self.x * alfa_sin + self.z * alfa_cos,
        };
        self.x = temp.x * beta_cos - temp.z * beta_sin;
        self.y = temp.y;
        self.z = temp.x * beta_sin + temp.z * beta_cos;
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
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
        let eps = 1e-5;
        (self.x - other.x).abs() < eps && (self.y - other.y).abs() < eps && (self.z - other.z).abs() < eps
        // self.x.eq(&other.x) && self.y.eq(&other.y) && self.z.eq(&other.z)
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

pub fn square_distance(arg1: Vec3, arg2: Vec3) -> f32 {
    let dx = arg1.x - arg2.x;
    let dy = arg1.y - arg2.y;
    let dz = arg1.z - arg2.z;
    dx * dx + dy * dy + dz * dz
}