#![allow(unused)]

mod engine_structure;

use engine_structure::Camera;
use engine_structure::Cube;
use engine_structure::Point as V3;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use rand::Rng;


pub fn main() -> Result<(), String>{

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo", 400, 400)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

	let mut c : u8 = 0;
	let mut r : u32 = 0;
	
	let mut rng = rand::thread_rng();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    
    let mut event_pump = sdl_context.event_pump()?;

	
	let mut camera : Camera = Camera::new(V3{x: 0.0, y: 0.0, z: 0.0}, 0.0, 0.0, 0.0);

	let mut cube : Cube = Cube::new(V3{x:5.0, y: 0.0, z: 0.0}, 2.0);
	
    let (w, h) = canvas.output_size().unwrap();
    
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
        		
        		let vxp : f64 = j as f64 / w as f64;
        		let vyp : f64 = i as f64 / h as f64;
        		
        		let v0 : V3 = camera.x;
        		let v : V3 = V3{
	        		x: 1.0,//camera.v[0].x, 
	        		y: -0.5 + vxp,//camera.v[0].y + (camera.v[0].y - camera.v[1].y) * vxp + (camera.v[0].y - camera.v[2].y) * vxp, 
	        		z: -0.5 + vyp//camera.v[0].z + (camera.v[0].z - camera.v[1].z) * vyp + (camera.v[0].z - camera.v[2].z) * vyp
        		};
        		let mut p : V3 = v0;
        		let mut d : f64 = 0.0;
        		let mut last_d : f64 = v0.d(cube.m);
        		
        		loop {
		            d = p.d(cube.m);
					r = cube.has_point(p);
					
		            if (r != 0) {
		            	c = 100 + r as u8 * 10;
		            	break;
		            }
		            else if (d > last_d) {
		            	c = 50;
		            	break;
		            }
		            else {
		            	last_d = d;
		            	p.trans(v.x * d / 2.0, v.y * d / 2.0, v.z * d / 2.0);
		            }
		            
        		}
        		
        		canvas.set_draw_color(Color::RGB(c, 100, 255));
        		
        		canvas.draw_point(Point::new(j as i32, i as i32));
        	}
        	
        }
        //break;
        
        canvas.present();
        
        cube.rot(0.01, 0.01, 0.0);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    }
    Ok(())
}
