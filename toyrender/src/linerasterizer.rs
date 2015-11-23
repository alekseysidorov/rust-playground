extern crate sdl2;

//use std::ops::{ Sub, Index, Mul, Add, IndexMut };

use num;
//use num::traits::{ Num, Zero, One };

use vector3d::{ Vec3i };

const SIZE: usize = 3;

pub struct LineRasterizer
{
    from: Vec3i,
    to: Vec3i,
    
    step: Vec3i,
    d: Vec3i,
    major_axis: usize,
}

impl LineRasterizer
{
    pub fn new(from: Vec3i, to: Vec3i) -> LineRasterizer {
        let mut n = from;
        let mut s = n;
 
        let mut axis = 0;
        let mut max = 0;
        for i in 0..SIZE {
            n[i] = 0;

            let d = to[i] - from[i];
            s[i] = if d > 0 { 1 } else { -1 };
            
            let d = num::abs(d);
            if d > max {
                max = d;
                axis = i as usize;
            };
        }
    
        LineRasterizer {
            from: from,
            to: to,
            
            step: s,
            d: n,
            major_axis: axis
        }
    }

    #[inline]
    pub fn has_next(&mut self) -> bool {
        if self.from == self.to { 
            return false;
        }
        return true;
    }

    #[inline]
    pub fn next_point(&mut self) -> bool {
    
        if !self.has_next() {
            return false;
        }
        
        let from = self.from; let to = self.to;
        let calc_rs = |axis| { num::abs(to[axis] - from[axis]) };
        
        self.from[self.major_axis] += self.step[self.major_axis];
        
        let rs_base = calc_rs(self.major_axis);
        for i in 0..SIZE {
            let rs = calc_rs(i);
            
            if rs > 0 && i != self.major_axis {
                self.d[i] += rs;
                if self.d[i] >= rs_base {
                    self.d[i] -= rs_base;
                    self.from[i] += self.step[i];
                }
            }
        }
        return true;
    }

    #[inline]
    pub fn point(&self) -> Vec3i {
        return self.from;
    }
}

impl Iterator for LineRasterizer
{
    type Item = Vec3i;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next() { 
            None
        } else {
            self.next_point();
            Some(self.point())
        }
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

#[cfg(test)]
mod tests {
    use super::LineRasterizer;
    use vector3d::{ Vec3i };
    
    #[test]
    fn test_rasterizer_simple() {

        let v1 = Vec3i::new(0, 5, 10);    
        let v2 = Vec3i::new(15, 0, 15);

        let raster = LineRasterizer::new(v1, v2);

        let p = raster.last().unwrap();
        assert_eq!(p, v2);
    }

    #[test]
    fn test_rasterizer_circle() {
        let center = Vec3i::new(450, 450, 30);
        
        let r = 300.0;
        let step = 0.01;
        
        for a in 1..((360.0/step) as i32) {
            use std::f32;
        
            let d = a as f32 * step / (2.0 * f32::consts::PI);
            let x = center.x() as f32 + r * d.cos();
            let y = center.y() as f32 + r * d.sin();
            
            let v = Vec3i::new(x as i32, y as i32, center.z());
            
            let raster = LineRasterizer::new(center, v);
            let p = raster.last().unwrap();
            assert_eq!(p, v);
        }
    }

    #[test]
    fn test_rasterizer_same() {

        let v1 = Vec3i::new(10, 15, 10);    
        let v2 = Vec3i::new(10, 15, 10);

        let raster = LineRasterizer::new(v1, v2);
        
        let p = raster.last();
        assert!(p.is_none());
    }
}
