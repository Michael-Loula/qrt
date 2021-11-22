


use std::ops::{Div, Add, Sub, Mul,DivAssign, AddAssign, SubAssign, MulAssign};
use std::fmt;
use std::io::{Write};

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

    pub fn write_color(&self) {
        if let Err(e) = writeln!(std::io::stdout(),"{} {} {}",(self.x*255.999) as i32,(self.y*255.999) as i32,(self.z*255.999) as i32) {
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
            vec3::Ray {origin: $origin, direction: $direction}
        }
    };
}




impl Ray {
    pub fn at(self,t: f64) -> Vec3 {
        self.origin + self.direction*t
    }

    pub fn color(self) -> Vec3 {
        let s = Sphere {center: v3!(0.0, 0.0, -1.0), radius: 0.5};
        let hr = s.hit(self,0.0,f64::INFINITY);
        match hr {
            Some(x) => {
                (x.normal+1.0)*0.5
            }
            None => {
                let gt = ((self.direction / self.direction.length()).y + 1.0)*0.5;
                v3!( 1.0, 1.0, 1.0)*(1.0-gt) + v3!(0.5,0.7,1.0)*gt
            }
        }

    }


}

pub struct HitRecord {
    point : Vec3,
    normal : Vec3,
    t : f64
}
trait Hittable {
    fn hit(&self, r: Ray, t_0 : f64, t_1 : f64) -> Option<HitRecord> ;
}


struct Sphere {
    center: Vec3,
    radius: f64
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
            Some(HitRecord {t: root, point: x, normal: (x - self.center) / self.radius})


        }



}

}