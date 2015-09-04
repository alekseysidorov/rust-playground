extern crate sdl2;

use std::vec::Vec;
use std::f64::consts::PI;

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::Renderer;

fn draw_koch(renderer: &mut Renderer, p: Point, q: Point, n: i32)
{
    if n == 0 {
        renderer.draw_line(p, q);
    } else {
        let r = Point::new(
            (2 * p.x() + q.x()) / 3,
            (2 * p.y() + q.y()) / 3
        );
        
        let s;
        {
            let d : f32 = f32::sqrt(3.0) / 6.0;
            let f1: f32 = (p.y() - q.y()) as f32 * d;
            let f2: f32 = (p.y() - q.y()) as f32 * d;
            
            s = Point::new(
                ((p.x() + q.x()) as f32 / 2.0 - f1) as i32,
                ((p.y() + q.y()) as f32 / 2.0 + f2) as i32,
            );
        }
        
        let t = Point::new(
            ((p.x() + 2 * q.x()) as f32 / 3.0) as i32,
            ((p.y() + 2 * q.y()) as f32 / 3.0) as i32        
        );
        
        draw_koch(renderer, p, r, n - 1);
        draw_koch(renderer, r, s, n - 1);
        draw_koch(renderer, s, t, n - 1);
        draw_koch(renderer, t, q, n - 1);
    }
}

fn draw_koch_snow_flake(renderer: &mut Renderer, c: Point, d: f64, n: i32, m: usize)
{
    let mut vs = Vec::with_capacity(m);
    
    for i in 0..m {
        let x: f64 = c.x() as f64 + d * f64::cos((2.0*PI / m as f64) * i as f64);
        let y: f64 = c.y() as f64 - d * f64::sin((2.0*PI / m as f64) * i as f64);
        let p = Point::new(
            x as i32,
            y as i32,
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
    
    let w: u32 = 1280;
    let h: u32 = 1024;
    
    let window = video_subsystem.window("Rust sdl example", w, h)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
        
    let mut renderer = window.renderer().build().unwrap();
    
    renderer.set_draw_color(Color::RGB(0,0,0));
    renderer.clear();
    
    renderer.set_draw_color(Color::RGB(255,255,255));
    draw_koch_snow_flake(&mut renderer, Point::new(w as i32 / 2, h as i32 / 2), h as f64 / 2.0, 8, 4);
    
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
