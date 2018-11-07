use std::ops::*;

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }
    pub fn r(&self) -> f32 {
        self.0
    }
    pub fn g(&self) -> f32 {
        self.1
    }
    pub fn b(&self) -> f32 {
        self.2
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn to_unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3(self * v.0, self * v.1, self * v.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3(self.0 * v.0, self.1 * v.1, self.2 * v.2)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, r: f32) -> Vec3 {
        (1.0 / r) * self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Ray {
        Ray {
            origin: o,
            direction: d,
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
