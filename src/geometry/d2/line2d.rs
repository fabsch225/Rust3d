/*
 * Author     Fabian Schuller
 * Version   0.1
 * Date        2024
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
use crate::engine::drawing::drawing::Drawing;
use crate::engine::projection::projection::{Projectable, Projection};
use crate::engine::projection::raster::Raster;
use crate::engine::utils::virtual_canvas::Color;
use crate::geometry::point::Point;
use crate::math::matrix::MatrixND;

pub struct Line2D {
    pub start: Point,
    pub end: Point,
    pub width: f64,
    pub color: Color,
}

impl Line2D {
    pub fn new(start: Point, end: Point, width: f64, color: Color) -> Line2D {
        Line2D {
            start,
            end,
            width,
            color,
        }
    }
    // Helper to convert floating point coordinates to integer grid coordinates
    fn to_grid_coords(&self, width: usize, height: usize) -> ((i32, i32), (i32, i32)) {
        let start_x = self.start.x as i32;
        let start_y = self.start.y as i32;
        let end_x = self.end.x as i32;
        let end_y = self.end.y as i32;
        ((start_x, start_y), (end_x, end_y))
    }

    fn to_raster_coords(&self, width: usize, height: usize) -> ((i32, i32), (i32, i32)) {
        let start_x = self.start.x as i32 - (self.width / 2.0) as i32;
        let start_y = self.start.y as i32 - (self.width / 2.0) as i32;;
        let end_x = self.end.x as i32 + (self.width / 2.0) as i32;
        let end_y = self.end.y as i32 + (self.width / 2.0) as i32;
        ((start_x, start_y), (end_x, end_y))
    }
}

impl Projectable for Line2D {
    fn project(&self, mat: &MatrixND) -> Box<&dyn Projection> {
        Box::new(self.clone())
    }
}

impl Projection for Line2D {
    fn rasterize(&self, width: usize, height: usize) -> Raster {
        let (start, end) = self.to_grid_coords(width, height);
        let (raster_start, raster_end) = self.to_raster_coords(width, height);
        let raster_width = i32::abs(raster_start.0 - raster_end.0) as usize;
        let raster_height = i32::abs(raster_start.1 - raster_end.1) as usize;
        let half_width = (self.width / 2.0) as i32;
        let mut raster = Raster {
            z: 0,
            rec_start: (i32::min(raster_start.0, raster_end.0) as usize, i32::min(raster_start.1, raster_end.1) as usize),
            rec_end: (i32::max(raster_start.0, raster_end.0) as usize, i32::max(raster_start.1, raster_end.1) as usize),
            screen_width: width,
            screen_height: height,
            raster_height,
            raster_width,
            pixels: vec![vec![Color::new(0, 0, 0, 255); raster_width]; raster_height],
        };

        //this is because the raster-bounds are fit around the line
        Drawing::bresenham_line_single_color_width((half_width, half_width), (self.end.x as i32 - self.start.x as i32, self.end.y as i32 - self.start.y as i32), self.width as i32, &self.color, &mut raster);
        //Drawing::bresenham_line_single_color((half_width, half_width), (self.end.x as i32 - self.start.x as i32, self.end.y as i32 - self.start.y as i32), &self.color, &mut raster);
        raster
    }
}