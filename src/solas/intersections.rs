use super::*;
/// Intersections
use cgmath::{prelude::*, Vector3};

#[derive(Copy, Clone)]
pub struct Hit {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: Material,
}

impl Hit {
    pub fn new(t: f64, p: Vector3<f64>, normal: Vector3<f64>, material: Material) -> Hit {
        Hit {
            t,
            p,
            normal,
            material,
        }
    }
}

pub fn hit(ray: &Ray, min: f64, max: f64, objects: &[Sphere]) -> Option<Hit> {
    let mut closest_hit: Option<Hit> = None;

    for object in objects {
        // Check to see if we actually intersect with this object.
        if let Some(new_hit) = object.hit(&ray, min, max) {
            if closest_hit.is_none() {
                // If we don't already have a closests hit, this is it.
                closest_hit = Some(new_hit);
            } else {
                // See if the new hit is closer to us than the closest one.
                if new_hit.t < closest_hit.unwrap().t {
                    closest_hit = Some(new_hit);
                }
            }
        }
    }

    closest_hit
}

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn hit(&self, ray: &super::Ray, min: f64, max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius.powi(2);
        let discriminant = b * b - a * c;

        if discriminant <= 0.0 {
            return None;
        }

        let temp = (-b - discriminant.sqrt()) / a;
        if temp < max && temp > min {
            let point = ray.point(temp);
            let normal = (point - self.center) / self.radius;

            return Some(Hit::new(temp, point, normal, self.material));
        }

        None
    }
}
