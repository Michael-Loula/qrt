mod vec3;

use rand::Rng;
use std::io::{stdout, Write};
use std::rc::Rc;
use vec3::Camera;
use vec3::Dielectric;
use vec3::HittableList;
use vec3::Lambertian;
use vec3::Metal;
use vec3::Sphere;
use vec3::Vec3;

fn main() {
    const W: i32 = 400;
    const AR: f64 = 16.0 / 9.0;
    const H: i32 = (W as f64 / AR) as i32;
    const S: i32 = 50;
    const MD: i32 = 50;

    if let Err(e) = writeln!(stdout(), "P3\n{} {}\n255", W, H) {
        println!("Writing error: {}", e.to_string());
    }

    let mut world = HittableList { v: Vec::new() };

    world.add(Box::new(Sphere {
        center: v3!(0, -100.5, -1),
        radius: 100.0,
        mat_ptr: Rc::new(Lambertian {
            albedo: v3!(0.8, 0.8, 0.0),
        }),
    }));
    world.add(Box::new(Sphere {
        center: v3!(0, 0, -1),
        radius: 0.5,
        mat_ptr: Rc::new(Lambertian {
            albedo: v3!(0.7, 0.3, 0.3),
        }),
    }));
    world.add(Box::new(Sphere {
        center: v3!(-1, 0, -1),
        radius: 0.5,
        mat_ptr: Rc::new(Dielectric { ir: 1.5 }),
    }));
    world.add(Box::new(Sphere {
        center: v3!(-1, 0, -1),
        radius: -0.4,
        mat_ptr: Rc::new(Dielectric { ir: 1.5 }),
    }));
    world.add(Box::new(Sphere {
        center: v3!(1, 0, -1),
        radius: 0.5,
        mat_ptr: Rc::new(Metal {
            albedo: v3!(0.8, 0.6, 0.2),
            fuzz: 0.05,
        }),
    }));

    let cam = Camera {
        ..Default::default()
    };
    let mut rng = rand::thread_rng();
    for j in (0..H).rev() {
        for i in 0..W {
            let mut pixel_color = v3!(0, 0, 0);
            for _ in 0..S {
                let ru: f64 = rng.gen();
                let rv: f64 = rng.gen();
                let u = (i as f64 + ru as f64) / (W - 1) as f64;
                let v = (j as f64 + rv as f64) / (H - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += r.color(&mut world, MD, i, j);
            }
            pixel_color.write_color(S);
        }
    }

    world.clear();
}
