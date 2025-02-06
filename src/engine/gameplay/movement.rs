/*
 * Author     Fabian Schuller
 * Version   0.1
 * Date        2024
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use crate::geometry::vector3::Vector3 as V;
use crate::engine::camera::RayCamera;
use crate::geometry::face::Face;

pub struct MovementInputMap {
    pub forward: Keycode,
    pub backward: Keycode,
    pub left: Keycode,
    pub right: Keycode,
    pub up: Keycode,
    pub down: Keycode,
    pub rotate_left: Keycode,
    pub rotate_right: Keycode,
}

impl MovementInputMap {
    pub fn get_default() -> Self {
        Self {
            forward: Keycode::W,
            backward: Keycode::S,
            left: Keycode::A,
            right: Keycode::D,
            up: Keycode::Space,
            down: Keycode::LShift,
            rotate_left: Keycode::Q,
            rotate_right: Keycode::E,
        }
    }
}

pub struct PlayerMovementController<'a> {
    event_pump: &'a mut EventPump,
    camera: &'a mut RayCamera,
    input_map: MovementInputMap,
    velocity: V,
    acceleration: V,
    mass: f64,
    friction: f64,
    speed: f64,
    yaw: f64,
    pitch: f64,
    sensitivity: f64,
}

impl<'a> PlayerMovementController<'a> {
    pub fn new(event_pump: &'a mut EventPump, camera: &'a mut RayCamera, input_map: MovementInputMap) -> Self {
        Self {
            event_pump,
            camera,
            input_map,
            velocity: V { x: 0.0, y: 0.0, z: 0.0 },
            acceleration: V { x: 0.0, y: 0.0, z: 0.0 },
            mass: 1.0,
            friction: 0.99,
            speed: 20.0,
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 0.001
        }
    }

    pub fn get_camera(&self) -> RayCamera {
        self.camera.clone()
    }

    pub fn handle_input(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(key), .. } => {
                    match key {
                        k if k == self.input_map.forward => self.acceleration.x = self.speed,
                        k if k == self.input_map.backward => self.acceleration.x = -self.speed,
                        k if k == self.input_map.left => self.acceleration.z = -self.speed,
                        k if k == self.input_map.right => self.acceleration.z = self.speed,
                        k if k == self.input_map.up => self.acceleration.y = self.speed,
                        k if k == self.input_map.down => self.acceleration.y = -self.speed,
                        k if k == self.input_map.rotate_left => self.yaw -= 0.1,
                        k if k == self.input_map.rotate_right => self.yaw += 0.1,
                        k if k == Keycode::Escape => {
                            return true;
                        }
                        _ => {}
                    }
                }
                Event::KeyUp { keycode: Some(key), .. } => {
                    match key {
                        k if k == self.input_map.forward || k == self.input_map.backward => self.acceleration.x = 0.0,
                        k if k == self.input_map.left || k == self.input_map.right => self.acceleration.z = 0.0,
                        k if k == self.input_map.up || k == self.input_map.down => self.acceleration.y = 0.0,
                        _ => {}
                    }
                }
                Event::MouseMotion { xrel, yrel, .. } => {
                    self.yaw += xrel as f64 * self.sensitivity;
                    self.pitch -= yrel as f64 * self.sensitivity;
                    self.pitch = self.pitch.clamp(-1.57, 1.57); // Limit pitch to avoid flipping

                    self.camera.set_rot(V { x: 0.0, y: self.yaw, z: self.pitch });
                }
                Event::Quit { .. } => {
                    return true;
                }
                _ => {}
            }
        }

        /*let forward = V {
            x: self.yaw.sin() * self.pitch.cos(),
            y: self.pitch.sin(),
            z: self.yaw.cos() * self.pitch.cos()
        };*/
        let forward = V {
            x: self.yaw.cos(),
            y: 0.0,
            z: self.yaw.sin()
        };
        let right = V {
            x: self.yaw.sin(),
            y: 0.0,
            z: -self.yaw.cos()
        };

        let movement = V {
            x: forward.x * self.acceleration.x + right.x * self.acceleration.z,
            y: self.acceleration.y,
            z: forward.z * self.acceleration.x + right.z * self.acceleration.z
        };

        self.velocity.x += movement.x / self.mass;
        self.velocity.y += movement.y / self.mass;
        self.velocity.z += movement.z / self.mass;

        self.velocity.x *= 1.0 - self.friction;
        self.velocity.y *= 1.0 - self.friction;
        self.velocity.z *= 1.0 - self.friction;

        self.camera.position.x += self.velocity.x;
        self.camera.position.y += self.velocity.y;
        self.camera.position.z += self.velocity.z;

        println!("pitch: {:?} yaw: {:?}", self.pitch, self.yaw);

        false
    }
}