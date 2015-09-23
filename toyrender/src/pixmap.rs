use std::ops::{Index, IndexMut};

use std::vec::Vec;

//TODO error handling

pub struct Pixmap {
    w: usize,
    
    data: Vec<u32> 
}

impl Pixmap {
    pub fn new(w: usize, h: usize, fill_value: u32) -> Pixmap {
        let mut pixmap = Pixmap {
            w: w,
            data: Vec::new()
        };
        
        //FIXME rewrite me
        for i in 0..w*h {
            pixmap.data.push(fill_value);
        }
        
        return pixmap;
    }    
}

impl Index<usize> for Pixmap {
    type Output = [u32];

    fn index<'a>(&'a self, _index: usize) -> &'a Self::Output {
        let i = _index*self.w;
        
        &self.data[i .. i + self.w]
    }
}

impl IndexMut<usize> for Pixmap {
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