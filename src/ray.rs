use vec3D::Vec3D;

pub struct Ray {
    pub origin: Vec3D,
    pub direction: Vec3D,
}

impl Ray {
    pub fn new(origin: Vec3D, direction: Vec3D) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3D {
        self.origin + self.direction * t
    }
}
