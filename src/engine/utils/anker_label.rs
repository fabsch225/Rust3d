use fontdue::{layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle}, Font};
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
    pub visible: bool,
    pub texture_size: (u32, u32),
    pub texture: Vec<Color>,
}

impl AnkerLabel {
    pub fn new(x_: f64, y_: f64, z_: f64, text_: String, font_ : Font, bg: Color, fg: Color) -> Self {
        let mut layout: Layout<Color> = Layout::new(CoordinateSystem::PositiveYDown);
        let mut layout_settings = LayoutSettings {
            x: 10.0,
            y: 10.0,
            max_width: Some(780.0),
            ..LayoutSettings::default()
        };
        let fonts = &[font_];
        layout.reset(&layout_settings);
        
        let c = Color::RGB(0, 0, 0);
        layout.append(fonts, &TextStyle::with_user_data("H", 180.0, 0, c));
        
        let mut texture = Vec::new();
        let glyps = layout.glyphs();
        let fg = Color::RED;
        let bg = Color::GREEN;
        let mut metrics_ = fonts[0].metrics('H', 180.0);
        for g in glyps {
            let (metrics, bitmap) = fonts[0].rasterize_config(g.key);
            metrics_ = metrics;
            let size = (g.width, g.height);
          
            let pos = (g.x, g.y);

            for coverage in bitmap {
                let r  = (coverage / 255) * fg.r + (1 - coverage / 255) * bg.r as u8;
                let g  = (coverage / 255) * fg.g + (1 - coverage / 255) * bg.g as u8;
                let b  = (coverage / 255) * fg.b + (1 - coverage / 255) * bg.b as u8;
                let c = Color::RGB(r, g, b);
                texture.push(c);
            }

            print!("{:?}", g);
        }
        
        
        
        AnkerLabel {
            x: x_,
            y: y_,
            z: z_,
            text: text_,
            size: 1.,
            color: fg,
            background_color: bg,
            visible: true,
            texture: texture,
            texture_size: (metrics_.width as u32, metrics_.height as u32),
        }
    }
}

impl UiElement for AnkerLabel {
    fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
        print!("{} {} {}", self.texture.len(), self.texture_size.0, self.texture_size.1);
        let w = self.texture_size.0;
        let t: &[Color] = &*self.texture;
        for i in 0..self.texture_size.1 {
            for j in 0..self.texture_size.0 {
                canvas.set_draw_color(self.texture[(i * w + j) as usize]);
                canvas.draw_point(sdl2::rect::Point::new(j as i32 + x, i as i32 + y)).unwrap();
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