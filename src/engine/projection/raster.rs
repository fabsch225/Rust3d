/*
 * Author     Fabian Schuller
 * Version   0.1
 * Date        2024
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
use crate::engine::utils::virtual_canvas::Color;

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
/// do a trait??

#[derive(Clone, Debug)]
pub struct Raster {
    pub z: i32,
    pub rec_start: (usize, usize),
    pub rec_end: (usize, usize),
    pub screen_width: usize,
    pub screen_height: usize,
    pub raster_width: usize,
    pub raster_height: usize,
    // field
    pub pixels: Vec<Vec<Color>>
}

///TODO add a SAFE Parameter, so it doesnt panic if i draw off-screen
impl Raster {
    pub fn get(&self, x: i32, y: i32) -> Color {
        //TODO what if illegal parameters?
        self.pixels[y as usize][x as usize]
    }
    pub fn set(&mut self, x: i32, y: i32, color: &Color) {
        if x >= self.raster_width as i32 || y >= self.raster_height as i32 { return; }
        if x < 0 || y < 0 { return; }
        self.pixels[y as usize][x as usize] = *color;
    }
}
