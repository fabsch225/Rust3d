#![allow(unused)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color as SdlColor;
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
use rust3d::engine::utils::rendering::RaySphereable;
use rust3d::engine::utils::renderung_ui::UiElement;
use rust3d::engine::utils::transformation::{PI, TWO_PI};
use rust3d::engine::utils::{rendering::{RayRenderScene, RayRenderable}, transformation::Transformable};
use rust3d::geometry::face::Face;
use rust3d::geometry::quad::Quad;
use rust3d::geometry::vector3::Vector3 as V;
use rust3d::engine::camera::RayCamera;
use rust3d::engine::pathtracing::PathTracingScene;
use rust3d::engine::pathtracing::PathtracingObject;
use rust3d::engine::projection::{Projection, ProjectiveScene};
use rust3d::engine::projection_camera::ProjectionCamera;
use rust3d::engine::utils::virtual_canvas::Color;
use rust3d::geometry::simplex3d::Simplex3D;
use rust3d::geometry::sphere::Sphere;
use rust3d::geometry::line::{Line, Line2D};
use rust3d::geometry::point::Point;
use rust3d::math::functions::FunctionR2ToR;
use rust3d::math::graph::Graph3D;

const W : usize = 800;
const H : usize = 800;
const FRAMERATE : u32 = 60;
const NANOS : u32 = 1_000_000_000 / FRAMERATE;

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

    let camera = ProjectionCamera::new(1.0, 90.0, W, H);
    let line = Line2D::new(Point{x: 0.0, y: 0.0}, Point{x: 300.0, y: 300.0}, 5.0, Color::new(100, 100, 0, 255));

    let mut scene = ProjectiveScene::new();
    scene.add(line);

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

            camera.draw(&mut canvas, &scene);
            canvas.present();
        }
    }
    Ok(())
}
