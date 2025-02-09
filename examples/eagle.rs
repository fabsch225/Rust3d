#![allow(unused)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::{self, Canvas};
use sdl2::video::Window;

use std::borrow::Borrow;
use std::ops::Deref;
use std::sync::{mpsc, Arc, RwLock, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;

use rust3d::engine::simplex3d_sphere_tree::poly_tree::PolyTree;
use rust3d::engine::raymarching::RayMarchingScene;
use rust3d::engine::utils::rendering::RaySphereable;
use rust3d::engine::utils::rendering_ui::UiElement;
use rust3d::engine::utils::transformation::{PI, TWO_PI};
use rust3d::engine::utils::{rendering::{RayRenderScene, RayRenderable}, transformation::Transformable};
use rust3d::geometry::face::Face;
use rust3d::geometry::quad::Quad;
use rust3d::geometry::vector3::Vector3 as V;
use rust3d::engine::camera::RayCamera;
use rust3d::engine::pathtracing::PathTracingScene;
use rust3d::engine::pathtracing::PathtracingObject;
use rust3d::geometry::simplex3d::Simplex3D;
use rust3d::geometry::sphere::Sphere;
use rust3d::geometry::line::Line;
use rust3d::math::functions::FunctionR2ToR;
use rust3d::math::graph::Graph3D;

const W : usize = 800;
const H : usize = 800;
const FRAMERATE : u32 = 60;
const NANOS : u32 = 1_000_000_000 / FRAMERATE;
const VARIABLE_RENDER_SPEED : u8 = 35;
const TURN_SPEED : f64 = 0.0035;

pub fn main() -> Result<(), String>{
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Eagle", W as u32, H as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let mut event_pump = sdl_context.event_pump()?;
    let mut state;

    let t = Instant::now();
    println!("Starting to parse objects");

   let mut t1 = Simplex3D::parse_wavefront(&String::from("demo_assets/models/eagle.obj"), &String::from("demo_assets/models/orzel-mat_Diffuse.jpg"));
    t1.scale(V{x: 0.1, y: 0.1,z: 0.1});
    let mut t1 = *PolyTree::new(t1);

    let mut pa_objs : PathTracingScene = PathTracingScene::new();

    t1.goto(V{x: 1.0, y: 0.0, z: 0.0});
    t1.rot(V{x: 0.0, y: 0.0, z: PI});

    pa_objs.add(t1);


    let pa_objs = Arc::new(RwLock::new(pa_objs));

    let mut camera : RayCamera = RayCamera::new(V{x: -3.0, y: 0.0, z: 0.0}, 0.0, 0.0, 0.0);

    let mut stage = 1;
    let mut modulus_size = 300;
    let mut change_modulus = 0;
    let mut block_size = 20;
    let mut motion = true; //first render without this condition
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

        if event_pump
            .mouse_state()
            .is_mouse_button_pressed(MouseButton::Left)
        {
            state = event_pump.relative_mouse_state();
            let rot_y = TURN_SPEED * state.x() as f64;
            if (rot_y != 0.0) {
                motion = true;
                block_size = 10;
            }

            if (rot_y > 0.0) {
                pa_objs.write().unwrap().get(0).rot(V{x: 0.0, y: rot_y, z: 0.0});
            }
            else {
                pa_objs.write().unwrap().get(0).rot_reverse(V{x: 0.0, y: - rot_y, z: 0.0});
            }
        }

        println!("Starting transformation");
        let now = Instant::now();

        let mut objs: RayRenderScene = RayRenderScene::new();

        objs.wrap(Box::new(PathTracingScene::wrapup(&pa_objs.read().unwrap())));
        if (motion) {
            render_multi(&mut canvas, objs, camera, &W, &H);
            canvas.present();
        }
        else {
            ::std::thread::sleep(Duration::new(0, NANOS as u32));
        }
    }
    Ok(())
}

pub fn render_mod(canvas : &mut Canvas<Window>, objs : RayRenderScene, camera : RayCamera, w_ : &usize, h_ : &usize, modulus : usize, stage : usize) {
    let w = canvas.window().drawable_size().0 as usize;
    let h = canvas.window().drawable_size().1 as usize;
    let section = camera.render_modulus(&objs, w, h, stage, modulus);
    camera.draw_modulus(&section, canvas, stage, modulus, *w_, *h_);
}

pub fn render_multi(canvas : &mut Canvas<Window>, objs : RayRenderScene, camera : RayCamera, w_ : &usize, h_ : &usize) {
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