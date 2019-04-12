use crate::{
    hitable::HitRecord,
    ray::Ray,
    utils::{
        random_in_unit_sphere,
        reflect,
        refract,
        schlick,
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
    Dielectric(Dielectric),
}

impl Scatter for Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3D, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambert(material) => material.scatter(ray_in, rec, attenuation, scattered),
            Material::Metal(material) => material.scatter(ray_in, rec, attenuation, scattered),
            Material::Dielectric(material) => material.scatter(ray_in, rec, attenuation, scattered),
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

#[derive(Clone, Copy)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric {
            ref_idx
        }
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3D, scattered: &mut Ray) -> bool {
        let outward_normal: Vec3D;
        let ni_over_nt: f64;
        let mut refracted = Vec3D::new(0.0, 0.0, 0.0);
        let reflect_prob: f64;
        let cosine: f64;

        let reflected = reflect(ray_in.direction.unit(), rec.normal);
        attenuation.x = 1.0;
        attenuation.y = 1.0;
        attenuation.z = 1.0;

        if ray_in.direction.dot(rec.normal) > 0.0 {
            outward_normal = rec.normal * -1.0;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * ray_in.direction.dot(rec.normal) / ray_in.direction.mag();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = ray_in.direction.dot(rec.normal) * -1.0 / ray_in.direction.mag();
        }

        if refract(ray_in.direction, outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            reflect_prob = 1.0;
        }

        if rand::random::<f64>() < reflect_prob {
            // let new_scattered = Ray::new(rec.p, reflected);
            scattered.direction = reflected;
            scattered.origin = rec.p;
        } else {
            // let new_scattered = Ray::new(rec.p, refracted);
            scattered.direction = refracted;
            scattered.origin = rec.p;
        }

        true
    }
}
