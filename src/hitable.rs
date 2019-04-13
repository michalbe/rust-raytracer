use crate::{
    ray::Ray,
    material::{ Material, Lambert },
    sphere::Sphere,
};
use vec3D::Vec3D;

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3D,
    pub normal: Vec3D,
    pub material: Material,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3D::new(0.0, 0.0, 0.0),
            normal: Vec3D::new(0.0, 0.0, 0.0),
            material: Material::Lambert(Lambert::new(Vec3D::new(0.0, 0.0, 0.0)))
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HitableList {
    pub list: Vec<Sphere>
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for element in self.list.iter() {
            if element.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.t = temp_rec.t;
                rec.normal = temp_rec.normal;
                rec.material = temp_rec.material;
            }
        }

        hit_anything
    }
}
