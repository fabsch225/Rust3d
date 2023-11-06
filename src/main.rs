#![allow(unused)]

mod engine_structure;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use rand::Rng;


pub fn main() -> Result<(), String>{

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

	let mut c : u8 = 0;
	
	let mut rng = rand::thread_rng();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    
    let mut event_pump = sdl_context.event_pump()?;

    let (w, h) = canvas.output_size().unwrap();
        
    let mut points = [Point::new(0, 0); 800*600];
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
     
        for i in 0..h {
        	for j in 0..w {
        		//let p : Point = Point::new(w as i32, h as i32);
        		
        		c = ((i * j) % 255) as u8;
        		
        		canvas.set_draw_color(Color::RGB(c, 64, 255));
        		
        		canvas.draw_point(Point::new(w as i32, h as i32));
        	}
        }
        
        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

    }
    Ok(())
}
