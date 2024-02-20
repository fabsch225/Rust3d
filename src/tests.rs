use std::sync::mpsc;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Instant;
use std::ops::Deref;

use sdl2::pixels::Color;

use crate::engine_pa::PathtracingObject as PO;
use crate::engine_pa::PathtracingCamera as PTC;
use crate::engine_pa::PathtracingObjects as POs;
use crate::point::Point as V3;
use crate::polytree::poly_tree::PolyTree as PT;
use crate::poly_shape::Poly as P;
use crate::render;

#[test]
pub fn load_poly_test() {
    let mut p1 = P::parse_wavefront(&String::from("samples/horse.obj"), &String::from("samples/horse_tex.png"));
}

#[test]
pub fn make_polytree_test() {
    let mut p1 = P::parse_wavefront(&String::from("samples/horse.obj"), &String::from("samples/horse_tex.png"));
    let mut p1 : PT = *PT::new(p1);
}

#[test]
pub fn render_test() {
    let w : usize = 1000;
    let h : usize = 1000;
   
    let mut p1 = P::parse_wavefront(&String::from("samples/horse.obj"), &String::from("samples/horse_tex.png"));
    p1.rot(V3{x: 3.14*1.5, y: 0.0, z: 3.14*1.6});
    let mut p1 : PT = *PT::new(p1); 
    let mut objs : POs = POs::new();
    objs.add(p1);
	let mut camera : PTC = PTC::new(V3{x: -5.0, y: 0.0, z: 0.0}, 0.0, 0.0, 270.0);
    let objs_arc = Arc::new(RwLock::new(objs));
    

    let objs_ = objs_arc.read().unwrap();
    let section = camera.render_modulus(objs_.deref(), w, h, 3, 10);
}

#[test]
pub fn render_multithreading_test() {
    let w : usize = 1000;
    let h : usize = 1000;
   
    let mut p1 = P::parse_wavefront(&String::from("samples/horse.obj"), &String::from("samples/horse_tex.png"));
    p1.rot(V3{x: 3.14*1.5, y: 0.0, z: 3.14*1.6});
    let mut p1 : PT = *PT::new(p1); 
    let mut objs : POs = POs::new();
    objs.add(p1);
    let mut camera : PTC = PTC::new(V3{x: -5.0, y: 0.0, z: 0.0}, 0.0, 0.0, 270.0);

    println!("Setting up threads...");
    let now = Instant::now();


    let objs_arc = Arc::new(RwLock::new(objs));
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
            let objs_ = objs_arc.read().unwrap();
            let section = camera_arc.render_modulus(objs_.deref(), w_, h_, i, n);
            tx.send((i.to_owned(), section));
        });
    }
    println!("Setup took {}ms", now.elapsed().as_millis());
    println!("Started rendering without issues");
    let now = Instant::now();

    for i in 0..n {
        let section = rx.recv().unwrap();
        println!("Thread {} finished rendering", section.0);
       
    }

    println!("Render took {}ms", now.elapsed().as_millis());
}