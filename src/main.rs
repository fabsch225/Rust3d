#![allow(unused)]

mod poly_shape;
mod engine_pa;
mod engine_rm;
mod point; 
mod face;
mod cube; 
mod sphere;

use engine_rm::{RayMarchingCamera as Camera, RayMarchingObjects, RayMarchingObject};

use engine_pa::{PathtracingCamera as PTC, PathtracingObject as PO, PathtracingObjects as POs};

use sphere::Sphere;
use cube::Cube;
use face::Face;
use point::Point as V3;
use poly_shape::Poly as P;
use poly_shape::Collision as Collision;

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

    let mut f1 : Face = Face::new(V3{x:20.0, y: -5.0, z: -5.0}, V3{x:20.0, y: -5.0, z: 5.0}, V3{x: 20.0, y: 5.0, z: -5.0});
    let mut pl : P = P::new(V3{x: 10.0, y: 0.0, z: 0.0}, vec![f1]);

    //let mut p2 : P = P::parse_wavefront(String::from("data/horse.obj"), String::from("data/horse_tex.png"));
    let mut p2 : P = P::parse_wavefront(String::from("data/ref_cube.obj"), String::from("data/standart_text.jpg"));
    p2.rot(V3{x: 3.14, y: 0.0, z: 0.0});
    p2.trans(V3{x: 0.0, y: -1.0, z: -1.0});

	let mut camera : PTC = PTC::new(V3{x: -5.0, y: 0.0, z: 0.0}, 0.0, 0.0, 270.0);
    
	
    let mut objs : POs = POs::new();

    objs.add(p2);
    
    let mut p : V3 = V3{x: 10.0, y: 10.0, z: 10.0};

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
                
                camera.render_pixel_at(j, i, &mut canvas, &objs, 500, 500);
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
        
    
        objs.get(0).rot(V3{x: 0.1, y: 0.15, z: -0.05});

        for i in 0..500 {
            for j in 0..500 {
               
                camera.render_pixel_at(j, i, &mut canvas, &objs, 500, 500);
            }  
        }

        canvas.present();
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
