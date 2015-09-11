extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;

struct SdlCanvas
{
    renderer: Renderer<'static>
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
        let dx = (a.x() - b.x()).abs();
        let dy = (a.y() - b.y()).abs();
        
        let mut x = a.x();
        let mut y = a.y();

        let sx = if b.x() - x > 0 { 1 } else { -1 };
        let sy = if b.y() - y > 0 { 1 } else { -1 };
        if dx > dy {
            let mut dx = 0;
            let mut dy = 0;
            while x != b.x() {
                let rx = (b.x() - x).abs();
                let ry = (b.y() - y).abs();
                
                dx += rx;
                dy += ry;
                if dy >= dx {
                    y += sy;
                }
            
                self.set_pixel(x, y, color);
                x += sx;
                dx -= rx + sx;
                dy -= rx + sx;
            }
        } else {
            let mut dx = 0;
            let mut dy = 0;
            while y != b.y() {
                let rx = (b.x() - x).abs();
                let ry = (b.y() - y).abs();
                
                dx += rx;
                dy += ry;
                if dx >= dy {
                    y += sx;
                }
            
                self.set_pixel(x, y, color);
                y += sy;
                dx -= ry;
                dy -= ry;
            }
        }
    }
    
    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        self.renderer.set_draw_color(Color::RGB((color >> (8*2)) as u8, (color >> (8*1)) as u8, color as u8));
        self.renderer.draw_point(Point::new(x, y));
        self.renderer.present();
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    
    let mut canvas = SdlCanvas::new(renderer);
    canvas.line(Point::new(100, 50), Point::new(400, 200), 0xFF);
    canvas.line(Point::new(400, 200), Point::new(300, 250), 0xFFFFF);
    
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
