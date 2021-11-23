

use std::rc::Rc;
use std::ops::{Div, Add, Sub, Mul,DivAssign, AddAssign, SubAssign, MulAssign};
use std::fmt;
use std::cmp;
use std::io::{Write};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

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
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    #[inline(always)]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    pub fn normalize(self) -> Vec3 {
        self / self.length_squared().sqrt()
    }

    pub fn is_close_to_zero(&self) -> bool {
        const eps : f64 = 1e-8;
        (self.x.abs() < eps) && (self.y.abs() < eps) && (self.z.abs() < eps)
    }

    pub fn write_color(&self, samples_per_pixel : i32) {
        let mut r = self.x;
        let mut g = self.y;
        let mut b = self.z;
        let scale = 1.0 / samples_per_pixel as f64;
        r = (r*scale).sqrt();
        g = (g*scale).sqrt();
        b = (b*scale).sqrt();
        
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
    Vec3 {x: t.gen_range(x..y), y: t.gen_range(x..y), z: t.gen_range(x..y)}
}

fn random_in_unit() -> Vec3 {
            let mut r;
            loop {
                r = rv3(-1.0,1.0);
                if r.length_squared() > 1.0 {continue;} else {break;}
            }
            r
}

fn random_in_hemisphere(normal : Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit();
    if Vec3::dot(in_unit_sphere, normal) > 0.0
    {in_unit_sphere}
    else
    {in_unit_sphere*-1.0}
}




impl Ray {
    pub fn at(self,t: f64) -> Vec3 {
        self.origin + self.direction*t
    }

    pub fn color(self, world : &mut HittableList, depth : i32, u : i32, v : i32) -> Vec3 {
        let hr = world.hit(self,0.001,f64::INFINITY);
        if depth <= 0 {return v3!(0,0,0)} else {}
        match hr {
            Some(ref x) => {
                            let (att, scat, hit) = x.mat_ptr.scatter(self,x);
                            if hit {scat.color(world,depth-1,u,v)*att}
                            else {
                                v3!(0,0,0)
                            }
                        }
                    
            None => {
              let gt = ((self.direction / self.direction.length()).y + 1.0)*0.5;
                v3!( 1.0, 1.0, 1.0)*(1.0-gt) + v3!(0.5,0.7,1.0)*gt
            }
        }

    }


}

#[derive(Clone)]
pub struct HitRecord {
    point : Vec3,
    normal : Vec3,
    t : f64,
    front_face : bool,
    mat_ptr : Rc<dyn Material>

    
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
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_0 : f64, t_1 : f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(oc,r.direction);
        let c = oc.length_squared() - self.radius.powf(2.0);
        let disc = half_b.powf(2.0) - a*c;
        let sqrtd = disc.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root.is_nan() || root < t_0 || t_1 < root {
            root = (-half_b + sqrtd) / a;
            if root.is_nan() || root < t_0 || t_1 < root { None } else { 
                let x = r.at(root);
                let on = (x - self.center) / self.radius;
                let (ff, norm) = HitRecord::gen_face_normal(r, on);
                Some(HitRecord {t: root, point: x, normal: norm, front_face : ff, mat_ptr : self.mat_ptr.clone()})
            }
        } 
        else {
            let x = r.at(root);
            let on = (x - self.center) / self.radius;
            let (ff, norm) = HitRecord::gen_face_normal(r, on);
            Some(HitRecord {t: root, point: x, normal: norm, front_face : ff, mat_ptr : self.mat_ptr.clone()})


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
        let mut hr : HitRecord = HitRecord {point: v3!(0,0,0), normal: v3!(0,0,0), t: 0.0, front_face : false, mat_ptr : Rc::new(Dielectric {ir: 1.5})};
        let mut closest_so_far : f64 = t_1;
        let mut hit_anything : bool = false;
        for obj in self.v.iter() {
            let test = obj.hit(r,t_0,closest_so_far);
            match test {
                Some(val) => {

                    hit_anything = true;
                    closest_so_far = val.t;
                    hr = val;
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

pub trait Material {
    fn scatter(&self, r: Ray, hr: &HitRecord) -> (Vec3,Ray,bool);
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo : Vec3
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo : Vec3,
    pub fuzz : f64
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub ir : f64,
}
 
impl Material for Lambertian {
    fn scatter(&self, r: Ray, hr: &HitRecord) -> (Vec3,Ray,bool) {
        let rand = random_in_unit();
        let mut scatter_dir = hr.normal + rand/rand.length();
        if scatter_dir.is_close_to_zero() {scatter_dir = hr.normal;} else {}
        (self.albedo,r!(hr.point, scatter_dir),true)
    }
}

impl Material for Metal {
    fn scatter(&self, r: Ray, hr: &HitRecord) -> (Vec3,Ray,bool) {
        let refl = reflect(r.direction/r.direction.length(),hr.normal);
        let scatter = r!(hr.point, refl + random_in_unit()*self.fuzz);
        (self.albedo,scatter,Vec3::dot(scatter.direction, hr.normal) > 0.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: Ray, hr: &HitRecord) -> (Vec3,Ray,bool) {
        
        let ref_ratio = if hr.front_face {1.0/self.ir} else {self.ir};
        let unit_dir = r.direction.normalize();
        let neg_ud = unit_dir*-1.0;
        let cos = Vec3::dot(neg_ud,hr.normal).min(1.0);
        let sin = (1.0 - cos.powi(2)).sqrt();
        let mut rng = rand::thread_rng();
        let normal = Uniform::from(0.0..1.0);
        let cant = (ref_ratio * sin) > 1.0;
        let refl = reflectance(cos, ref_ratio) > normal.sample(&mut rng);
        let direction = if  cant || refl
        {
            reflect(unit_dir, hr.normal)
        } 
        else 
        {
            refract(unit_dir, hr.normal, ref_ratio)
        };
    
        (v3!(1,1,1),r!(hr.point, direction),true)
    }
    
}

fn reflect(v : Vec3, n : Vec3) -> Vec3 {
    return v - n*Vec3::dot(v,n)*2.0;
}

fn refract(uv : Vec3, n : Vec3,etai_over_etat : f64) -> Vec3 {
    let negative_uv = uv*-1.0;
    let cos = Vec3::dot(negative_uv,n).min(1.0);
    let r_out_per = (uv + n*cos)*etai_over_etat;
    let r_out_par = n*(1.0 - r_out_per.length_squared()).abs().sqrt()*-1.0;
    r_out_par + r_out_per
}


fn reflectance(cosine : f64, ref_idx : f64) -> f64 {
    let r0 = ((1.0-ref_idx) / (1.0+ref_idx)).powi(2);
    r0 + (1.0-r0) * (1.0 - cosine).powi(5)
}




