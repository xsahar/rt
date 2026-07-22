mod vec3;
mod ray;
mod hit;
mod hittable;
mod plane;
mod cube;
mod sphere;
mod world;
mod cylinder;
mod camera;

use vec3::Vec3;
use cube::Cube;
use camera::Camera;
use ray::Ray;
use plane::Plane;
use sphere::Sphere;
use world::World;
use cylinder::Cylinder;
use std::env;

fn is_in_shadow(point: Vec3, normal: Vec3, world: &World, light_direction: Vec3) -> bool {
    let shadow_origin = point + normal * 0.001;
    let shadow_ray = Ray::new(shadow_origin, light_direction);
    world.hit_record(&shadow_ray, 0.001, f64::INFINITY).is_some()
}

fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot(v, normal) * normal
}

fn ray_color(r: &Ray, world: &World, depth: i32, ambient_light: f64) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit_record(r, 0.001, f64::INFINITY) {
        let light_direction = Vec3::new(-0.8, 1.2, 0.8).normalize();
        
        let brightness = if is_in_shadow(hit.point, hit.normal, world, light_direction) {
            ambient_light * 0.4
        } else {
            ambient_light + (1.0 - ambient_light) * Vec3::dot(hit.normal, light_direction).max(0.0)
        };

        let shaded_color = brightness * hit.albedo;
        let reflected_direction = reflect(r.direction.normalize(), hit.normal).normalize();
        let reflected_origin = hit.point + hit.normal * 0.001;
        let reflected_ray = Ray::new(reflected_origin, reflected_direction);
        let reflected_color = ray_color(&reflected_ray, world, depth - 1, ambient_light);

        return 0.85 * shaded_color + 0.15 * reflected_color;
    }

    let t = 0.5 * (r.direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let active_scene = env::args()
        .nth(1)
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(3);

    let aspect_ratio = 800.0 / 600.0;
    let image_width = 400; 
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let mut world = World::new();
    let camera: Camera;
    let mut ambient_light = 0.25;

    match active_scene {
        1 => {
            // Scene 1: Standalone Sphere
            camera = Camera::new(
                Vec3::new(0.0, 0.0, 1.5), 
                Vec3::new(0.0, 0.0, -1.2), 
                Vec3::new(0.0, 1.0, 0.0), 
                40.0, 
                aspect_ratio
            );
            world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, Vec3::new(1.0, 0.2, 0.2)));
        }
        2 => {
            // Scene 2: Plane + Cube with lower brightness
            ambient_light = 0.08; 
            camera = Camera::new(
                Vec3::new(0.0, 0.2, 2.2), 
                Vec3::new(0.0, -0.1, -1.2), 
                Vec3::new(0.0, 1.0, 0.0), 
                40.0, 
                aspect_ratio
            );
            world.add(Plane::new(Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.2, 0.8, 0.2)));
            world.add(Cube::new(Vec3::new(0.0, -0.15, -1.2), 0.5, Vec3::new(0.2, 0.4, 1.0)));
        }
        4 => {
            // Scene 4: Alternate Camera Perspective
            camera = Camera::new(
                Vec3::new(2.2, 1.2, 1.8),   
                Vec3::new(0.0, -0.2, -1.2), 
                Vec3::new(0.0, 1.0, 0.0), 
                45.0, 
                aspect_ratio
            );
            world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, Vec3::new(1.0, 0.2, 0.2)));
            world.add(Plane::new(Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.2, 0.8, 0.2)));
            world.add(Cube::new(Vec3::new(1.1, -0.15, -1.2), 0.5, Vec3::new(0.2, 0.4, 1.0)));
            world.add(Cylinder::new(Vec3::new(-1.1, -0.25, -1.2), 0.25, -0.25, 0.5, Vec3::new(1.0, 0.8, 0.1)));
        }
        _ => {
            // Scene 3: Natural Lens FOV (40 deg) pulled back
            camera = Camera::new(
                Vec3::new(0.0, 0.2, 2.2), 
                Vec3::new(0.0, -0.1, -1.2), 
                Vec3::new(0.0, 1.0, 0.0), 
                40.0, 
                aspect_ratio
            );
            world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, Vec3::new(1.0, 0.2, 0.2)));
            world.add(Plane::new(Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.2, 0.8, 0.2)));
            world.add(Cube::new(Vec3::new(1.1, -0.15, -1.2), 0.5, Vec3::new(0.2, 0.4, 1.0)));
            world.add(Cylinder::new(Vec3::new(-1.1, -0.25, -1.2), 0.25, -0.25, 0.5, Vec3::new(1.0, 0.8, 0.1)));
        }
    }

    println!("P3\n{} {}\n255", image_width, image_height);

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let u = x as f64 / (image_width - 1) as f64;
            let v = y as f64 / (image_height - 1) as f64;
            
            let r = camera.get_ray(u, v);
            let mut pixel_color = ray_color(&r, &world, 3, ambient_light);
            
            // Gamma Correction
            pixel_color.x = pixel_color.x.clamp(0.0, 1.0).sqrt();
            pixel_color.y = pixel_color.y.clamp(0.0, 1.0).sqrt();
            pixel_color.z = pixel_color.z.clamp(0.0, 1.0).sqrt();

            let ir = (255.999 * pixel_color.x) as i32;
            let ig = (255.999 * pixel_color.y) as i32;
            let ib = (255.999 * pixel_color.z) as i32;
            
            println!("{} {} {}", ir, ig, ib);
        }
    }
}