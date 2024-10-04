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

use rust3d::engine::polytree::poly_tree::PolyTree;
use rust3d::engine::raymarching::RayMarchingScene;
use rust3d::engine::utils::rendering::Sphereable;
use rust3d::engine::utils::renderung_ui::UiElement;
use rust3d::engine::utils::transformation::{PI, TWO_PI};
use rust3d::engine::utils::{rendering::{RayRenderScene, RayRenderable}, transformation::Transformable};
use rust3d::geometry::face::Face;
use rust3d::geometry::quad::Quad;
use rust3d::geometry::vector3::Vector3 as V;
use rust3d::engine::camera::RayCamera;
use rust3d::engine::pathtracing::PathTracingScene;
use rust3d::engine::pathtracing::PathtracingObject;
use rust3d::geometry::poly_shape::Poly;
use rust3d::geometry::sphere::Sphere;
use rust3d::geometry::line::Line;
use rust3d::math::functions::FunctionR2ToR;
use rust3d::math::graph::Graph3D;

const W : usize = 1200;
const H : usize = 1200;
const FRAMERATE : u32 = 60;
const NANOS : u32 = 1_000_000_000 / FRAMERATE;
const VARIABLE_RENDER_SPEED : u8 = 35;
const TURN_SPEED : f64 = 0.0035;

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
    let mut state;

    let font = include_bytes!("../demo_assets/fonts/NotoSansMath-Regular.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

    let t = Instant::now();
    println!("Starting to parse objects");

    let mut p1 = Quad::new(V{x: 0.0, y: 0.0, z: 0.0}, V{x: 1., y: 2., z: 1.}, Color::RED);

    let mut g1 = Graph3D::new(p1, FunctionR2ToR::new(Box::new(|x, y| - x*x -  y*y + 1.0)), vec!["x", "y", "z"]);
    g1.rot(V{x: PI / 2., y: 0.0, z: 0.0});
    let root = p1.x[7];
    let mut label1 = rust3d::engine::utils::anker_label::AnkerLabel::new(root.x, root.y, root.z, String::from("Root"), &font, Color::RED, Color::WHITE);

    let mut camera : RayCamera = RayCamera::new(V{x: -3.0, y: 0.0, z: 0.0}, 0.0, 0.0, 0.0);

    let mut stage = 1;
    let mut modulus_size = 300;
    let mut change_modulus = 0;
    let mut block_size = 20;
    let mut motion = true; //first render without this condition
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
            if (rot_y != 0.0) { //rot_z != 0.0 ||
                motion = true;
                block_size = 10;
            }
            if (rot_y > 0.0) {
                g1.rot(V{x: 0.0, y: rot_y, z: 0.0});
            }
            else {
                g1.rot_reverse(V{x: 0.0, y: - rot_y, z: 0.0});
            }
        }
        let now = Instant::now();
        let mut objs: RayRenderScene = RayRenderScene::new();
        objs.wrap(Box::new(Graph3D::wrapup(&g1)));

        if (motion) {
            camera.render_and_draw_modulus_block(&mut canvas, &objs, block_size, stage, modulus_size / block_size, W, H);

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
            if (stage >= modulus_size / block_size) {
                stage = 0;
                if (block_size > 1) {
                    block_size /= 4;
                    if block_size < 1 {
                        block_size = 1;
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
                else {
                    motion = false;
                    camera.render_anker_labels(&g1, &mut canvas, W, H);
                }
            }
            canvas.present();
        }
        else {
            ::std::thread::sleep(Duration::new(0, NANOS as u32));
        }
    }
    Ok(())
}
