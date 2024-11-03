/*
 *      Author    Fabian Schuller
 *      Version   0.1
 *      Date        2024
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
    pub fn midpoint_circle(x_center: i32, y_center: i32, radius: i32, color: &Color, raster: &mut Raster) {
        let mut x = 0;
        let mut y = radius;
        let mut d = 1 - radius;

        while x <= y {
            // Each point is mirrored across all octants of the circle
            raster.set((x_center + x), (y_center + y), color);
            raster.set((x_center - x), (y_center + y), color);
            raster.set((x_center + x), (y_center - y), color);
            raster.set((x_center - x), (y_center - y), color);
            raster.set((x_center + y), (y_center + x), color);
            raster.set((x_center - y), (y_center + x), color);
            raster.set((x_center + y), (y_center - x), color);
            raster.set((x_center - y), (y_center - x), color);

            // Update decision parameter and coordinates
            if d < 0 {
                d += 2 * x + 3;
            } else {
                d += 2 * (x - y) + 5;
                y -= 1;
            }
            x += 1;
        }
    }

    pub fn filled_midpoint_circle(
        x_center: i32,
        y_center: i32,
        radius: i32,
        color: &Color,
        raster: &mut Raster
    ) {
        let mut x = 0;
        let mut y = radius;
        let mut d = 1 - radius;

        while x <= y {
            // Draw horizontal lines between the left and right boundaries for each y level
            Drawing::draw_horizontal_line(x_center - x, x_center + x, y_center + y, color, raster);
            Drawing::draw_horizontal_line(x_center - x, x_center + x, y_center - y, color, raster);
            Drawing::draw_horizontal_line(x_center - y, x_center + y, y_center + x, color, raster);
            Drawing::draw_horizontal_line(x_center - y, x_center + y, y_center - x, color, raster);

            // Update decision parameter and coordinates
            if d < 0 {
                d += 2 * x + 3;
            } else {
                d += 2 * (x - y) + 5;
                y -= 1;
            }
            x += 1;
        }
    }
}