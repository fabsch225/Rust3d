use std::sync::{mpsc, Arc, RwLock};
use std::thread;
use std::time::Instant;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use rust3d::geometry::vector3::Vector3 as V;
use rust3d::engine::camera::{RayCamera};
use rust3d::engine::lighting::{Light, Material};
use rust3d::engine::raymarching::{RayMarchingScene};
use rust3d::engine::utils::rendering::{RayRenderScene};
use rust3d::engine::utils::transformation::Transformable;
use rust3d::geometry::sphere::Sphere;

const W : usize = 300;
const H : usize = 300;
const FRAMERATE : u32 = 60;
const NANOS : u32 = 1_000_000_000 / FRAMERATE;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("rust3d", W as u32, H as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let mut event_pump = sdl_context.event_pump()?;
    let camera : RayCamera = RayCamera::new(V{x: -3.0, y: 0.0, z: 0.0}, 0.0, 0.0, 0.0);
    let p2 = Sphere::new(V{x: 2.0, y: 1.0, z: 1.0}, 1.0, Color::GREEN);
    let mut rm_objs : RayMarchingScene = RayMarchingScene::new(0.005);
    rm_objs.add(p2);
    rm_objs.add_light(Light {
        position: V{x: 2.0, y: 2.0, z: 5.0},
        color: Color::YELLOW,
        intensity: 1.0,
    });
    rm_objs.add_light(Light {
        position: V{x: -2.0, y: 2.0, z: 3.0},
        color: Color::BLUE,
        intensity: 1.0,
    });
    rm_objs.add_material(Material{ color: Color::YELLOW, diffuse: 1.0 });
    let rm_objs = Arc::new(RwLock::new(rm_objs));
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

        let mut objs: RayRenderScene = RayRenderScene::new();
        rm_objs.write().unwrap().lights[0].rot_by(V::new(2.0,1.0,1.0), V::new(0.1, 0.0, -0.15));

        objs.wrap(Box::new(RayMarchingScene::wrapup(&rm_objs.read().unwrap())));
        render_multi(&mut canvas, objs, camera, &W, &H);
        canvas.present();
    }
    Ok(())
}

pub fn render_multi(canvas : &mut Canvas<Window>, objs : RayRenderScene, camera : RayCamera, w_ : &usize, h_ : &usize) {
    println!("Setting up threads...");
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
            let section = camera_arc.render_modulus_multi(objs, w_, h_, i, n);
            tx.send((i.to_owned(), section));
        });
    }
    let now = Instant::now();

    for i in 0..n {
        let section = rx.recv().unwrap();
        camera.draw_modulus(&section.1, canvas, section.0, n, *w_, *h_);
    }

    println!("Render took {}ms", now.elapsed().as_millis());
}