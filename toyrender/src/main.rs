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

use toyrender::vector3d::{ Vec3f, Vec3i };
use toyrender::linerasterizer::LineRasterizer;
use toyrender::pixmap::Pixmap;

struct SdlCanvas
{
    renderer: Renderer<'static>,
    
    z_buffer: Pixmap
}



#[derive(Default)]
struct Model {
    pub verticies: Vec<Vec3f>,
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
                let v = Vec3f::new(points[0], points[1], points[2]);
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
    pub fn new(renderer: Renderer<'static>, w: usize, h: usize) -> SdlCanvas {
        SdlCanvas { 
            renderer: renderer, 
            z_buffer: Pixmap::new(w + 1, h + 1, std::i32::MIN) 
        }
    }
    
    pub fn present(&mut self)
    {
        self.renderer.present()
    }
    
    pub fn line(&mut self, a: Vec3i, b: Vec3i, color: u32)
    {
        let mut raster = LineRasterizer::new(a, b);
        
        self.set_pixel(a, color);
        while raster.next() {
            let p = raster.point();
            
            self.set_pixel(p, color);
        }
        self.set_pixel(b, color); //HACK
    }
    
    pub fn triangle(&mut self, mut a: Vec3i, mut b: Vec3i, mut c: Vec3i, color: u32)
    {
        if b.y() > a.y() { std::mem::swap(&mut a, &mut b); }
        if c.y() > a.y() { std::mem::swap(&mut a, &mut c); }
        if c.y() > b.y() { std::mem::swap(&mut c, &mut b); }       
        
        self.line(a, b, color);
        self.line(b, c, color);
        self.line(c, a, color);

        let mut fill_fn = |raster1 : &mut LineRasterizer, raster2: &mut LineRasterizer| {
            let mut y = raster1.point().y();

            while raster1.next() { 
                if y != raster1.point().y() {
                    y = raster1.point().y();

                    while raster2.point().y() != y {
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
    
    pub fn set_pixel(&mut self, v: Vec3i, color: u32) {
        let x = v.x() as usize; let y = v.y() as usize;
        
        if self.z_buffer[x][y] < v.z() {
            self.z_buffer[x][y] = v.z();
        
            self.renderer.set_draw_color(Color::RGB((color >> (8*2)) as u8, (color >> (8*1)) as u8, color as u8));
            self.renderer.draw_point(Point::new(v.x(), v.y()));
        }
    }
}

pub fn main() {
    env_logger::init().unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let w = 900;
    let h = 900;
    let d = 255;

    let window = video_subsystem.window("rust-sdl2 demo: Video", w, h)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let renderer = window.renderer().build().unwrap();
    
    let mut canvas = SdlCanvas::new(renderer, w as usize, h as usize);

    canvas.triangle(
        Vec3i::new(20, 30, 20),
        Vec3i::new(40, 90, 180),
        Vec3i::new(10, 150, 150),
        0xffeeaa
    );
    
    let center = Vec3i::new(150, 150, 30);
    let r = 50.0;
    let step = 0.1;
    
    let mut color = 0xff;
    
    for a in 1..((360.0/step) as i32) {
        use std::f32;
    
        let d = a as f32 * step / (2.0 * f32::consts::PI);
        let x = center.x() as f32 + r * d.cos();
        let y = center.y() as f32 + r * d.sin();
        
        let v = Vec3i::new(x as i32, y as i32, center.z());
        
        canvas.line(center, v, color);
        canvas.set_pixel(v, 0xff00aa);
        color += 0xff;
    }
    
    let light_dir = Vec3f::new(0.0, 0.0, -1.0);

    let model = Model::load_from_file("obj/african_head.obj");
    for face in model.faces {        
        let mut screen_coords = [Vec3f::new(0.0, 0.0, 0.0); 3];
        let mut world_coords = [Vec3f::new(0.0,0.0,0.0); 3];
    
        for i in 0..3 {
            let world = model.verticies[face[i] as usize];
            
            screen_coords[i] = Vec3f::new(
                ((world.x + 1.0) * w as f32 / 2.0), 
                h as f32 - ((world.y + 1.0) * h as f32 / 2.0), 
                world.z as f32 * d as f32
            );
            world_coords[i] = world;
        }
         
        let mut n: Vec3f = ((world_coords[2]-world_coords[0]) ^ (world_coords[1]-world_coords[0])).normalized();        
        let intensity = light_dir * n;
        
        if intensity > 0.0 {
        
            let l = (255.0 * intensity) as u32;
            let color = l | l << 8 | l << 16;

            canvas.triangle(
                screen_coords[0].round(),
                screen_coords[1].round(),
                screen_coords[2].round(),
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
