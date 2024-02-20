#![allow(unused)]

mod poly_shape;
mod engine_pa;
mod engine_rm;
mod point; 
mod face;
mod cube; 
mod sphere;
mod engine_utils;
mod tests;

mod polytree {
    pub mod poly_tree;
    pub mod poly_tree_element;
    pub mod poly_tree_utils;
}

use engine_pa::{PathtracingCamera as PTC, PathtracingObject as PO, PathtracingObjects as POs};

use face::Face;
use point::Point as V3;
use poly_shape::Poly as P;
use polytree::poly_tree::PolyTree as PT;
use polytree::poly_tree_element::PolyTreeElement as PTE;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::ops::Deref;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;
use std::time::Duration;
use std::time::Instant;

pub fn main() -> Result<(), String>{
    let w : usize = 1000;
    let h : usize = 1000;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("rust3d", w as u32, h as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let mut event_pump = sdl_context.event_pump()?;

    let t = Instant::now();
    println!("Starting to parse wavefront file");

    //let mut p1 = P::parse_wavefront(&String::from("samples/eagle.obj"), &String::from("samples/orzel-mat_Diffuse.jpg"));
    let mut p2 = P::parse_wavefront(&String::from("samples/ref_cube.obj"), &String::from("samples/standart_text.jpg"));
    //let mut p1 = P::parse_wavefront(&String::from("samples/whale.obj"), &String::from("samples/whale.jpg"));
    let mut p1 = P::parse_wavefront(&String::from("samples/horse.obj"), &String::from("samples/horse_tex.png"));

    println!("Parsing took {}ms", t.elapsed().as_millis());

    p1.rot(V3{x: 3.14*1.5, y: 0.0, z: 3.14*1.6});

    let t = Instant::now();
    println!("Starting to create polytree from poly");

    let mut p1 : PT = *PT::new(p1); 
    let mut p2 : PT = *PT::new(p2);  
    
    println!("Creating polytree took {}ms", t.elapsed().as_millis());

    p1.trans(V3{x: 0.0, y: -1.0, z: 0.0});
    p2.trans(V3{x: 7.0, y: 0.0, z: 2.0});
    p2.scale(V3{x: 15.0, y: 15.0, z: 15.0});

    let mut objs : POs = POs::new();
    objs.add(p1);
    objs.add(p2);
    
	let mut camera : PTC = PTC::new(V3{x: -5.0, y: 0.0, z: 0.0}, 0.0, 0.0, 270.0);
    let objs_arc = Arc::new(RwLock::new(objs));

    println!("Starting main Loop");
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
        println!("Starting transformation");
        let now = Instant::now();
       
        //camera.rot(V3{x: 0.0, y: 0.1, z: 0.0});
        objs_arc.write().unwrap().get(0).rot(V3{x: -0.1, y: 0.0, z: 0.0});
        
        println!("transformation took {}ms", now.elapsed().as_millis());

        render(&mut canvas, Arc::clone(&objs_arc), camera, &w, &h);

        ::std::thread::sleep(Duration::new(0, 1_000_000u32 / 60));
    }
    Ok(())
}

pub fn render(canvas : &mut Canvas<Window>, objs_arc : Arc<RwLock<POs>>, camera : PTC, w : &usize, h : &usize) {
    
    canvas.clear();
    println!("Setting up threads...");
    let now = Instant::now();

    let (tx, rx) = mpsc::channel::<(usize, Vec<Color>)>();
    let n = 16;
    let camera_arc = Arc::new(camera);

    for i in 0..n {
        let objs_arc = Arc::clone(&objs_arc);
        let camera_arc = Arc::clone(&camera_arc);
        let tx = tx.clone();
        let w_ = w.clone();
        let h_ = h.clone();

        thread::spawn(move || {
            let objs = objs_arc.read().unwrap();
            let section = camera_arc.render_modulus(objs.deref(), w_, h_, i, n);
            tx.send((i.to_owned(), section));
        });
    }
    println!("Setup took {}ms", now.elapsed().as_millis());
    println!("Started rendering without issues");
    let now = Instant::now();

    for i in 0..n {
        let section = rx.recv().unwrap();

        camera.draw_modulus(&section.1, canvas, section.0, n, *w, *h);

        println!("Thread {} finished rendering", section.0);
       
    }

    println!("Render took {}ms", now.elapsed().as_millis());
    canvas.present();
}

    
