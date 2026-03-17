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
}

impl Projectable for Line2D {
    fn project(&self, mat: &MatrixND) -> Box<&dyn Projection> {
        Box::new(self as &dyn Projection)
    }
}

impl Projection for Line2D {
    fn rasterize(&self, width: usize, height: usize) -> Raster {
        let thickness = self.width.round().max(1.0) as i32;
        let half_width = thickness / 2;

        let start = (self.start.x as i32, self.start.y as i32);
        let end = (self.end.x as i32, self.end.y as i32);

        let min_x = i32::min(start.0, end.0) - half_width;
        let min_y = i32::min(start.1, end.1) - half_width;
        let max_x = i32::max(start.0, end.0) + half_width;
        let max_y = i32::max(start.1, end.1) + half_width;

        let rec_start_x = min_x.clamp(0, width as i32);
        let rec_start_y = min_y.clamp(0, height as i32);
        let rec_end_x = (max_x + 1).clamp(0, width as i32);
        let rec_end_y = (max_y + 1).clamp(0, height as i32);

        let raster_width = (rec_end_x - rec_start_x).max(0) as usize;
        let raster_height = (rec_end_y - rec_start_y).max(0) as usize;

        let mut raster = Raster {
            z: 0,
            rec_start: (rec_start_x as usize, rec_start_y as usize),
            rec_end: (rec_end_x as usize, rec_end_y as usize),
            screen_width: width,
            screen_height: height,
            raster_height,
            raster_width,
            pixels: vec![vec![Color::new(0, 0, 0, 255); raster_width]; raster_height],
        };

        if raster_width == 0 || raster_height == 0 {
            return raster;
        }

        let local_start = (start.0 - rec_start_x, start.1 - rec_start_y);
        let local_end = (end.0 - rec_start_x, end.1 - rec_start_y);

        //this is because the raster-bounds are fit around the line
        Drawing::bresenham_line_single_color_width(local_start, local_end, thickness, &self.color, &mut raster);
        //Drawing::bresenham_line_single_color((half_width, half_width), (self.end.x as i32 - self.start.x as i32, self.end.y as i32 - self.start.y as i32), &self.color, &mut raster);
        raster
    }
}