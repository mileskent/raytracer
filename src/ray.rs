use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    orig: Point3f,
    dir: Vec3f,
}

impl Ray {
    pub fn new(orig: Point3f, dir: Vec3f) -> Self {
        Self {orig, dir}
    }

    pub fn origin(&self) -> Point3f {
        self.orig
    }

    pub fn direction(&self) -> Vec3f {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3f {
        self.orig + self.dir * t
    }
}
