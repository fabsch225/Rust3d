#![allow(unused)]

mod engine;

use engine::{RayMarchingCamera as Camera, RayMarchingObjects, RayMarchingObject};

use engine::sphere::Sphere;
use engine::cube::Cube;
use engine::face::Face;
use engine::point::Point as V3;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use rand::Rng;

pub fn main() -> Result<(), String>{

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo", 300, 300)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

	let mut c : u8 = 0;
	let mut r : u32 = 0;
	
	let mut rng = rand::thread_rng();

    //canvas.set_draw_color(Color::RGB(0, 255, 255));
    
    let mut event_pump = sdl_context.event_pump()?;

	let mut camera : Camera = Camera::new(V3{x: 0.0, y: 0.0, z: 0.0}, 0.0, 0.0, 270.0);

	let mut cube : Cube = Cube::new(V3{x:20.0, y: 1.5, z: 0.0}, 6.0, Color::RGB(255, 0, 0));
    let mut cube2 : Cube = Cube::new(V3{x:20.0, y: -1.5, z: 0.0}, 6.0, Color::RGB(0, 0, 255));
    let mut sphere : Sphere = Sphere::new(V3{x:25.0, y: 0.0, z: 5.0}, 3.0, Color::RGB(0, 255, 0));
	
    let mut objs : RayMarchingObjects = RayMarchingObjects::new();

    cube2.rot_reverse(V3{x: 0.0, y: 0.5, z: 0.0});
    cube.rot(V3{x: 0.5, y: 0.5, z: 0.0});

    objs.add(cube2);
	objs.add(cube);
    //objs.add(sphere);
    
    let mut p : V3 = V3{x: 10.0, y: 10.0, z: 10.0};

    'running: loop {
   		//break;
    	objs.get(0).rot(V3{x: 0.1, y: 0.1, z: 0.0});
        objs.get(1).rot(V3{x: 0.0, y: -0.1, z: 0.0});
        //camera.rot(V3{x: 0.0, y: 0.01, z: 0.0});

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
     
        camera.render(&mut canvas, &objs);
        
        canvas.present();
		
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    }
    Ok(())
}
