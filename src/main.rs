#![allow(unused)]

mod poly_shape;
mod engine_pa;
mod engine_rm;
mod point; 
mod face;
mod cube; 
mod sphere;
mod engine_utils;

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
    
    let mut p1 = P::parse_wavefront(&String::from("data/horse.obj"), &String::from("data/horse_tex.png"));
    let mut p2 = P::parse_wavefront(&String::from("data/ref_cube.obj"), &String::from("data/standart_text.jpg"));
    let mut p1 = P::parse_wavefront(&String::from("data/whale.obj"), &String::from("data/whale.jpg"));


    p1.rot(V3{x: 3.14*1.5, y: 0.0, z: 3.14*1.6});

    let mut p1 : PT = *PT::new(p1); 
    let mut p2 : PT = *PT::new(p2);  
    

    
    p1.trans(V3{x: 0.0, y: -1.0, z: -1.0});

    //p2.trans(V3{x: -5.0, y: 0.0, z: 0.0});
    //p2.scale(V3{x: 3.0, y: 3.0, z: 3.0});

    let mut objs : POs = POs::new();

    objs.add(p1);
    //objs.add(p2);
    
	let mut camera : PTC = PTC::new(V3{x: -5.0, y: 0.0, z: 0.0}, 0.0, 0.0, 270.0);
    

    let mut p : V3 = V3{x: 10.0, y: 10.0, z: 10.0};

    let objs_arc = Arc::new(RwLock::new(objs));

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
       
        //objs_arc.write().unwrap().get(0).rot(V3{x: 0.3, y: 0.1, z: -0.1});
        
        println!("transformation took {}ms", now.elapsed().as_millis());

        render(&mut canvas, Arc::clone(&objs_arc), camera);

        ::std::thread::sleep(Duration::new(0, 1_000_000u32 / 60));
    }
    Ok(())
}

pub fn render(canvas: &mut Canvas<Window>, objs_arc: Arc<RwLock<POs>>, camera: PTC) {
    
    canvas.clear();
    println!("Setting up threads...");
    let now = Instant::now();

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
    println!("Setup took {}ms", now.elapsed().as_millis());
    println!("Started rendering without issues");

    for i in 0..10 {

        let section = rx.recv().unwrap();

        camera.draw_section(&section.1, canvas, section.0 * 50, 0, (section.0 + 1) * 50, 500);

        canvas.present();

        println!("Thread {} finished rendering", section.0);
    }

    
}