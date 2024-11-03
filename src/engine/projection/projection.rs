/*
 * Author     Fabian Schuller
 * Version   0.1
 * Date        2024
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::cmp::PartialEq;
use sdl2::{render::Canvas, video::Window};
use crate::engine::pathtracing::PathtracingObject;
use crate::engine::projection::raster::Raster;
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
        //TODO this will not do for clipping!
        rasters = Self::sort_rasters(rasters);
        for raster in rasters {
            for x in raster.rec_start.0..raster.rec_end.0 {
                for y in raster.rec_start.1..raster.rec_end.1 {
                    let color = raster.get((x - raster.rec_start.0) as i32, (y - raster.rec_start.1) as i32);
                    //check bounds
                    if (color == Color::new(0,0,0,255)) {
                        continue;
                    }
                    canvas.draw_pixel(x, y, color);
                    if (color != Color::new(255,255,0,55)) {
                        continue;
                    }
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
