mod vec3;

    
use vec3::Vec3;
use vec3::Sphere;
use vec3::Camera;
use vec3::Metal;
use vec3::Lambertian;
use vec3::HittableList;
use std::io::{Write,stdout};
use rand::distributions::{Distribution, Uniform};
use std::rc::Rc;

fn main() {
    const W : i32 = 400;
    const AR : f64 = 16.0/9.0;
    const H : i32 = (W as f64/AR) as i32;
    const S : i32 = 100;
    const MD : i32 = 50;
    
    if let Err(e) = writeln!(stdout(),"P3\n{} {}\n255",W,H) {
        println!("Writing error: {}", e.to_string()); 
    }


    let mut world = HittableList {v: Vec::new()};

    let m1 = Rc::<Metal>::new(Metal {albedo : v3!(0.8, 0.8, 0.0)});
    let s1 = Sphere {center: v3!(0,0,-1), radius: 0.5, mat_ptr: m1};
    let m2 = Rc::<Lambertian>::new(Lambertian {albedo : v3!(0.7, 0.3, 0.3)});
    let s2 = Sphere {center: v3!(0,-100.5,-1), radius: 100.0, mat_ptr: m2};
    //let s3 = Sphere {center: v3!(-1,0.75,-1.5), radius: 0.5, mat_ptr: None};
    //let s4 = Sphere {center: v3!(1,0.75,-1.5), radius: 0.5, mat_ptr: None};

    //let e1 = Sphere {center: v3!(-0.11,0,-0.5), radius: 0.07, mat_ptr: None};
    //let e2 = Sphere {center: v3!(0.11,0,-0.5), radius: 0.07, mat_ptr: None};

    world.add(Box::<Sphere>::new(s1));
    world.add(Box::<Sphere>::new(s2));
    //world.add(Box::<Sphere>::new(s3));
    //world.add(Box::<Sphere>::new(s4));
    //world.add(Box::<Sphere>::new(e1));
    //world.add(Box::<Sphere>::new(e2));

    let cam =  Camera {..Default::default() };

    for j in (0..H).rev() {
        for i in 0..W {
            
            let mut pixel_color = v3!(0,0,0);
            for _ in 0..S {
                let mut rng = rand::thread_rng();
                let normal = Uniform::from(0.0..1.0);

                let u = (i as f64 + normal.sample(&mut rng) as f64)/(W-1) as f64;
                let v = (j as f64 + normal.sample(&mut rng) as f64)/(H-1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += r.color(&mut world,MD);


            }
                pixel_color
                .write_color(S);
        }  
    }
    

}


