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
            raster.set(x, y, color);

            if x == end.0 && y == end.1 {
                break;
            }
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
        let thickness = width.max(1);
        if thickness == 1 {
            Drawing::bresenham_line_single_color(start, end, color, raster);
            return;
        }

        let radius = thickness / 2;

        if start == end {
            Drawing::filled_midpoint_circle(start.0, start.1, radius, color, raster);
            return;
        }

        let mut x = start.0;
        let mut y = start.1;
        let dx = (end.0 - start.0).abs();
        let dy = -(end.1 - start.1).abs();
        let sx = if start.0 < end.0 { 1 } else { -1 };
        let sy = if start.1 < end.1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            Drawing::filled_midpoint_circle(x, y, radius, color, raster);

            if x == end.0 && y == end.1 {
                break;
            }

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
