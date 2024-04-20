#![allow(unused)]

mod engine {
    pub mod camera;
    pub mod pathtracing;
    pub mod raymarching;
    pub mod utils {
        pub mod anker_label;
        pub mod rendering;
        pub mod transformation;
        pub mod renderung_ui;
    }
    
    pub mod polytree {
        pub mod poly_tree;
        pub mod poly_tree_element;
        pub mod poly_tree_utils;
    }
}

mod geometry {
    pub mod poly_shape;
    pub mod face;
    pub mod point;
    pub mod cube;
    pub mod sphere;
}

mod math {
    pub mod graph;
    pub mod matrix;
}

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{self, Canvas};
use sdl2::video::Window;

use std::borrow::Borrow;
use std::ops::Deref;
use std::sync::{mpsc, Arc, RwLock, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::engine::polytree::poly_tree::PolyTree;
use crate::engine::raymarching::RayMarchingObjects;
use crate::engine::utils::{rendering::{RenderObjects, Renderable}, transformation::Transformable};
use crate::geometry::cube::Cube;
use crate::geometry::point::Point as V;
use crate::engine::camera::Camera;
use crate::engine::pathtracing::PathtracingObjects;
use crate::engine::pathtracing::PathtracingObject;
use crate::geometry::poly_shape::Poly;
use crate::geometry::sphere::Sphere;
use crate::math::graph::Line;

pub fn main() -> Result<(), String>{
    let w : usize = 400;
    let h : usize = 300;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("rust3d", w as u32, h as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let mut event_pump = sdl_context.event_pump()?;

    let label1 = engine::utils::anker_label::AnkerLabel::new(0.0, 0.0, 0.0, String::from("Hello Rust!"), String::from("demo_assets/fonts/Roboto-Regular.ttf"), &canvas);

    let t = Instant::now();
    println!("Starting to parse objects");

    let mut line1 = Line::new(V{x: 2.0, y: 1.0, z: 1.0}, V{x: 0.0, y: 0.0, z: 0.0}, 0.01);
    let mut p1 = Cube::new(V{x: 0.0, y: 0.0, z: 0.0}, 0.2, Color::RED);
    let mut p2 = Sphere::new(V{x: 2.0, y: 1.0, z: 1.0}, 0.01, Color::GREEN);

    let mut t1 = Poly::parse_wavefront(&String::from("demo_assets/models/horse.obj"), &String::from("demo_assets/models/horse_tex.png"));
    //let mut t1 = Poly::parse_wavefront(&String::from("demo_assets/models/whale.obj"), &String::from("demo_assets/models/whale.jpg"));
    let mut t1 = *PolyTree::new(t1); 

    t1.translate(V{x: 5.0, y: -1.0, z: 0.0});
    //let mut m1 = Cube::new(V{x: 0.0, y: 0.0, z: 0.0}, 3.0, Color::RED);
    /*let mut m2 = Cube::new(V{x: 0.0, y: 0.0, z: 0.0}, 3.0, Color::BLUE);

    //let mut p1 = P::parse_wavefront(&String::from("samples/eagle.obj"), &String::from("samples/orzel-mat_Diffuse.jpg"));
    let mut p2 = Poly::parse_wavefront(&String::from("samples/ref_cube.obj"), &String::from("samples/standart_text.jpg"));
    //let mut p1 = Poly::parse_wavefront(&String::from("samples/whale.obj"), &String::from("samples/whale.jpg"));
    let mut p1 = Poly::parse_wavefront(&String::from("samples/horse.obj"), &String::from("samples/horse_tex.png"));

    println!("Parsing took {}ms", t.elapsed().as_millis());

    p1.rot(V{x: 3.14*1.5, y: 0.0, z: 3.14*1.6});

    let t = Instant::now();
    println!("Starting to create polytree from poly");

    let mut p1 : PolyTree = *PolyTree::new(p1); 
    let mut p2 : PolyTree = *PolyTree::new(p2);  
    
    println!("Creating polytree took {}ms", t.elapsed().as_millis());

    p1.translate(V{x: 0.0, y: -1.0, z: 0.0});
    p2.translate(V{x: 7.0, y: 0.0, z: 2.0});
    p2.scale(V{x: 15.0, y: 15.0, z: 15.0});

    m1.translate(V{x: 7.0, y: 0.0, z: 2.0});
    m2.translate(V{x: 7.0, y: 3.0, z: 2.0});
    */
    let mut pa_objs : PathtracingObjects = PathtracingObjects::new();
    pa_objs.add(t1);
    
    let mut rm_objs : RayMarchingObjects = RayMarchingObjects::new(0.05);
    //rm_objs.add(line1);
    //rm_objs.add(p1);
    //rm_objs.add(p2);
    //rm_objs.add(m2);

    let rm_objs = Arc::new(RwLock::new(rm_objs));
    let pa_objs = Arc::new(RwLock::new(pa_objs));   
    
	let mut camera : Camera = Camera::new(V{x: -5.0, y: 0.0, z: 0.0}, 0.0, 0.0, 270.0);
    //let pa_objs_arc = Arc::new(RwLock::new(pa_objs));
    //let rm_objs_arc = Arc::new(RwLock::new(rm_objs));
    
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
       
        pa_objs.write().unwrap().get(0).rot(V{x: -0.1, y: 0.0, z: 0.1});
        //rm_objs.write().unwrap().get(0).rot(V{x: 0.0, y: 0.1, z: 0.0}); 
        //rm_objs.write().unwrap().get(0).translate(V{x: 0.0, y: 0.01, z: 0.0});
        //rm_objs.write().unwrap().get(1).translate(V{x: 0.0, y: 0.01, z: 0.0});

        let mut objs: RenderObjects = RenderObjects::new();
        
        objs.wrap(Box::new(PathtracingObjects::wrapup(&pa_objs.read().unwrap())));
        objs.wrap(Box::new(RayMarchingObjects::wrapup(&rm_objs.read().unwrap())));

        //camera.rot(V{x: 0.0, y: 0.1, z: 0.0});
        
        println!("transformation took {}ms", now.elapsed().as_millis());

        
        render(&mut canvas, objs, camera, &w, &h);
        

        ::std::thread::sleep(Duration::new(0, 1_000_000u32 / 60));
    }
    Ok(())
}

pub fn render(canvas : &mut Canvas<Window>, objs : RenderObjects, camera : Camera, w : &usize, h : &usize) {
    canvas.clear();
    println!("Setting up threads...");
    let now = Instant::now();

    let (tx, rx) = mpsc::channel::<(usize, Vec<Color>)>();
    let n = 10;
    let camera_arc = Arc::new(camera);
    let objs =  Arc::new(objs);

    for i in 0..n {
        let camera_arc = Arc::clone(&camera_arc);
        let objs = Arc::clone(&objs);
        let tx = tx.clone();
        let w_ = w.clone();
        let h_ = h.clone();

        thread::spawn(move || {
            //let obj_mutex_cloned = Arc::clone(&obj_mutex_cloned);
            let section = camera_arc.render_modulus_multi(objs, w_, h_, i, n);
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

    
