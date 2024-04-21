use fontdue::Font;
use sdl2::{pixels::Color, render::{Canvas, Texture}, video::Window};

use crate::engine::utils::{rendering::{RenderObjects, Renderable}, transformation::Transformable};
use crate::geometry::point::Point as V3;

use super::renderung_ui::UiElement;

pub struct AnkerLabel {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub text: String,
    pub size: f64,
    pub color: Color,
    pub background_color: Color,
    pub font: Font,
    pub visible: bool,
    pub texture_size: (u32, u32),
    pub texture: Vec<Color>,
}

impl AnkerLabel {
    pub fn new(x_: f64, y_: f64, z_: f64, text_: String, font_ : Font, bg: Color, fg: Color) -> Self {
        let (metrics, bitmap) = font_.rasterize('A', 500.0);
        let size = (metrics.bounds.height as u32, metrics.bounds.width as u32);

        let mut texture = Vec::new();
        
        for i in 0..bitmap.len() {
            let r  = (bitmap[i] / 255) * fg.r + (1 - bitmap[i] / 255) * bg.r as u8;
            let g  = (bitmap[i] / 255) * fg.g + (1 - bitmap[i] / 255) * bg.g as u8;
            let b  = (bitmap[i] / 255) * fg.b + (1 - bitmap[i] / 255) * bg.b as u8;
            let c = Color::RGB(r, g, b);
           
            texture.push(c);
        }
        
        //panic!("{:?}", texture);

        AnkerLabel {
            x: x_,
            y: y_,
            z: z_,
            text: text_,
            size: 1.,
            color: fg,
            background_color: bg,
            font: font_,
            visible: true,
            texture: texture,
            texture_size: size,
        }
    }
}

impl UiElement for AnkerLabel {
    fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
        print!("{} {} {}", self.texture.len(), self.texture_size.0, self.texture_size.1);

        for i in 0..self.texture_size.0 {
            for j in 0..self.texture_size.1 {
                canvas.set_draw_color(self.texture[(i * self.texture_size.1 + j) as usize]);
                canvas.draw_point(sdl2::rect::Point::new(i as i32 + x, j as i32 + y)).unwrap();
            }
        }
    }
}

impl Transformable for AnkerLabel {
    fn translate(&mut self, p: V3) {
        self.x += p.x;
        self.y += p.y;
        self.z += p.z;
    }

    fn scale(&mut self, p: V3) {
        //not implemented
    }

    fn rot(&mut self, r: V3) {
        //not implemented
    }

    fn rot_reverse(&mut self, r: V3) {
        //not implemented
    }

    fn transform(&mut self) -> Box<&mut dyn Transformable> {
        Box::new(self)
    }
}