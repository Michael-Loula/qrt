mod vec3;

    
use vec3::Vec3;
use vec3::Sphere;
use vec3::Camera;
use vec3::HittableList;
use std::io::{Write,stdout};
use rand::distributions::{Distribution, Uniform};

fn main() {
    const W : i32 = 400;
    const AR : f64 = 16.0/9.0;
    const H : i32 = (W as f64/AR) as i32;
    const S : i32 = 100;
    
    if let Err(e) = writeln!(stdout(),"P3\n{} {}\n255",W,H) {
        println!("Writing error: {}", e.to_string()); 
    }

    let vp_height = 2.0;
    let vp_width = AR*vp_height;
    let focal_length = 1.0;

    let origin = v3!(0,0,0);
    let horiz = v3!(vp_width,0,0);
    let vert = v3!(0,vp_height,0);
    let llc = origin - horiz/2.0 - vert/2.0 - v3!(0,0,focal_length);
    let mut world = HittableList {v: Vec::new()};
    let s1 = Sphere {center: v3!(0,0,-1), radius: 0.5};
    let s2 = Sphere {center: v3!(0,-100.5,-1), radius: 100.0};

    world.add(Box::<Sphere>::new(s1));
    world.add(Box::<Sphere>::new(s2));

    let cam =  Camera {..Default::default() };

    for j in (0..H).rev() {
        for i in 0..W {
            
            let mut pixel_color = v3!(0,0,0);
            for _ in 0..S {
                let mut rngu = rand::thread_rng();
                let mut rngv = rand::thread_rng();
                let normal = Uniform::from(0.0..1.0);

                let u = (i as f64 + normal.sample(&mut rngu) as f64)/(W-1) as f64;
                let v = (j as f64 + normal.sample(&mut rngv) as f64)/(H-1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += r.color(&mut world);


            }
                pixel_color
                .write_color(S);
        }  
    }
    

}


