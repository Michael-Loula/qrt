


use std::ops::{Div, Add, Sub, Mul,DivAssign, AddAssign, SubAssign, MulAssign};
use std::fmt;
use std::io::{Write};
use rand::distributions::{Distribution, Uniform};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
            z: self.z / _rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    
    fn div(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    
    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;
    
    fn add(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x + _rhs,
            y: self.y + _rhs,
            z: self.z + _rhs,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;
    
    fn sub(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x + _rhs,
            y: self.y + _rhs,
            z: self.z + _rhs,
        }
    }
}

//here

impl AddAssign for Vec3 {

    fn add_assign(&mut self, _rhs: Vec3) {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self.z += _rhs.z;
    }
}

impl SubAssign for Vec3 {

    fn sub_assign(&mut self, _rhs: Vec3) {
            self.x -= _rhs.x;
            self.y -= _rhs.y;
            self.z -= _rhs.z;
    }
    
}

impl MulAssign for Vec3 {

    fn mul_assign(&mut self, _rhs: Vec3) {
        self.x *= _rhs.x;
        self.y *= _rhs.y;
        self.z *= _rhs.z;
    }
}

impl DivAssign for Vec3 {

    fn div_assign(&mut self, _rhs: Vec3) {
        self.x /= _rhs.x;
        self.y /= _rhs.y;
        self.z /= _rhs.z;
    }
}

impl AddAssign<f64> for Vec3 {

    fn add_assign(&mut self, _rhs: f64) {
        self.x += _rhs;
        self.y += _rhs;
        self.z += _rhs;
    }
}

impl SubAssign<f64> for Vec3 {

    fn sub_assign(&mut self, _rhs: f64) {
            self.x -= _rhs;
            self.y -= _rhs;
            self.z -= _rhs;
    }
    
}

impl MulAssign<f64> for Vec3 {

    fn mul_assign(&mut self, _rhs: f64) {
        self.x *= _rhs;
        self.y *= _rhs;
        self.z *= _rhs;
    }
}

impl DivAssign<f64> for Vec3 {

    fn div_assign(&mut self, _rhs: f64) {
        self.x /= _rhs;
        self.y /= _rhs;
        self.z /= _rhs;
    }
}

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vec3")
         .field("x", &self.x)
         .field("y", &self.y)
         .field("y", &self.z)
         .finish()
    }
}

impl Vec3 {

    #[inline(always)]
    pub fn length_squared(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }
    #[inline(always)]
    pub fn length(&self) -> f64 {
        self.length_squared().powf(0.5)
    }

    pub fn write_color(&self, samples_per_pixel : i32) {
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;
        let scale = 1.0 / samples_per_pixel as f64;
        r *= scale;
        g *= scale;
        b *= scale;
        
        if let Err(e) = writeln!(std::io::stdout(),"{} {} {}",(256.0 * clamp(r, 0.0, 0.999)) as i32,(256.0 * clamp(g, 0.0, 0.999)) as i32,(256.0 * clamp(b, 0.0, 0.999)) as i32) {
            println!("Writing error: {}", e.to_string()); 
        }
    }

    #[inline(always)]
    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        u.x*v.x + u.y*v.y + u.z*v.z
    }

    #[inline(always)]
    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x
        }
    }

}



//macro for easy construction of ray using int or decimal values
#[macro_export]
macro_rules! v3 {
    ($x:expr,$y:expr,$z:expr) => {
        {
            Vec3 {x: $x as f64, y: $y as f64, z: $z as f64}
        }
    };
}

//macro for easy construction of ray
#[macro_export]
macro_rules! r {
    ($origin:expr,$direction:expr) => {
        {
            Ray {origin: $origin, direction: $direction}
        }
    };
}


fn rv3(x: f64, y: f64) -> Vec3 {
    let mut t = rand::thread_rng();
    let mut t2 = rand::thread_rng();
    let mut t3 = rand::thread_rng();
    Vec3 {x: Uniform::from(x..y).sample(&mut t), y: Uniform::from(x..y).sample(&mut t2), z: Uniform::from(x..y).sample(&mut t3)}
}

fn rus() -> Vec3 {
            let mut r = rv3(-1.0,1.0);
            loop {
                r = rv3(-1.0,1.0);
                if r.length_squared() > 1.0 {continue;} else {break;}
            }
            r
}




impl Ray {
    pub fn at(self,t: f64) -> Vec3 {
        self.origin + self.direction*t
    }

    pub fn color(self, world : &mut HittableList, depth : i32) -> Vec3 {
        //let s = Sphere {center: v3!(0.0, 0.0, -1.0), radius: 0.5};
        let hr = world.hit(self,0.0,f64::INFINITY);
        if depth <= 0 {return v3!(0,0,0)} else {}
        match hr {
            Some(x) => {
                
                r!(x.point,(x.point + x.normal + rus() - x.point)).color(world,depth-1)*0.5
                //(x.normal+1.0)*0.5
            }
            None => {
                let gt = ((self.direction / self.direction.length()).y + 1.0)*0.5;
                v3!( 1.0, 1.0, 1.0)*(1.0-gt) + v3!(0.5,0.7,1.0)*gt
            }
        }

    }


}

#[derive(Copy, Clone)]
pub struct HitRecord {
    point : Vec3,
    normal : Vec3,
    t : f64,
    front_face : bool

    
}

impl HitRecord {
    fn gen_face_normal(r: Ray, outward_normal : Vec3) -> (bool,Vec3) {
        let ff = Vec3::dot(r.direction,outward_normal) < 0.0;
        (ff,if ff {outward_normal} else {outward_normal*-1.0})
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_0 : f64, t_1 : f64) -> Option<HitRecord> ;
}


pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_0 : f64, t_1 : f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(oc,r.direction);
        let c = oc.length_squared() - self.radius.powf(2.0);
        let disc = half_b.powf(2.0) - a*c;
        let sqrtd = disc.sqrt();
        let root = (-half_b - sqrtd) / a;
        if root.is_nan() || root < t_0 || t_1 < root {

            None
        } 
        else {
            
            let x = r.at(root);
            let on = (x - self.center) / self.radius;
            let (ff, norm) = HitRecord::gen_face_normal(r, on);
            Some(HitRecord {t: root, point: x, normal: norm, front_face : ff})


        }



}

}

pub struct HittableList {
    pub v:  Vec<Box<dyn Hittable>>,
}



impl HittableList {
    pub fn clear(&mut self) -> () {self.v.clear();}

    pub fn add(&mut self, item: Box<dyn Hittable>) -> () {
        self.v.push(item);
    }

}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_0 : f64, t_1 : f64) -> Option<HitRecord> {
        let mut hr : HitRecord = HitRecord {point: v3!(0,0,0), normal: v3!(0,0,0), t: 0.0, front_face : false};
        let mut closest_so_far : f64 = t_1;
        let mut hit_anything : bool = false;
        for obj in self.v.iter() {
            let test = obj.hit(r,t_0,closest_so_far);
            match test {
                Some(val) => {
                    hit_anything = true;
                    closest_so_far = val.t;
                    hr = HitRecord {point : val.point, normal : val.normal,t: val.t,front_face : val.front_face};
                }
                None => {}
            }
        }
        if hit_anything 
        {Some(hr)} else {None}
    }
}


pub struct Camera {
    pub origin : Vec3,
    pub llc : Vec3,
    pub horiz : Vec3,
    pub vert : Vec3
}

impl Default for Camera {
    fn default() -> Camera {
        let ar = 16.0 / 9.0;
        let vh = 2.0;
        let vw = ar*vh;
        let focal_length = 1.0;
        let origin = v3!(0,0,0);
        let horiz = v3!(vw,0,0);
        let vert = v3!(0,vh,0);
        Camera {    origin : origin,
            llc : origin - horiz/2.0 - vert/2.0 - v3!(0,0,focal_length),
            horiz : horiz,
            vert : vert}
    }
}

impl Camera {
    pub fn get_ray(&self, u : f64, v : f64) -> Ray {
        Ray {origin : self.origin, direction: self.llc + self.horiz*u + self.vert*v - self.origin}
    }
}

#[inline(always)]
    pub fn clamp(u: f64, min: f64, max : f64) -> f64 {
        if u < min {min}
        else if u > max {max}
        else {u}
    }