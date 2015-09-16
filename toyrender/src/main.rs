#[macro_use]
extern crate log;
extern crate env_logger;

use log::LogLevel;

extern crate sdl2;

use std::default::Default;

use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;

struct SdlCanvas
{
    renderer: Renderer<'static>
}

struct Vector3D {
    x: f32,
    y: f32,
    z: f32,
}

struct LineRasterizer {
    a: Point,
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
            a: a, b: b,
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
        if (self.x == self.b.x() && self.y == self.b.y()) {
            return false
        }
        self.next_point();        
        return true
    }
    
    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }
    
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

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3D {
        Vector3D {
            x: x, y:y, z:z
        }
    }
}

#[derive(Default)]
struct Model {
    pub verticies: Vec<Vector3D>,
    pub faces: Vec<[i32; 3]>
}

impl Model {
    pub fn new() -> Model {
        Model {
            ..Default::default()
        }
    }

    pub fn load_from_file(file_path: &str) -> Model {
        let path = Path::new(file_path);
        let file = BufReader::new(File::open(&path).unwrap());

        let mut model = Model::new();

        for line in file.lines() {
            let line = line.unwrap();

            let words: Vec<&str>;
            words = line.split_whitespace().collect();
            if line.starts_with("v ") {
                
                let mut points : [f32; 3] = [ 0.0, 0.0, 0.0 ];
                for i in 0..3 {
                    points[i] = words[i+1].parse::<f32>().unwrap();
                }                
                let v = Vector3D::new(points[0], points[1], points[2]);
                model.verticies.push(v);
            } else if line.starts_with("f ") {
                let mut face = [-1, -1, -1];

                for i in 0..3 {
                    let mut words = words[i+1].split("/");
                    face[i] = words.next().unwrap().parse::<i32>().unwrap();
                    face[i] -= 1;
                }
                model.faces.push(face);

            }
        }

        return model;
    }
}

impl SdlCanvas {
    pub fn new(renderer: Renderer<'static>) -> SdlCanvas {
        SdlCanvas { renderer: renderer }
    }
    
    pub fn present(&mut self)
    {
        self.renderer.present()
    }
    
    pub fn line(&mut self, a: Point, b: Point, color: u32)
    {
        let mut raster = LineRasterizer::new(a, b);
        
        while raster.next() {
            self.set_pixel(raster.x(), raster.y(), color);
        }
    }
    
    pub fn triangle(&mut self, mut a: Point, mut b: Point, mut c: Point, color: u32)
    {
        if b.y() > a.y() { std::mem::swap(&mut a, &mut b); }
        if c.y() > a.y() { std::mem::swap(&mut a, &mut c); }
        if c.y() > b.y() { std::mem::swap(&mut c, &mut b); }        
    
        self.line(a, b, color);
        self.line(b, c, color);
        self.line(c, a, color);
        
        // Fill top triangle part
        let mut raster1 = LineRasterizer::new(a, b);
        let mut raster2 = LineRasterizer::new(a, c);
        
        while raster1.next() {
            raster2.next();
            
            let a = Point::new(raster1.x(), raster1.y());
            let b = Point::new(raster2.x(), raster2.y());
            
            self.line(a, b, color);
        }
        
        // Fill bottom triangle part
        let mut raster1 = LineRasterizer::new(b, c);
        while raster2.next() {
            raster1.next();
            
            let a = Point::new(raster1.x(), raster1.y());
            let b = Point::new(raster2.x(), raster2.y());
            
            self.line(a, b, color);
        }
    }
    
    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        self.renderer.set_draw_color(Color::RGB((color >> (8*2)) as u8, (color >> (8*1)) as u8, color as u8));
        self.renderer.draw_point(Point::new(x, y));
    }
}

pub fn main() {
    env_logger::init().unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let w = 900;
    let h = 900;

    let window = video_subsystem.window("rust-sdl2 demo: Video", w, h)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let renderer = window.renderer().build().unwrap();
    
    let mut canvas = SdlCanvas::new(renderer);
    
    canvas.triangle(Point::new(100, 50), Point::new(150, 60), Point::new(120, 110), 0xFF);
    
    canvas.triangle(Point::new(150, 250), Point::new(350, 70), Point::new(120, 610), 0xFF00FF);

//     let model = Model::load_from_file("obj/african_head.obj");
//     for face in model.faces {
// 
//         for i in 0..3 {
//             let v0 = &model.verticies[face[i] as usize];
//             let v1 = &model.verticies[face[(i+1)%3] as usize];
// 
//             let conv = |v : &Vector3D, w, h| {
//                 let x = (v.x + 1.0) * w as f32 / 2.0;
//                 let y = (v.y + 1.0) * h as f32 / 2.0;
// 
//                 (w as i32 - x as i32, h as i32 - y as i32)
//             };
// 
//             let (x0, y0) = conv(&v0, w, h);
//             let (x1, y1) = conv(&v1, w, h);
// 
//             canvas.line(Point::new(x0, y0), Point::new(x1, y1), 0xFFFFFF);
//         }
//     }
// 
    canvas.present();
    
    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();

    while running {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }
}
