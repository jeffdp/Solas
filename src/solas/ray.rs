/// Ray
use cgmath::{prelude::*, Vector3};
use random_number::random;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        Ray { origin, direction }
    }

    pub fn point(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }
}

pub fn random_in_unit_sphere() -> Vector3<f64> {
    loop {
        let x: f64 = random!();
        let y: f64 = random!();
        let z: f64 = random!();

        let p = 2.0 * Vector3::new(x, y, z) - Vector3::new(1.0, 1.0, 1.0);

        if p.magnitude() * p.magnitude() < 1.0 {
            return p;
        }
    }
}
