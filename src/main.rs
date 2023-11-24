#![allow(unused)]

mod engine_rm;
mod point; 
mod face;
mod cube; 
mod sphere;

use engine_rm::{RayMarchingCamera as Camera, RayMarchingObjects, RayMarchingObject};

use sphere::Sphere;
use cube::Cube;
use face::Face;
use point::Point as V3;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use rand::Rng;

pub fn main() -> Result<(), String>{

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("rust-sdl2 demo", 500, 500)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let mut event_pump = sdl_context.event_pump()?;


    let pixel_total : usize = 500 * 500;
    let ppx : usize = 50 * 500;
    let mut pixel_progress: usize = 0;

    let mut f1 : Face = Face::new(V3{x: 10.0, y: 0.0, z: 0.0}, V3{x: 10.0, y: 0.0, z: 5.0}, V3{x: 10.0, y: 5.0, z: 0.0});

	let mut camera : Camera = Camera::new(V3{x: -1.0, y: 0.0, z: 0.0}, 0.0, 0.0, 270.0);

	let mut cube : Cube = Cube::new(V3{x:20.0, y: 1.5, z: 0.0}, 6.0, Color::RGB(255, 0, 0));
    let mut cube2 : Cube = Cube::new(V3{x:20.0, y: -1.5, z: 0.0}, 6.0, Color::RGB(0, 0, 255));
    let mut sphere : Sphere = Sphere::new(V3{x:25.0, y: 0.0, z: 5.0}, 3.0, Color::RGB(0, 255, 0));
	
    let mut objs : RayMarchingObjects = RayMarchingObjects::new();

    cube2.rot_reverse(V3{x: 0.0, y: 0.5, z: 0.0});
    cube.rot(V3{x: 0.5, y: 0.5, z: 0.0});

    objs.add(cube2);
	objs.add(cube);
    
    
    let mut p : V3 = V3{x: 10.0, y: 10.0, z: 10.0};

    objs.get(0).rot(V3{x: 0.1, y: 0.1, z: 0.0});
    objs.get(1).rot(V3{x: 0.0, y: -0.1, z: 0.0});
    
    canvas.clear();
    
    'running: loop {
        if (pixel_progress >= 500) {
            break;
        }
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


        for i in 0..500 {
            for j_ in 0..5 {
                let j = pixel_progress + j_;
                let vxp : f64 = j as f64 / 500.0;
        		let vyp : f64 = i as f64 / 500.0;
        		
        		let v0 : V3 = camera.x;
				let b : V3 = V3{x: camera.v[0].x - v0.x, y: camera.v[0].y - v0.y, z: camera.v[0].z - v0.z};

        		let v : V3 = V3{
	        		x: b.x,
	        		y: b.y + (camera.v[1].y - camera.v[0].y) * vyp + (camera.v[2].y - camera.v[0].y) * vxp,
	        		z: b.z + (camera.v[1].z - camera.v[0].z) * vyp + (camera.v[2].z - camera.v[0].z) * vxp
        		};

        		let mut p : V3 = v0;
        		let mut d : f64 = 0.0;
        		let mut c = Color::RGB(51, 51, 51); //TODO Base-Color as Attribute of RMC

        		loop {
		            //d = objs.nearest_distance_smoothed(p, self.epsilon * 0.5f64);
					d = objs.nearest_distance(p);
					
		            if (d < camera.epsilon) {
		            	//c = objs.current_color(p); // need delta function that exaddertes the edges WRONG!
						c = objs.current_color_gradient(p, 10f64);
		            	break;
		            }
		            else if (p.d(v0) > camera.view_distance) {
		            	c = Color::RGB(51, 51, 51);
		            	break;
		            }
		            else {
		            	p.trans(v.x * d / 2.0, v.y * d / 2.0, v.z * d / 2.0);
		            }
        		}

        		canvas.set_draw_color(c);
        		
        		canvas.draw_point(Point::new(j as i32, i as i32));
            }  
        }

        pixel_progress += 5;

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

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
    }

    Ok(())
}
