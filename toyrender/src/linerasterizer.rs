extern crate sdl2;

//use std::ops::{ Sub, Index, Mul, Add, IndexMut };

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
        for i in 0..Size {
            let residual_steps = num::abs(self.to[i] - self.from[i]);
            self.delta_error[i] += residual_steps;
            if i == self.major_axis || self.delta_error[i] > residual_steps {
                self.from[i] += self.step[i];
            }
            self.delta_error[i] -= (residual_steps + 1)
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
