/*
 *      Author    Fabian Schuller
 *      Version   0.1
 *      Date      2024
 *
 *      This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 *     You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::engine::drawing::drawing::Drawing;
use crate::engine::projection::raster::Raster;
use crate::engine::utils::virtual_canvas::Color;

impl Drawing {
    ///TODO Implement Point Struct
    // Bresenham's line algorithm to rasterize a line
    pub fn bresenham_line_single_color(
        start: (i32, i32),
        end: (i32, i32),
        color: &Color,
        raster: &mut Raster,
    ) {
        let mut x = start.0;
        let mut y = start.1;
        let dx = (end.0 - start.0).abs();
        let dy = -(end.1 - start.1).abs();
        let sx = if start.0 < end.0 { 1 } else { -1 };
        let sy = if start.1 < end.1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            //TODO this is not required
            if x >= 0 && x < raster.screen_width as i32 && y >= 0 && y < raster.pixels.len() as i32 {
                raster.set(x, y, color);
            }

            if x == end.0 && y == end.1 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                if x == end.0 { break; }
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                if y == end.1 { break; }
                err += dx;
                y += sy;
            }
        }
    }

    ///TODO Implement Point Struct, Vector Struct. I HATE THIS STATE
    ///TODO Implement an oriented circle
    ///TODO Not Filled? Filled?
    // this algorithm is stupid
    pub fn bresenham_line_single_color_width(
        start: (i32, i32),
        end: (i32, i32),
        width: i32,
        color: &Color,
        raster: &mut Raster,
    ) {
        let down_shift_x = end.0 - start.0;
        let down_shift_y = end.1 - start.1;
        let length = ((down_shift_x * down_shift_x + down_shift_y * down_shift_y) as f64).sqrt();
        let radius = (width as f64 / 2.0) as i32;
        let unit_dx = down_shift_x as f64 / length;
        let unit_dy = down_shift_y as f64 / length;
        let perp_dx = -unit_dy;
        let perp_dy = unit_dx;
        let half_width = width / 2;
        let left_shift_x = (perp_dx * half_width as f64).round() as i32;
        let left_shift_y = (perp_dy * half_width as f64).round() as i32;
        let perp_line_start = (start.0 - left_shift_x, start.1 - left_shift_y);
        let perp_line_end = (start.0 + left_shift_x, start.1 + left_shift_y);
        let mut x = perp_line_start.0;
        let mut y = perp_line_start.1;
        let dx = (perp_line_end.0 - perp_line_start.0).abs();
        let dy = -(perp_line_end.1 - perp_line_start.1).abs();
        let sx = if perp_line_start.0 < perp_line_end.0 { 1 } else { -1 };
        let sy = if perp_line_start.1 < perp_line_end.1 { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            //TODO this is not required
            if x >= 0 && x < raster.screen_width as i32 && y >= 0 && y < raster.pixels.len() as i32 {
                Drawing::bresenham_line_single_color((x,y), (x + down_shift_x, y + down_shift_y), color, raster);
            }

            if x == perp_line_end.0 && y == perp_line_end.1 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
        /*
        for i in -half_width..=half_width {
            let line_start = (perp_line_start.0 + left_shift_x, perp_line_start.1 + left_shift_y);
            let line_end = (perp_line_end.0 + left_shift_x, perp_line_end.1 + left_shift_y);


            Drawing::bresenham_line_single_color(line_start, line_end, color, raster);
        }
        */

        Drawing::filled_midpoint_circle(start.0, start.1, radius, color, raster);
        Drawing::filled_midpoint_circle(end.0, end.1, radius, color, raster);
    }

    pub fn draw_horizontal_line(
        x_start: i32,
        x_end: i32,
        y: i32,
        color: &Color,
        raster: &mut Raster,
    ) {
        for x in x_start..=x_end {
            raster.set(x, y, color);
        }
    }

    //Wallis, Bob, Rendering Fat Lines on a Raster Grid, Graphics Gems, p. 114-120.
}
