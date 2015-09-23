extern crate sdl2;

use sdl2::rect::Point;

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
