use crate::vec3::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: &Vec3, b: &Vec3) -> Ray {
        Ray { a: *a, b: *b }
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn point_on_parameter(&self, t: f32) -> Vec3 {
        self.a + self.b * t
    }
}
