mod prelude;
mod ray;

use std::fs::File;
use std::io::{BufWriter, Write};
use prelude::*;
use ray::*;

fn hit_sphere(center: Point3f, radius: f64, r: &Ray) -> bool {
    let oc = center - r.origin();
    let a = r.direction().dot(&r.direction());
    let b = -2.0 * r.direction().dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn pixel_color(ray: &Ray) -> Vec3f {
    if (hit_sphere(Point3f::new(0.0, 0.0, -1.0), 0.5, ray)) {
        Vec3f::new(1.0, 0.0, 0.0)
    } else {
        let unit_direction = ray.direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vec3f::new(1.0, 1.0, 1.0) + a * Vec3f::new(0.5, 0.7, 1.0)
    }
}

pub fn color_to_rgb(color: Vec3f) -> [u8; 3] {
    let r = color.x;
    let g = color.y;
    let b = color.z;
    [
        (256.0 * r.clamp(0.0, 0.999)) as u8,
        (256.0 * g.clamp(0.0, 0.999)) as u8,
        (256.0 * b.clamp(0.0, 0.999)) as u8,
    ]
}

fn main() -> std::io::Result<()> {
    let file = File::create("/tmp/output.ppm")?;
    let mut file = BufWriter::new(file);

    let scale: u32 = 120;
    let width: u32 = 16 * scale;
    let height: u32 = 9 * scale;
    let focal_length: f64 = 1.0;
    let viewport_width: f64 = 3.55;
    let viewport_height: f64 = 2.0;
    let viewport_u = Vec3f::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3f::new(0.0, -viewport_height, 0.0);
    let camera_center = Point3f::new(0.0,0.0,0.0);
    let pixel_delta_u = viewport_u / (width as f64);
    let pixel_delta_v = viewport_v / (height as f64);
    let viewport_upper_left = camera_center
        - Vec3f::new(0.0,0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left
        + 0.5 * (pixel_delta_u + pixel_delta_v);

    writeln!(file, "P6")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "255")?;

    for y in 0..height {
        for x in 0..width {
            let pixel_center = pixel00_loc
                + (x as f64 * pixel_delta_u)
                + (y as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let ray = Ray::new(camera_center, ray_direction);
            let color = pixel_color(&ray);
            let rgb = color_to_rgb(color);
            file.write_all(&rgb);
        }
    }

    Ok(())
}
