#![allow(unused)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::borrow::Borrow;
use rust3d::engine::projection::projection::ProjectiveScene;
use rust3d::engine::projection_camera::ProjectionCamera;
use rust3d::engine::utils::virtual_canvas::Color;
use rust3d::geometry::d2::line2d::Line2D;
use rust3d::geometry::point::Point;

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
    let line = Line2D::new(Point{x: 145.0, y: 145.0}, Point{x: 300.0, y: 300.0}, 50.0, Color::new(255, 255, 0, 55));
    let line2 = Line2D::new(Point{x: 500.0, y: 500.0}, Point{x: 650.0, y: 500.0}, 25.0, Color::new(0, 255, 0, 55));
    let line3 = Line2D::new(Point{x: 300.0, y: 500.0}, Point{x: 400.0, y: 700.0}, 90.0, Color::new(0, 0, 255, 55));

    let mut scene = ProjectiveScene::new();
    scene.add(line);
    scene.add(line2);
    scene.add(line3);

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
