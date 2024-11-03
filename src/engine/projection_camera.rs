use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};
use crate::engine::projection::projection::ProjectiveScene;
use crate::engine::utils::rendering::RayRenderable;
use crate::engine::utils::virtual_canvas::VirtualCanvas;
use crate::math::matrix::MatrixND;

//ToDo Transformations
pub struct ProjectionCamera {
    pub near: f64,
    pub far: f64,
    pub fov: f64,
    pub aspect_ratio: f64,
    pub width: usize,
    pub height: usize,
}

impl ProjectionCamera {
    pub fn new(aspect_ratio: f64, fov: f64, width: usize, height: usize) -> Self {
        ProjectionCamera {
            near: 0.1,
            far: 1000.0,
            fov,
            aspect_ratio,
            width,
            height,
        }
    }

    pub fn draw(&self, canvas : &mut Canvas<Window>, scene: &ProjectiveScene) {
        let matrix = MatrixND::projection3d2d(self.fov, self.aspect_ratio, self.near, self.far);
        let mut vcanvas = VirtualCanvas::new(self.width, self.height);
        //ToDO translate

        scene.render(&matrix, &mut vcanvas);

        for x in 0..self.width {
            for y in 0..self.height {
                let color = vcanvas.get_sdl2_color(x, y);

                canvas.set_draw_color(color);
                canvas.draw_point(Point::new(x as i32, y as i32));
            }
        }
    }
}