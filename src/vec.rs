#[derive(Debug)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    fn x(&self) -> f32 {
        self.0
    }
    fn y(&self) -> f32 {
        self.1
    }
    fn z(&self) -> f32 {
        self.2
    }
    fn r(&self) -> f32 {
        self.0
    }
    fn g(&self) -> f32 {
        self.1
    }
    fn b(&self) -> f32 {
        self.2
    }

    fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    fn squared_length(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Ray {
        Ray {
            origin: o,
            direction: o,
        }
    }

    fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use vec::Vec3;

    #[test]
    fn prims() {
        let v = Vec3(1.0, 2.0, 3.0);
        assert_eq!(1.0, v.x());
        assert_eq!(2.0, v.y());
        assert_eq!(3.0, v.z());

        assert_eq!(1.0, v.r());
        assert_eq!(2.0, v.g());
        assert_eq!(3.0, v.b());
    }

    #[test]
    fn test_length() {
        let v = Vec3(1.0, 2.0, 3.0);
        assert_eq!((14.0_f32).sqrt(), v.length());
    }

    #[test]
    fn sq_length() {
        let v = Vec3(1.0, 2.0, 3.0);
        assert_eq!(14.0_f32, v.squared_length());
    }
}
