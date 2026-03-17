#![allow(unused)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color as SdlColor;

use std::thread;
use std::time::{Duration, Instant};

use rust3d::engine::projection::projection::ProjectiveScene;
use rust3d::engine::projection_camera::ProjectionCamera;
use rust3d::engine::utils::virtual_canvas::Color;
use rust3d::geometry::nline::NLine;
use rust3d::math::matrix::MatrixND;
use rust3d::math::vector::NVector;

const W: usize = 900;
const H: usize = 900;
const FRAMERATE: u32 = 60;
const NANOS: u32 = 1_000_000_000 / FRAMERATE;

fn tesseract_vertices() -> Vec<NVector> {
    let mut vertices = Vec::with_capacity(16);
    for i in 0..16 {
        let x = if i & 0b0001 == 0 { -1.0 } else { 1.0 };
        let y = if i & 0b0010 == 0 { -1.0 } else { 1.0 };
        let z = if i & 0b0100 == 0 { -1.0 } else { 1.0 };
        let w = if i & 0b1000 == 0 { -1.0 } else { 1.0 };
        vertices.push(NVector::from_vec(vec![x, y, z, w]));
    }
    vertices
}

fn tesseract_edges() -> Vec<(usize, usize, usize)> {
    let mut edges = Vec::with_capacity(32);
    for i in 0..16 {
        for dim in 0..4 {
            let j = i ^ (1 << dim);
            if i < j {
                edges.push((i, j, dim));
            }
        }
    }
    edges
}

fn edge_color(dim: usize) -> Color {
    match dim {
        0 => Color::new(255, 90, 90, 255),
        1 => Color::new(90, 255, 150, 255),
        2 => Color::new(90, 170, 255, 255),
        _ => Color::new(255, 220, 90, 255),
    }
}

fn build_tesseract_nlines() -> Vec<NLine> {
    let vertices = tesseract_vertices();
    let edges = tesseract_edges();
    let mut lines = Vec::with_capacity(edges.len());

    for (a, b, dim) in edges {
        let width = if dim == 3 { 5.0 } else { 3.0 };
        lines.push(NLine::new(
            vertices[a].clone(),
            vertices[b].clone(),
            width,
            edge_color(dim),
            170.0,
        ));
    }

    lines
}

fn rotate_nline(line: &NLine, t: f64) -> NLine {
    let rot_xw = MatrixND::givens_rotation_from_indices(4, 0, 3, t * 0.65);
    let rot_yz = MatrixND::givens_rotation_from_indices(4, 1, 2, t * 0.40);
    let rot_zw = MatrixND::givens_rotation_from_indices(4, 2, 3, t * 0.30);
    let rot_4d = rot_xw
        .multiply_single_thread(&rot_yz)
        .multiply_single_thread(&rot_zw);

    NLine::new(
        rot_4d.multiply_nvector(&line.a),
        rot_4d.multiply_nvector(&line.b),
        line.width,
        line.color,
        line.scale,
    )
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Teseract", W as u32, H as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");
    let mut event_pump = sdl_context.event_pump()?;

    let camera = ProjectionCamera::new(1.0, 90.0, W, H);
    let start_time = Instant::now();
    let base_lines = build_tesseract_nlines();

    println!("Starting teseract loop");
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

        let t = start_time.elapsed().as_secs_f64();
        let mut scene = ProjectiveScene::new();
        for line in base_lines.iter() {
            scene.add(rotate_nline(line, t));
        }

        canvas.set_draw_color(SdlColor::RGBA(0, 0, 0, 255));
        canvas.clear();
        camera.draw(&mut canvas, &scene);
        canvas.present();

        let elapsed = frame_start.elapsed();
        let target = Duration::from_nanos(NANOS as u64);
        if elapsed < target {
            thread::sleep(target - elapsed);
        }
    }

    Ok(())
}
