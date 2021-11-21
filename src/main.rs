

use std::ops::{Div, Add, Sub, Mul,DivAssign, AddAssign, SubAssign, MulAssign};
use std::fmt;
use std::io::{self, Write};




#[derive(Copy, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
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

#[inline(always)]
fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x*v.x + u.y*v.y + u.z*v.z
}

#[inline(always)]
fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x
    }
}





impl Vec3 {

    #[inline(always)]
    fn length_squared(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }
    #[inline(always)]
    fn length(&self) -> f64 {
        self.length_squared().powf(0.5)
    }

    fn write_color(&self) {
        if let Err(e) = writeln!(std::io::stdout(),"{} {} {}",(self.x*255.999) as i32,(self.y*255.999) as i32,(self.z*255.999) as i32) {
            println!("Writing error: {}", e.to_string()); 
        }
    }

}

#[derive(Copy, Clone)]
struct Ray {
    origin: Vec3,
    direction: Vec3
}

#[macro_export]
macro_rules! v3 {
    ( $( $x:expr ),* ) => {
        {
            Vec3 {$($x as f64,)*}
        }
    };
}




impl Ray {
    fn at(self,t: f64) -> Vec3 {
        self.origin + self.direction*t
    }

    fn color(self) -> Vec3 {
        let t = self.hit_sphere(Vec3 {x: 0.0, y: 0.0, z: -1.0}, 0.5);
        let gt = ((self.direction / self.direction.length()).y + 1.0)*0.5;
        let grad = Vec3{x: 1.0,y: 1.0,z: 1.0}*(1.0-gt) + Vec3{x: 0.5,y: 0.7,z: 1.0}*gt;
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
        let a = dot(self.direction,self.direction);
        let b = 2.0*dot(oc,self.direction);
        let c = dot(oc,oc) - radius*radius;
        let disc = b*b - 4.0*a*c;
        if disc < 0.0 {-1.0} else {(-b - disc.powf(0.5))/2.0*a }

    }
}


fn main() {
    const W : i32 = 400;
    const AR : f64 = 16.0/9.0;
    const H : i32 = (W as f64/AR) as i32;
    
    if let Err(e) = writeln!(std::io::stdout(),"P3\n{} {}\n255",W,H) {
        println!("Writing error: {}", e.to_string()); 
    }

    let vp_height = 2.0;
    let vp_width = AR*vp_height;
    let focal_length = 1.0;

    let origin = Vec3 {x: 0.0,y: 0.0, z: 0.0};
    let horiz = Vec3 {x: vp_width,y: 0.0, z: 0.0};
    let vert = Vec3 {x: 0.0,y: vp_height, z: 0.0};
    let llc = origin - horiz/2.0 - vert/2.0 - Vec3{x: 0.0,y: 0.0, z: focal_length};


    for j in (0..H).rev() {
        for i in 0..W {
            let u = i as f64/(W-1) as f64;
            let v = j as f64/(H-1) as f64;
            Ray {origin: origin, direction: llc + horiz*u + vert*v - origin}
                .color()
                .write_color();
        }  
    }


}


