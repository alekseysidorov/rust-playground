extern crate sdl2;

use sdl2::rect::Point;

use std::clone::Clone;
use std::marker::Copy;
use std::ops::{ Sub, Index, Mul, Add, IndexMut };

use num;
use num::traits::{ Num, Zero, One };

use vector3d::{ Vector3D, Vec3f, Vec3i };

const Size: usize = 3;

pub struct LineRasterizer
{
    from: Vec3i,
    to: Vec3i,
    
    step: Vec3i,
    error: Vec3i,
    delta_error: Vec3i,
    major_axis: usize,
}

impl LineRasterizer
{
    pub fn new(from: Vec3i, to: Vec3i) -> LineRasterizer {
        let mut n = from;
        let mut s = n;
 
        let mut axis = 0;
        let mut max = 0;
        for i in 0..Size {
            n[i] = 0;

            let d = to[i] - from[i];
            s[i] = if d > 0 { 1 } else { -1 };
            
            if num::abs(d) > max {
                max = d;
                axis = i as usize;
            };
        }
    
        LineRasterizer {
            from: from,
            to: to,
            
            step: s,
            error: n,
            delta_error: n,
            major_axis: axis
        }
    }

    pub fn has_next(&mut self) -> bool {
        if self.from == self.to { 
            return false;
        }
        return true;
    }

    pub fn next(&mut self) -> bool {
        if !self.has_next() {
            return false;
        }
        
        let mut has_pending_step = [false; Size];
    
        let residual_steps_base = num::abs(self.to[self.major_axis] - self.from[self.major_axis]);
        println!("rs_base: {}", residual_steps_base);
        for i in 0..Size {
            let residual_steps = num::abs(self.to[i] - self.from[i]);
            self.delta_error[i] += residual_steps;
            println!("\t{}: d: {}, rs: {}", i, self.delta_error[i], residual_steps);
            if i == self.major_axis || self.delta_error[i] > residual_steps_base {
                has_pending_step[i] = true;

            }
        }
        
        for i in 0..Size {
            if has_pending_step[i] {
                self.from[i] += self.step[i];
                self.delta_error[i] -= (residual_steps_base + 1)
            }
        }
        return self.has_next();
    }

    pub fn point(&self) -> Vec3i {
        return self.from;
    }
}


// #[derive(Copy, Clone, Debug)]
// pub struct LineRasterizer2<T> where
//     T: Index<usize> + IndexMut<usize> + Copy + Clone
// {
//     from: T,
//     to: T,
// 
//     step: T,
//     error: T,
//     delta_error: T
// }
// 
// impl<T> LineRasterizer2<T>  where
//     T: Index<usize> + IndexMut<usize> + Copy + Clone
// {
//     type Output = <T as Index<usize>>::Output;
// 
//     pub fn new(from: T, to: T) -> LineRasterizer2<T> {
//         let n = from;
//         let s = n;
//         
//         for i in 0..Size {
//             let v = n[i] + n[i];
//             //s[i] = if to[i] - from[i] > 0 { 1 } else { -1 };
//         }
//     
//         LineRasterizer2 {
//             from: from,
//             to: to,
//             
//             step: s,
//             error: n,
//             delta_error: n,
//         }
//     }
// }

// #[test]
// fn test_rasterizer_simple() {
// 
//     let v1 = Vec3i::new(0, 5, 10);    
//     let v2 = Vec3i::new(15, 0, 15);
// 
//     let mut raster = LineRasterizer::new(v1, v2);
//     
//     while raster.next() {
//         let p = raster.point();
//     }
//     assert_eq!(raster.point(), v2);
// }
// 
// #[test]
// fn test_rasterizer_single() {
// 
//     let v1 = Vec3i::new(0, 0, 0);    
//     let v2 = Vec3i::new(25, 0, 0);
// 
//     let mut raster = LineRasterizer::new(v1, v2);
//     
//     while raster.next() {
//         let p = raster.point();
//     }
//     assert_eq!(raster.point(), v2);
// }

#[test]
fn test_rasterizer_diag() {

    let v1 = Vec3i::new(0, 0, 0);    
    let v2 = Vec3i::new(150, 150, 0);

    let mut raster = LineRasterizer::new(v1, v2);
    
    for i in 0..152 {
        raster.next();
    //while raster.next() {
        let p = raster.point();
        println!("Point now {:?}", p);
    }
    assert_eq!(raster.point(), v2);
}

// #[test]
// fn test_rasterizer_same() {
// 
//     let v1 = Vec3i::new(10, 15, 10);    
//     let v2 = Vec3i::new(10, 15, 10);
// 
//     let mut raster = LineRasterizer::new(v1, v2);
//     
//     while raster.next() {
//     }
//     assert_eq!(raster.point(), v2);
// }

