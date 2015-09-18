extern crate num;

use std::ops::BitXor;
use std::ops::Sub;
use std::ops::Index;
use std::ops::Mul;

use num::pow;

#[derive(Copy, Clone)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3D {
        Vector3D {
            x: x, y:y, z:z
        }
    }
    
    pub fn normalize(&mut self) {
        let l = (num::pow(self.x, 2) + num::pow(self.y, 2) + num::pow(self.z, 2)).sqrt();
        
        self.x /= l;
        self.y /= l;
        self.z /= l;
    }
    
    pub fn cross(&self, other: Vector3D) -> Vector3D {
        Vector3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }
    
    pub fn dot(&self, other: Vector3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl BitXor for Vector3D {
    type Output = Vector3D;

    fn bitxor(self, _rhs: Vector3D) -> Vector3D {       
        self.cross(_rhs)
    }
}

impl Mul for Vector3D {
    type Output = f32;

    fn mul(self, _rhs: Vector3D) -> f32 {       
        self.dot(_rhs)
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, _rhs: Vector3D) -> Vector3D {
        Vector3D::new(
            self.x - _rhs.x,
            self.y - _rhs.y,
            self.z - _rhs.z,
        )
    }
}

impl Index<usize> for Vector3D {
    type Output = f32;

    fn index<'a>(&'a self, _index: usize) -> &'a Self::Output {
        match _index {
            0   => &self.x,
            1   => &self.y,
            2   => &self.z,
            _   => panic!("Oo"),
        }
    }
}
