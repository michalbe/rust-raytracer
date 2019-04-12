use crate::{
    hitable::HitRecord,
    ray::Ray,
    utils::{
        random_in_unit_sphere,
        reflect,
    },
};

use vec3D::Vec3D;

pub trait Scatter {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3D, scattered: &mut Ray) -> bool;
}

#[derive(Clone, Copy)]
pub enum Material {
    Lambert(Lambert),
    Metal(Metal),
}

impl Scatter for Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3D, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambert(material) => material.scatter(ray_in, rec, attenuation, scattered),
            Material::Metal(material) => material.scatter(ray_in, rec, attenuation, scattered),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Lambert {
    albedo: Vec3D,
}

impl Lambert {
    pub fn new(albedo: Vec3D) -> Lambert {
        Lambert {
            albedo
        }
    }
}
impl Scatter for Lambert {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3D, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let new_scattered = Ray::new(rec.p, target - rec.p);
        scattered.direction = new_scattered.direction;
        scattered.origin = new_scattered.origin;
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        true
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Vec3D,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vec3D, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0)
        }
    }
}
impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3D, scattered: &mut Ray) -> bool {
        let reflected = reflect(ray_in.direction.unit(), rec.normal);
        let new_scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * self.fuzz);
        scattered.direction = new_scattered.direction;
        scattered.origin = new_scattered.origin;
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        scattered.direction.dot(rec.normal) > 0.0
    }
}
