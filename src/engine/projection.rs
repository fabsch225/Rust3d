use std::cmp::PartialEq;
use sdl2::{render::Canvas, video::Window};
use crate::engine::pathtracing::PathtracingObject;
use crate::engine::utils::virtual_canvas::{VirtualCanvas, Color};
use crate::geometry::point::Point;
use crate::math::matrix::MatrixND;

pub struct ProjectiveScene {
    pub objects: Vec<Box<dyn Projectable + Sync + Send>>
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.r == other.r && self.g == other.g
    }
}

impl ProjectiveScene {
    pub fn new() -> ProjectiveScene {
        ProjectiveScene {
            objects: vec![]
        }
    }
    pub fn render(&self, projection_matrix: &MatrixND, canvas: &mut VirtualCanvas) {
        let mut rasters = vec![];
        let width = canvas.width;
        let height = canvas.height;
        for object in self.objects.iter() {
            rasters.push(object.project(projection_matrix).rasterize(width, height));
        }
        rasters = Self::sort_rasters(rasters);
        for raster in rasters {
            for x in raster.rec_start.0..raster.rec_end.0 {
                for y in raster.rec_start.1..raster.rec_end.1 {
                    let color = raster.get_color(x - raster.rec_start.0, y - raster.rec_start.1);
                    if (color != Color::new(0,0,0,255)) {
                        //println!("here");
                    }
                    canvas.draw_pixel(x, y, color);
                }
            }
        }
    }

    pub fn add(&mut self, obj : impl Projectable + 'static + Send + Sync) {
        self.objects.push(Box::new(obj));
    }

     pub fn get(&mut self, i : usize) -> &mut Box<dyn Projectable + 'static + Send + Sync> {
        &mut self.objects[i]
    }
}

pub trait Projectable {
    fn project(&self, mat: &MatrixND) -> Box<&dyn Projection>;
}

//Todo [NEXTSTEP] implement struct LineProjection as trait Projection, and draw something 2d
pub trait Projection {
    fn rasterize(&self, width: usize, height: usize) -> Raster;
}

/// Todo [THINK] Maybe draw the pixels differently i.e.
/// struct Pixel {
///     x: usize
///     y: usize
///     z: usize // z-index
///     c: Color
/// }
///
/// and then, render them by iterating the pixel-vector.
/// maybe add another Raster -> SparseRaster that works this way

#[derive(Clone, Debug)]
pub struct Raster {
    pub z: i32,
    pub rec_start: (usize, usize),
    pub rec_end: (usize, usize),
    pub width: usize,
    pub height: usize,
    // field
    pub pixels: Vec<Vec<Color>>
}

impl Raster {
    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
    }
}
