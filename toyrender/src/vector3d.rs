extern crate num;
extern crate log;

use std::ops::BitXor;
use std::ops::{ Sub, Index, IndexMut, Mul, Add };
use num::traits::NumCast;

use num::pow;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Vector3D<T> {   
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vec3i = Vector3D<i32>;
pub type Vec3f = Vector3D<f32>;

impl<T> Vector3D<T> 
    where T: Copy + Clone 
{
    pub fn new(x: T, y: T, z: T) -> Vector3D<T> {
        Vector3D {
            x: x, y:y, z:z
        }
    }
    
    #[inline]
    pub fn x(&self) -> T { self.x }
    #[inline]
    pub fn y(&self) -> T { self.y }
    #[inline]
    pub fn z(&self) -> T { self.z }    
}

impl Vector3D<f32> {

    pub fn dot(&self, other: Vector3D<f32>) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    pub fn norm(self) -> f32 {
        return (num::pow(self.x, 2) + num::pow(self.z, 2) + num::pow(self.y, 2)).sqrt();
    }

    pub fn normalized(self) -> Vector3D<f32> { 
        self * (1.0 / self.norm())
    }
    
    pub fn zero() -> Vector3D<f32> {
        Vector3D::new(0.0, 0.0, 0.0)
    }
}

impl Vector3D<i32> {
    pub fn zero() -> Vector3D<i32> {
        Vector3D::new(0, 0, 0)
    }
}

impl<T: Add<Output = T>> Add for Vector3D<T> {
    type Output = Vector3D<T>;
    
    fn add(self, other: Vector3D<T>) -> Self::Output {
        Vector3D {
            x: self.x + other.x, 
            y: self.y + other.y, 
            z: self.z + other.z
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vector3D<T> {
    type Output = Vector3D<T>;
    
    fn sub(self, other: Vector3D<T>) -> Self::Output {
        Vector3D {
            x: self.x - other.x, 
            y: self.y - other.y, 
            z: self.z - other.z
        }
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Copy> BitXor for Vector3D<T> {
    type Output = Vector3D<T>;

    fn bitxor(self, other: Vector3D<T>) -> Self::Output {       
        Vector3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }
}

impl<T: Mul<Output = T> + Add<Output = T>> Mul for Vector3D<T> {
    type Output = T;
    fn mul(self, other: Vector3D<T>) -> Self::Output {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector3D<T> {
    type Output = Vector3D<T>;
    
    fn mul(self, other: T) -> Self::Output {
        Vector3D { 
            x: self.x * other, 
            y: self.y * other, 
            z: self.z * other
        }
    }
}

impl<T> Index<usize> for Vector3D<T> {
    type Output = T;

    fn index<'a>(&'a self, _index: usize) -> &'a Self::Output {
        match _index {
            0   => &self.x,
            1   => &self.y,
            2   => &self.z,
            _   => panic!("Oo"),
        }
    }
}

impl<T> IndexMut<usize> for Vector3D<T> {
    fn index_mut<'a>(&'a mut self, _index: usize) -> &'a mut Self::Output {
        match _index {
            0   => &mut self.x,
            1   => &mut self.y,
            2   => &mut self.z,
            _   => panic!("Oo"),
        }
    }
}

impl<T: NumCast> Vector3D<T> {
    pub fn to<V: NumCast>(self) -> Vector3D<V> {
        Vector3D {
            x: NumCast::from(self.x).unwrap(),
            y: NumCast::from(self.y).unwrap(),
            z: NumCast::from(self.z).unwrap(),
        }
    }
}

#[test]
fn test_normalized() {
    const EPS: f32 = 0.001;

    let v1 = Vec3f::new(2.0, 2.0, 0.0);    
    let v2 = Vec3f::new(1.0, 1.0, 1.0);
    
    assert!(v1.normalized().norm() - 1.0 < EPS);
    assert!(v2.normalized().norm() - 1.0 < EPS);
}

#[test]
fn test_numcast() {
    let v1 = Vec3f::new(2.0, 2.0, 0.0);
    let v3 = Vec3i::new(2, 2, 0);

    let v2 = v1.to::<i32>();
    assert_eq!(v2, v3);
}
