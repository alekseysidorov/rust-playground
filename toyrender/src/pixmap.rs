use std::ops::{Index, IndexMut};

use std::vec::Vec;

//TODO error handling
#[derive(Default)]
pub struct Pixmap {
    w: usize,
    h: usize,
    
    data: Vec<i32> 
}

impl Pixmap {
    pub fn new(w: usize, h: usize, fill_value: i32) -> Pixmap {
        Pixmap {
            w: w,
            h: h,
            data: vec![fill_value; w*h]
        }
    }
    
    pub fn fill(&mut self, fill_value: i32) {
        self.data = vec![fill_value; self.w*self.h]
    } 
    
    pub fn width(&self) -> usize { self.w }
    pub fn height(&self) -> usize { self.h }

    pub fn get(&self, x: i32, y: i32) -> i32 {
        if x < 0 || y < 0 {
            return 0;
        }
        if x >= self.w as i32 || y >= self.h as i32 {
            return 0;
        }

        self[x as usize][y as usize]
    }
}

impl Index<usize> for Pixmap {
    type Output = [i32];

    #[inline]
    fn index<'a>(&'a self, _index: usize) -> &'a Self::Output {
        let i = _index*self.w;
        
        &self.data[i .. i + self.w]
    }
}

impl IndexMut<usize> for Pixmap {
    #[inline]
    fn index_mut<'a>(&'a mut self, _index: usize) -> &'a mut Self::Output {
        let i = _index*self.w;
        
        &mut self.data[i .. i + self.w]
    }    
}

#[test]
fn test_slice() {
    let w = 100; let h = 100;

    let mut p = Pixmap::new(w, h, 0);

    p[10][10] = 10;
    
    assert!(p[10][10] == p.data[10 * w + 10]);
}
