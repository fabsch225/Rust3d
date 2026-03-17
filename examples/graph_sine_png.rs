#![allow(unused)]

use std::path::Path;
use std::sync::Arc;

use image::{Rgb, RgbImage};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Point;

use rust3d::engine::camera::RayCamera;
use rust3d::engine::utils::rendering::RayRenderScene;
use rust3d::engine::utils::transformation::{Transformable, PI};
use rust3d::geometry::quad::Quad;
use rust3d::geometry::vector3::Vector3 as V;
use rust3d::math::functions::FunctionR2ToR;
use rust3d::math::graph::Graph3D;

const W: usize = 1200;
const H: usize = 900;

pub fn main() -> Result<(), String> {
    // Graph bounds and function z = sin(x) * cos(y)
    let bounds = Quad::new(
        V { x: 0.0, y: 0.0, z: -1.0 },
        V { x: 2.0, y: 1.5, z: 0.75 },
        sdl2::pixels::Color::RGB(255, 0, 0),
    );

    let mut graph = Graph3D::new(
        bounds,
        FunctionR2ToR::new(Box::new(|x, y| (2.5 * x).sin() * (2.5 * y).cos() * 0.35)),
        vec!["x", "y", "z"],
    );
    graph.rot(V { x: PI / 2.0, y: 0.0, z: 0.0 });
    graph.rot(V { x: PI, y: 0.0, z: 0.0 });

    let mut scene = RayRenderScene::new();
    scene.wrap(Box::new(Graph3D::wrapup(&graph)));

    let camera = RayCamera::new(
        V {
            x: -3.0,
            y: -0.5,
            z: 0.0,
        },
        0.0,
        0.0,
        0.0,
    );

    let pixels = camera.render_frame_multi(Arc::new(scene), W, H, 8);

    // Offscreen canvas for labels + final pixel readback
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("graph_sine_png", W as u32, H as u32)
        .hidden()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().software().build().map_err(|e| e.to_string())?;

    for x in 0..W {
        for y in 0..H {
            let idx = x * H + y;
            let c = pixels[idx];
            canvas.set_draw_color(c);
            canvas.draw_point(Point::new(x as i32, y as i32)).map_err(|e| e.to_string())?;
        }
    }

    camera.render_anker_labels(&graph, &mut canvas, W, H);
    canvas.present();

    let rgb = canvas
        .read_pixels(None, PixelFormatEnum::RGB24)
        .map_err(|e| e.to_string())?;
    let img = RgbImage::from_raw(W as u32, H as u32, rgb)
        .ok_or_else(|| String::from("failed to create image buffer from canvas pixels"))?;

    let out_dir = Path::new("target/renders");
    std::fs::create_dir_all(out_dir).map_err(|e| e.to_string())?;
    let out_file = out_dir.join("graph_sine.png");
    img.save(&out_file).map_err(|e| e.to_string())?;

    println!("Saved {}", out_file.display());
    Ok(())
}
