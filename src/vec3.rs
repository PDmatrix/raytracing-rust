use std::ops::*;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(f0: f32, f1: f32, f2: f32) -> Self {
        Self { e: [f0, f1, f2] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }

    pub fn g(&self) -> f32 {
        self.e[1]
    }

    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn make_unit_vector(&mut self) {
        let k: f32 = 1.0 / self.squared_length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    let f0 = v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1];
    let f1 = v1.e[2] * v2.e[0] - v1.e[0] * v2.e[2];
    let f2 = v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0];

    Vec3::new(f0, f1, f2)
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let f0 = self.e[0] + rhs.e[0];
        let f1 = self.e[1] + rhs.e[1];
        let f2 = self.e[2] + rhs.e[2];

        Self::new(f0, f1, f2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let f0 = self.e[0] - rhs.e[0];
        let f1 = self.e[1] - rhs.e[1];
        let f2 = self.e[2] - rhs.e[2];

        Self::new(f0, f1, f2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let f0 = self.e[0] * rhs.e[0];
        let f1 = self.e[1] * rhs.e[1];
        let f2 = self.e[2] * rhs.e[2];

        Self::new(f0, f1, f2)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[2];
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, t: f32) -> Self::Output {
        let f0 = t * self.e[0];
        let f1 = t * self.e[1];
        let f2 = t * self.e[2];

        Self::new(f0, f1, f2)
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let f0 = self.e[0] / rhs.e[0];
        let f1 = self.e[1] / rhs.e[1];
        let f2 = self.e[2] / rhs.e[2];

        Self::new(f0, f1, f2)
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        rhs / self
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.e[0] /= rhs.e[0];
        self.e[1] /= rhs.e[1];
        self.e[2] /= rhs.e[2];
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, t: f32) -> Self::Output {
        let f0 = self.e[0] / t;
        let f1 = self.e[1] / t;
        let f2 = self.e[2] / t;

        Self::new(f0, f1, f2)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        let k = 1.0 / t;

        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.e[index]
    }
}
