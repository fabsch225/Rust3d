/*
 *      Author    Fabian Schuller
 *      Version   0.1
 *      Date        2024
 *
 *      This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 *     You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
 */


#![allow(unused)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color as SdlColor;
use std::time::Instant;
use std::time::Duration;

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
    let window = video_subsystem.window("Lines", W as u32, H as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let mut event_pump = sdl_context.event_pump()?;

    let camera = ProjectionCamera::new(1.0, 90.0, W, H);
    let line1 = Line2D::new(Point{x: 0.0, y: 0.0}, Point{x: 300.0, y: 300.0}, 5.0, Color::new(255, 220, 0, 255));
    let line2 = Line2D::new(Point{x: 50.0, y: 700.0}, Point{x: 750.0, y: 100.0}, 7.0, Color::new(255, 80, 80, 255));
    let line3 = Line2D::new(Point{x: 100.0, y: 120.0}, Point{x: 780.0, y: 120.0}, 4.0, Color::new(80, 220, 255, 255));
    let line4 = Line2D::new(Point{x: 400.0, y: 50.0}, Point{x: 400.0, y: 760.0}, 9.0, Color::new(160, 255, 120, 255));
    let line5 = Line2D::new(Point{x: 120.0, y: 760.0}, Point{x: 760.0, y: 640.0}, 3.0, Color::new(220, 120, 255, 255));

    let mut scene = ProjectiveScene::new();
    scene.add(line1);
    scene.add(line2);
    scene.add(line3);
    scene.add(line4);
    scene.add(line5);

    println!("Starting main Loop");
    'running: loop {
        let frame_start = Instant::now();

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

        canvas.set_draw_color(SdlColor::RGBA(0, 0, 0, 255));
        canvas.clear();
        camera.draw(&mut canvas, &scene);
        canvas.present();

        let elapsed = frame_start.elapsed();
        let target = Duration::from_nanos(NANOS as u64);
        if elapsed < target {
            std::thread::sleep(target - elapsed);
        }
    }
    Ok(())
}
