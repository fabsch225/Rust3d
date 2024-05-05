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
    pub mod quad;
    pub mod sphere;
    pub mod line;
}

mod math {
    pub mod graph;
    pub mod matrix;
    pub mod functions;
    pub mod utils {
        pub mod graph_utils;
    }
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
use crate::engine::utils::rendering::Sphereable;
use crate::engine::utils::renderung_ui::UiElement;
use crate::engine::utils::{rendering::{RenderObjects, Renderable}, transformation::Transformable};
use crate::geometry::face::Face;
use crate::geometry::quad::Quad;
use crate::geometry::point::Point as V;
use crate::engine::camera::Camera;
use crate::engine::pathtracing::PathtracingObjects;
use crate::engine::pathtracing::PathtracingObject;
use crate::geometry::poly_shape::Poly;
use crate::geometry::sphere::Sphere;
use crate::geometry::line::Line;
use crate::math::functions::FunctionR2ToR;
use crate::math::graph::Graph3D;
 
const W : usize = 600;
const H : usize = 600;
const FRAMERATE : u32 = 60;
const NANOS : u32 = 1_000_000_000 / FRAMERATE;
const VARIABLE_RENDER_SPEED : u8 = 35;

///Todos
/// - [ ] Camera should have w and h as parameters and map them to the canvas obj.
/// - [ ] Refactor polytree to be untexured and textured
/// - [ ] Fix RM coloring
/// - [ ] implement rectanguar Face
/// - [x] implement rot_by for transformable

pub fn main() -> Result<(), String>{
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("rust3d", W as u32, H as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let mut event_pump = sdl_context.event_pump()?;

    let font = include_bytes!("../demo_assets/fonts/NotoSansMath-Regular.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

    
    let t = Instant::now();
    println!("Starting to parse objects");

    
    let mut p1 = Quad::new(V{x: 0.0, y: 0.0, z: 0.0}, V{x: 1., y: 2., z: 1.}, Color::RED);
    let mut p2 = Sphere::new(V{x: 2.0, y: 1.0, z: 1.0}, 0.01, Color::GREEN);

    let mut t1 = Poly::parse_wavefront(&String::from("demo_assets/models/horse.obj"), &String::from("demo_assets/models/horse_tex.png"));
    //let mut t1 = Poly::parse_wavefront(&String::from("demo_assets/models/eagle.obj"), &String::from("demo_assets/models/orzel-mat_Diffuse.jpg"));
    t1.scale(V{x: 0.7, y: 0.7, z: 0.7});
    let mut t1 = *PolyTree::new(t1); 

    
    
    let mut pa_objs : PathtracingObjects = PathtracingObjects::new();
    //pa_objs.add(t1);
    
    let mut rm_objs : RayMarchingObjects = RayMarchingObjects::new(0.005);
    //rm_objs.add(line1);
    //rm_objs.add(p1);
    //rm_objs.add(p2);
    //rm_objs.add(m2);

    let f1 =  Face::new(V{x: 0.0, y: 0.0, z: 0.0}, V{x: 0.0, y: 0.0, z: 2.0}, V{x: 0.0, y: 2.0, z: 0.0});
    let f1 = Poly::new(f1.get_middle(), vec![f1]);

    //pa_objs.add(f1);
    let mut line1 = Line::new(p1.x[7], p1.x[6], 0.01);
    let mut p2 = Sphere::new(p1.x[6], 0.1, Color::GREEN);
    //rm_objs.add(line1);
    //rm_objs.add(p2);
    //t1.goto(V{x: 1.0, y: 1.0, z: 1.0});
    //t1.scale(V{x: 0.1, y: 0.1, z: 0.1});
    //pa_objs.add(t1);

    let mut g1 = Graph3D::new(p1, FunctionR2ToR::new(Box::new(|x, y| f64::sin(y * 3.))), vec!["x", "y", "z"]);
    //rm_objs.add(p1);
    let root = p1.x[7];
    let mut label1 = engine::utils::anker_label::AnkerLabel::new(root.x, root.y, root.z, String::from("Root"), &font, Color::RED, Color::WHITE);

    //let s2 = Sphere::new(p1.x[5], 0.12, Color::BLUE);
    //rm_objs.add(s1);

    let rm_objs = Arc::new(RwLock::new(rm_objs));
    let pa_objs = Arc::new(RwLock::new(pa_objs));   
    
	let mut camera : Camera = Camera::new(V{x: -3.0, y: 0.0, z: 0.0}, 0.0, 0.0, 0.0);
    
    let mut stage = 1;
    let mut modulus_size = 400;
    let mut change_modulus = 0;
    //println!("Starting main Loop");
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
        //println!("Starting transformation");
        let now = Instant::now();
        //g1.rot(V{x: 0.0, y: 0.0, z: 0.1});

        //pa_objs.write().unwrap().get(0).rot(V{x: -0.1, y: 0.0, z: 0.1});
        //rm_objs.write().unwrap().get(0).rot(V{x: -0.1, y: 0.1, z: 0.0}); 
        //rm_objs.write().unwrap().get(0).translate(V{x: 0.0, y: 0.01, z: 0.0});
        //rm_objs.write().unwrap().get(1).translate(V{x: 0.01, y: 0.01, z: 0.01});

       
        let mut objs: RenderObjects = RenderObjects::new();
        
        //objs.wrap(Box::new(PathtracingObjects::wrapup(&pa_objs.read().unwrap())));
        //objs.wrap(Box::new(RayMarchingObjects::wrapup(&rm_objs.read().unwrap())));
        objs.wrap(Box::new(Graph3D::wrapup(&g1)));
        //camera.rot(V{x: 0.0, y: 0.1, z: 0.0});
        
        //println!("transformation took {}ms", now.elapsed().as_millis());
        
        //println!("Starting rendering {} {}" , stage, modulus_size);

        render_mod(&mut canvas, objs, camera, &W, &H, modulus_size, stage);
        
        //camera.render_anker_labels(&g1, &mut canvas, w, h);
        canvas.present();

        let diff = now.elapsed().as_nanos();
        if ((diff as u32) < NANOS) {
            ::std::thread::sleep(Duration::new(0, NANOS - diff as u32));
            if (modulus_size > 1) {
                change_modulus -= 1;
            }
        }
        else {
            change_modulus += 1;
        }

        stage += 1;

        if (stage == modulus_size) {
            stage = 1;
            if (change_modulus > 0) {
                modulus_size += VARIABLE_RENDER_SPEED as usize;
            }
            else if (change_modulus < 0) {
                modulus_size -= VARIABLE_RENDER_SPEED as usize;
                if (modulus_size < 1) {
                    modulus_size = 1;
                }
            }
            change_modulus = 0;
        }
    }
    Ok(())
}

pub fn render_mod(canvas : &mut Canvas<Window>, objs : RenderObjects, camera : Camera, w_ : &usize, h_ : &usize, modulus : usize, stage : usize) {
    let w = canvas.window().drawable_size().0 as usize;
    let h = canvas.window().drawable_size().1 as usize;
    let section = camera.render_modulus(&objs, w, h, stage, modulus);
    camera.draw_modulus(&section, canvas, stage, modulus, *w_, *h_);
}

pub fn render_multi(canvas : &mut Canvas<Window>, objs : RenderObjects, camera : Camera, w_ : &usize, h_ : &usize) {
    //let w = canvas.window().drawable_size().0 as usize;
    //let h = canvas.window().drawable_size().1 as usize;
    //canvas.clear();
    println!("Setting up threads...");
    let now = Instant::now();

    let (tx, rx) = mpsc::channel::<(usize, Vec<Color>)>();
    let n = 8;
    let camera_arc = Arc::new(camera);
    let objs =  Arc::new(objs);

    for i in 0..n {
        let camera_arc = Arc::clone(&camera_arc);
        let objs = Arc::clone(&objs);
        let tx = tx.clone();
        let w_ = w_.clone();
        let h_ = h_.clone();

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

        camera.draw_modulus(&section.1, canvas, section.0, n, *w_, *h_);

        println!("Thread {} finished rendering", section.0);
    }

    println!("Render took {}ms", now.elapsed().as_millis());
}

    
