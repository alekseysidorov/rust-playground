#[macro_use]
extern crate log;
extern crate env_logger;
extern crate sdl2;
extern crate num;

extern crate toyrender;

use std::default::Default;

use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;

use toyrender::vector3d::Vector3D;
use toyrender::linerasterizer::LineRasterizer;
use toyrender::pixmap::Pixmap;

struct SdlCanvas
{
    renderer: Renderer<'static>
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

        let mut fill_fn = |raster1 : &mut LineRasterizer, raster2: &mut LineRasterizer| {
            let mut y = raster1.y();

            while raster1.next() { 
                if y != raster1.y() {
                    y = raster1.y();

                    while raster2.y() != y {
                        raster2.next();
                    }          
              
                    self.line(raster1.point(), raster2.point(), color);
                }
            }
        };
        
        // Fill top triangle part
        let mut raster1 = LineRasterizer::new(a, b);
        let mut raster2 = LineRasterizer::new(a, c);
        fill_fn(&mut raster1, &mut raster2);
        
        // Fill bottom triangle part
        raster1 = LineRasterizer::new(b, c);
        fill_fn(&mut raster1, &mut raster2);
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
    
    let light_dir = Vector3D::new(0.0, 0.0, -1.0);

    let model = Model::load_from_file("obj/african_head.obj");
    for face in model.faces {        
        let mut screen_coords = [Point::new(0,0); 3];
        let mut world_coords = [Vector3D::new(0.0,0.0,0.0); 3];
    
        for i in 0..3 {
            let world = model.verticies[face[i] as usize];
            
            screen_coords[i] = Point::new(
                ((world.x + 1.0) * w as f32 / 2.0) as i32, 
                h as i32 - ((world.y + 1.0) * h as f32 / 2.0) as i32, 
            );
            world_coords[i] = world;
        }
         
        let mut n: Vector3D = (world_coords[2]-world_coords[0]) ^ (world_coords[1]-world_coords[0]);
        n.normalize();
        
        let intensity = light_dir * n;

        if intensity > 0.0 {
            let l = (255.0 * intensity) as u32;
            let color = l | l << 8 | l << 16;

            canvas.triangle(
                screen_coords[0],
                screen_coords[1],
                screen_coords[2],
                color,
            );
        }
    }

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
