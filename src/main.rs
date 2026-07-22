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
        let light_direction = Vec3::new(-1.0, 1.0, -1.0).normalize();
        
        let brightness = if is_in_shadow(hit.point, hit.normal, world, light_direction) {
            ambient_light * 0.5
        } else {
            ambient_light + (1.0 - ambient_light) * Vec3::dot(hit.normal, light_direction).max(0.0)
        };

        let shaded_color = brightness * hit.albedo;
        let reflected_direction = reflect(r.direction.normalize(), hit.normal).normalize();
        let reflected_origin = hit.point + hit.normal * 0.001;
        let reflected_ray = Ray::new(reflected_origin, reflected_direction);
        let reflected_color = ray_color(&reflected_ray, world, depth - 1, ambient_light);

        return 0.75 * shaded_color + 0.25 * reflected_color;
    }

    let t = 0.5 * (r.direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let active_scene = env::args()
        .nth(1)
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(3);

    // Image resolution configuration (Adjust image_width to change resolution)
    let aspect_ratio = 800.0 / 600.0;
    let image_width = 400; 
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let mut world = World::new();
    let camera: Camera;
    let mut ambient_light = 0.2; // Default brightness level

    match active_scene {
        1 => {
            // Image 1: Sphere Only
            camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), 90.0, aspect_ratio);
            world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.1), 0.5, Vec3::new(0.9, 0.3, 0.3)));
        }
        2 => {
            // Image 2: Plane + Cube with lower brightness (0.05 vs 0.2)
            ambient_light = 0.05; 
            camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), 90.0, aspect_ratio);
            world.add(Plane::new(Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.3, 0.9, 0.3)));
            world.add(Cube::new(Vec3::new(0.0, -0.15, -1.2), 0.5, Vec3::new(0.3, 0.3, 0.9)));
        }
        4 => {
            // Image 4: All 4 Objects from a DIFFERENT Camera Perspective
            camera = Camera::new(
                Vec3::new(1.2, 0.8, 0.5),   // Elevated side lookfrom
                Vec3::new(0.0, -0.2, -1.1), // Lookat center of scene
                Vec3::new(0.0, 1.0, 0.0), 
                75.0, 
                aspect_ratio
            );
            world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.1), 0.5, Vec3::new(0.9, 0.3, 0.3)));
            world.add(Plane::new(Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.3, 0.9, 0.3)));
            world.add(Cube::new(Vec3::new(0.8, -0.15, -1.2), 0.5, Vec3::new(0.3, 0.3, 0.9)));
            world.add(Cylinder::new(Vec3::new(-0.8, -0.25, -1.2), 0.25, -0.25, 0.5, Vec3::new(0.9, 0.8, 0.2)));
        }
        _ => {
            // Image 3: All 4 Objects (Sphere, Plane, Cube, Cylinder) from Default Perspective
            camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), 90.0, aspect_ratio);
            world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.1), 0.5, Vec3::new(0.9, 0.3, 0.3)));
            world.add(Plane::new(Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.3, 0.9, 0.3)));
            world.add(Cube::new(Vec3::new(0.8, -0.15, -1.2), 0.5, Vec3::new(0.3, 0.3, 0.9)));
            world.add(Cylinder::new(Vec3::new(-0.8, -0.25, -1.2), 0.25, -0.25, 0.5, Vec3::new(0.9, 0.8, 0.2)));
        }
    }

    // Output Header
    println!("P3\n{} {}\n255", image_width, image_height);

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let u = x as f64 / (image_width - 1) as f64;
            let v = y as f64 / (image_height - 1) as f64;
            
            let r = camera.get_ray(u, v);
            let pixel_color = ray_color(&r, &world, 3, ambient_light);
            
            let ir = (255.999 * pixel_color.x) as i32;
            let ig = (255.999 * pixel_color.y) as i32;
            let ib = (255.999 * pixel_color.z) as i32;
            
            println!("{} {} {}", ir, ig, ib);
        }
    }
}