extern crate sdl2;

use sdl2::rect::Point;

use std::clone::Clone;
use std::marker::Copy;
use std::ops::{ Sub, Index, Mul, Add, IndexMut };

use num;
use num::traits::{ Num, Zero, One };

use vector3d::{ Vector3D, Vec3f, Vec3i };

pub struct LineRasterizer {
    b: Point,
    
    x: i32,
    y: i32,
    
    dx: i32,    
    dy: i32,
    
    sx: i32,
    sy: i32,
    
    error: i32,
    delta_error: i32,
}

impl LineRasterizer {
    pub fn new(a: Point, b: Point) -> LineRasterizer {    
        let mut line = LineRasterizer {
            b: b,
            x: a.x(), y: a.y(),
            
            dx: (b.x() - a.x()).abs(),
            dy: (b.y() - a.y()).abs(),
            
            error: 0, delta_error: 0,
            
            sx: if b.x() - a.x() > 0 { 1 } else { -1 },
            sy: if b.y() - a.y() > 0 { 1 } else { -1 },
        };      
    
        if line.dx > line.dy {
            line.delta_error = line.dy;
        } else {
            line.delta_error = line.dx;
        }   
        return line;
    }

    pub fn next(&mut self) -> bool {
        if self.x == self.b.x() && self.y == self.b.y() {
            return false
        }
        self.next_point();        
        return true
    }
    
    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }
    pub fn point(&self) -> Point { Point::new(self.x, self.y) }
    
    fn next_point(&mut self) {
    
        self.error += self.delta_error;
        
        if self.dx > self.dy {        
            if 2 * self.error >= self.dx {
                self.y += self.sy;
                self.error -= self.dx;
            }           
            self.x += self.sx;
        } else {            
            if 2 * self.error >= self.dy {
                self.x += self.sx;
                self.error -= self.dy;
            }           
            self.y += self.sy;       
        }
    }
} 

const Size: usize = 3;

pub struct LineRasterizer3
{
    from: Vec3i,
    to: Vec3i,
    
    step: Vec3i,
    error: Vec3i,
    delta_error: Vec3i,
    major_axis: usize,
}

impl LineRasterizer3 
{
    pub fn new(from: Vec3i, to: Vec3i) -> LineRasterizer3 {
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
    
        LineRasterizer3 {
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
