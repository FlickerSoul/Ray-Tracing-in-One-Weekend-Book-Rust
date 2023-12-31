use crate::math_traits::InnerProduct;
use crate::objects::Hittable;
use crate::vec3;

pub struct Ray {
    pub origin: vec3::Point3,
    pub direction: vec3::Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: vec3::Point3, direction: vec3::Vec3) -> Self {
        Ray {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn with_timing(origin: vec3::Point3, direction: vec3::Vec3, time: f64) -> Self {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> vec3::Point3 {
        self.origin + t * self.direction
    }
}

#[inline(always)]
pub fn background(ray: &Ray) -> vec3::Color {
    let unit_direction = ray.direction.unit();

    let t = 0.5 * (unit_direction.y() + 1.0); // going to work with focal length

    (1.0 - t) * vec3::Color::new(1.0, 1.0, 1.0) + t * vec3::Color::new(0.5, 0.7, 1.0)
}

#[inline(always)]
pub fn ray_color(
    ray: &Ray,
    world: &crate::WorldType,
    iter: u32,
    background: &vec3::Color,
) -> vec3::Color {
    if iter <= 0 {
        return vec3::Color::zero();
    }

    if let Some(record) = world.hit(&ray, 0.001, f64::INFINITY) {
        let emitted = record.material.emit(record.u, record.v, &record.hit_point);
        if let Some((color, out_ray)) = record.material.scatter(&ray, &record) {
            emitted + ray_color(&out_ray, world, iter - 1, background) * color
        } else {
            emitted
        }
    } else {
        *background
    }
}

#[allow(dead_code)]
#[inline(always)]
pub fn ray_color_unit_vector(ray: &Ray, world: &crate::WorldType, iter: u32) -> vec3::Color {
    if iter <= 0 {
        return vec3::Color::zero();
    }

    if let Some(record) = world.hit(&ray, 0.001, f64::INFINITY) {
        let target = record.hit_point + record.normal + vec3::Vec3::random_unit_vector();
        0.5 * ray_color_unit_vector(
            &Ray::new(record.hit_point, target - record.hit_point),
            world,
            iter - 1,
        )
    } else {
        background(&ray)
    }
}

#[allow(dead_code)]
#[inline(always)]
pub fn ray_color_hemisphere(ray: &Ray, world: &crate::WorldType, iter: u32) -> vec3::Color {
    if iter <= 0 {
        return vec3::Color::zero();
    }

    if let Some(record) = world.hit(&ray, 0.001, f64::INFINITY) {
        let target = record.hit_point + vec3::Vec3::random_in_hemisphere(&record.normal);
        0.5 * ray_color_hemisphere(
            &Ray::new(record.hit_point, target - record.hit_point),
            world,
            iter - 1,
        )
    } else {
        background(&ray)
    }
}

#[allow(dead_code)]
#[inline(always)]
pub fn ray_color_unit_sphere(ray: &Ray, world: &crate::WorldType, iter: u32) -> vec3::Color {
    if iter <= 0 {
        return vec3::Color::zero();
    }

    if let Some(record) = world.hit(&ray, 0.001, f64::INFINITY) {
        let target = record.hit_point + record.normal + vec3::Vec3::random_in_unit_sphere();
        0.5 * ray_color_unit_sphere(
            &Ray::new(record.hit_point, target - record.hit_point),
            world,
            iter - 1,
        )
    } else {
        background(&ray)
    }
}
