


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
        let t = self.hit_sphere(v3!(0.0, 0.0, -1.0), 0.5);
        let gt = ((self.direction / self.direction.length()).y + 1.0)*0.5;
        let grad = v3!( 1.0, 1.0, 1.0)*(1.0-gt) + v3!(0.5,0.7,1.0)*gt;
        if t > 0.0 {
            let n = self.at(t) - Vec3 {x: 0.0, y: 0.0, z: -1.0};
            let n_bar = n/n.length();

            (n_bar+1.0)/2.0
        }
        else {
            grad
        }
    }

    fn hit_sphere(self, center: Vec3, radius: f64) -> f64 {
        //quadratic formula
        let oc = self.origin - center;
        let a = Vec3::dot(self.direction,self.direction);
        let b = 2.0*Vec3::dot(oc,self.direction);
        let c = Vec3::dot(oc,oc) - radius*radius;
        let disc = b*b - 4.0*a*c;
        if disc < 0.0 {-1.0} else {(-b - disc.powf(0.5))/2.0*a }

    }
}



