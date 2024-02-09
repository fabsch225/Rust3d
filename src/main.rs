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
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use rand::Rng;

pub fn main() -> Result<(), String>{

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("rust3d", 500, 500)
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

    let mut p2 : P = P::parse_wavefront(String::from("data/horse.obj"), String::from("data/horse_tex.png"));
    let mut p3 : P = P::parse_wavefront(String::from("data/ref_cube.obj"), String::from("data/standart_text.jpg"));
    //let mut p2 : P = P::parse_wavefront(String::from("data/whale.obj"), String::from("data/whale.jpg"));

    p2.rot(V3{x: 3.14*1.5, y: 0.0, z: 0.0});
    p2.trans(V3{x: 0.0, y: -1.0, z: -1.0});

    p3.trans(V3{x: 0.0, y: 0.0, z: 0.0});
    p3.scale(V3{x: 3.0, y: 3.0, z: 3.0});

	let mut camera : PTC = PTC::new(V3{x: -5.0, y: 0.0, z: 0.0}, 0.0, 0.0, 270.0);
    
    let mut objs : POs = POs::new();

    objs.add(p2);
    objs.add(p3);
    
    let mut p : V3 = V3{x: 10.0, y: 10.0, z: 10.0};

    let mut objs_arc = Arc::new(RwLock::new(objs));

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
        
        objs_arc.write().unwrap().get(0).rot(V3{x: 0.4, y: 0.0, z: 0.0});
        
        canvas = render(canvas, Arc::clone(&objs_arc), camera);

        ::std::thread::sleep(Duration::new(0, 1_000_000_00u32 / 60));
    }

    Ok(())
}

pub fn render(mut canvas: Canvas<Window>, objs_arc: Arc<RwLock<POs>>, camera: PTC) -> Canvas<Window> {
    
    canvas.clear();
    
    let (tx, rx) = mpsc::channel::<(usize, Vec<Color>)>();

    let camera_arc = Arc::new(camera);

    for i in 0..10 {
        let objs_arc = Arc::clone(&objs_arc);
        let camera_arc = Arc::clone(&camera_arc);
        let tx = tx.clone();

        thread::spawn(move || {
            let objs = objs_arc.read().unwrap();
            let section = camera_arc.render_section(0, i.to_owned() * 50, 500, (i.to_owned() + 1) * 50, objs.deref(), 500, 500);
            tx.send((i.to_owned(), section));
        });
    }

    println!("Started rendering without issues");

    for i in 0..10 {

        let section = rx.recv().unwrap();

        camera.draw_section(&section.1, &mut canvas, section.0 * 50, 0, (section.0 + 1) * 50, 500);
    }

    println!("Done rendering");

    canvas.present();

   
    return canvas;
}