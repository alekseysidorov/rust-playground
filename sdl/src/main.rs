extern crate sdl2;

use std::vec::Vec;
use std::f64::consts::PI;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::Renderer;

#[derive(Copy, Clone)]
struct PointF {
    x: f64,
    y: f64,
}

impl PointF {

    fn new(x: f64, y: f64) -> PointF
    {
        PointF {
            x: x,
            y: y
        }
    }

    fn round(&self) -> Point
    {
        Point::new(self.x as i32, self.y as i32)
    }
    
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
}

fn draw_koch(renderer: &mut Renderer, p: PointF, q: PointF, n: i32)
{
    if n == 0 {
        renderer.draw_line(p.round(), q.round());
    } else {
        let r = PointF::new(
            (2.0 * p.x() + q.x()) / 3.0,
            (2.0 * p.y() + q.y()) / 3.0
        );
        
        let s;
        {
            let d : f64 = f64::sqrt(3.0) / 6.0;
            let f1: f64 = (p.y() - q.y()) * d;
            let f2: f64 = (p.x() - q.x()) * d;
            
            s = PointF::new(
                ((p.x() + q.x()) / 2.0 - f1),
                ((p.y() + q.y()) / 2.0 + f2),
            );
        }
        
        let t = PointF::new(
            ((p.x() + 2.0 * q.x()) / 3.0),
            ((p.y() + 2.0 * q.y()) / 3.0)        
        );
        
        draw_koch(renderer, p, r, n - 1);
        draw_koch(renderer, r, s, n - 1);
        draw_koch(renderer, s, t, n - 1);
        draw_koch(renderer, t, q, n - 1);
    }
}

fn draw_koch_snow_flake(renderer: &mut Renderer, c: PointF, d: f64, n: i32, m: usize)
{
    let mut vs = Vec::with_capacity(m);
    
    for i in 0..m {
        let m: f64 = m as f64;
        let i: f64 = i as f64;
    
        let x: f64 = c.x() + d * f64::cos((2.0*PI / m) * i);
        let y: f64 = c.y() - d * f64::sin((2.0*PI / m) * i);
        
        let p = PointF::new(
            x,
            y,
        );
        vs.push(p);
    }
    for i in 0..m {
        draw_koch(renderer, vs[(i + 1) % m], vs[i], n);
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let w: u32 = 1000;
    let h: u32 = 1000;
    
    let window = video_subsystem.window("Rust sdl example", w, h)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
        
    let mut renderer = window.renderer().build().unwrap();
    
    renderer.set_draw_color(Color::RGB(0,0,0));
    renderer.clear();
    
    renderer.set_draw_color(Color::RGB(255,255,255));
    draw_koch_snow_flake(&mut renderer, 
                         PointF::new(w as f64 / 2.0, h as f64 / 2.0), 
                         h as f64 / 4.0, 
                         7, 
                         3);
    
    renderer.present();
    
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
    }
}
