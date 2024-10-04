use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};
use crate::engine::projection::ProjectiveScene;

pub struct ProjectionCamera {
    pub near: f64,
    pub far: f64,
    pub fov: f64,
    pub aspect_ratio: f64,
}

impl ProjectionCamera {
    pub fn new(aspect_ratio: f64, fov: f64) -> Self {
        ProjectionCamera {
            near: 0.1,
            far: 1000.0,
            fov,
            aspect_ratio
        }
    }

    pub fn draw(canvas : &mut Canvas<Window>, scene: ProjectiveScene) {

    }
}