/*
 * Author     Fabian Schuller
 * Version   0.1
 * Date        2024
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use sdl2::pixels::Color;
use crate::engine::utils::transformation::Transformable;
use crate::geometry::vector3::Vector3 as V3;

#[derive(Clone)]
pub struct Light {
    pub position: V3,
    pub color: Color,
    pub intensity: f64,
}

impl Transformable for Light {
    fn rot(&mut self, r: V3) {
        self.position.rotate(r);
    }

    fn rot_by(&mut self, p: V3, r: V3) {
        self.position.rot_by(p, r);
    }

    fn translate(&mut self, p: V3) {
        todo!()
    }

    fn scale(&mut self, p: V3) {
        todo!()
    }

    fn transform(&mut self) -> Box<&mut dyn Transformable> {
        todo!()
    }
}

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub diffuse: f64,
}

impl Material {
    pub fn new(color: Color, diffuse: f64) -> Self {
        Material { color, diffuse }
    }
}