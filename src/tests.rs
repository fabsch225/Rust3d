use std::sync::Arc;
use std::sync::RwLock;

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

pub fn init_canvas_test() -> Result<(), String> {
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
    Ok(())
}

#[test]
pub fn render_test() -> Result<(), String> {
    let w : usize = 1000;
    let h : usize = 1000;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust3d", w as u32, h as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut p1 = P::parse_wavefront(&String::from("samples/horse.obj"), &String::from("samples/horse_tex.png"));
    p1.rot(V3{x: 3.14*1.5, y: 0.0, z: 3.14*1.6});
    let mut p1 : PT = *PT::new(p1); 
    let mut objs : POs = POs::new();
    objs.add(p1);
	let mut camera : PTC = PTC::new(V3{x: -5.0, y: 0.0, z: 0.0}, 0.0, 0.0, 270.0);
    let objs_arc = Arc::new(RwLock::new(objs));
    render(&mut canvas, Arc::clone(&objs_arc), camera, &w, &h);
    Ok(())
}