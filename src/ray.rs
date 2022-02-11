use crate::vec3::Vec3;
use crate::Point3D;
use derive_builder::Builder;

#[derive(Builder, Debug, PartialEq)]
pub struct Ray {
    origin: Point3D,
    direction: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Ray {
            origin: Point3D::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn origin(&self) -> &Point3D {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3D {
        self.direction * t + self.origin
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds() {
        let ray = Ray::new();
        let builder = RayBuilder::default()
            .origin(Point3D::new(0.0, 0.0, 0.0))
            .direction(Vec3::new(0.0, 0.0, 0.0))
            .build()
            .expect("failed to build");

        assert_eq!(ray, builder);
    }
}
