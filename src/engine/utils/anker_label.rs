use std::f32::MAX_EXP;

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
            x: 0.,
            y: 0.,
            max_width: Some(780.0),
            ..LayoutSettings::default()
        };
        //TODO own font parser ... haha...
        //refactor this

        let fonts = &[font_];
        layout.reset(&layout_settings);
        
        let c = Color::RGB(0, 0, 0);
        layout.append(fonts, &TextStyle::with_user_data("Hp", 20.0, 0, c));
        let glyps = layout.glyphs();
        let mut texture = Vec::new();
        let mut sub_texture: Vec<Vec<Color>> = Vec::with_capacity(glyps.len());

        let mut sub_texture_width: Vec<i32> = Vec::with_capacity(glyps.len());
        let mut sub_texture_height: Vec<i32> = Vec::with_capacity(glyps.len());
        let mut sub_texture_x: Vec<i32> = Vec::with_capacity(glyps.len());
        let mut sub_texture_y: Vec<i32> = Vec::with_capacity(glyps.len());

        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;

       

        for i in 0..glyps.len() {
            let g = glyps[i];
            let (metrics, bitmap) = fonts[0].rasterize_config(g.key);
            //glyph_positions.push((g.x as i32, g.y as i32));
            sub_texture_height.push(metrics.height as i32);
            sub_texture_width.push(metrics.width as i32);
            sub_texture_x.push(metrics.xmin + g.x as i32);
            sub_texture_y.push(metrics.ymin + g.y as i32);

            if (metrics.xmin + g.x as i32) < min_x {
                min_x = metrics.xmin + g.x as i32;
            }
            if (metrics.ymin + g.y as i32) < min_y {
                min_y = metrics.ymin + g.y as i32;
            }
            if (metrics.xmin + g.x as i32 + metrics.width as i32) > max_x {
                max_x = metrics.xmin + g.x as i32 + metrics.width as i32;
            }
            if (metrics.ymin + g.y as i32 + metrics.height as i32) > max_y {
                max_y = metrics.ymin + g.y as i32 + metrics.height  as i32;
            }
            sub_texture.push(Vec::new());
            for coverage in bitmap {
                let r  = ((coverage as f64 / 255.) * fg.r as f64 + (1. - coverage as f64 / 255.) * bg.r as f64) as u8;
                let g  = ((coverage as f64 / 255.) * fg.g as f64+ (1. - coverage as f64 / 255.) * bg.g as f64) as u8;
                let b  = ((coverage as f64 / 255.) * fg.b as f64+ (1. - coverage as f64 / 255.) * bg.b as f64) as u8;
                let c = Color::RGB(r, g, b);
                sub_texture[i].push(c);
            }

            print!("{:?}", g);
        }

        let is_represented = |x: i32, y: i32| -> i32 {
            for i in 0..sub_texture.len() {
                if x >= sub_texture_x[i] && x < sub_texture_x[i] + sub_texture_width[i] && y >= sub_texture_y[i] && y < sub_texture_y[i] + sub_texture_height[i] {
                    return i as i32;
                }
            }
            return -1;
        };
        
        for i in min_y..max_y {
            for j in min_x..max_x {
                let idx = is_represented(j + min_x, i + min_y);
                if idx != -1 {
                    let j2 = j - sub_texture_x[idx as usize];
                    let i2 = i - sub_texture_y[idx as usize];
                    texture.push(sub_texture[idx as usize][(i2 * sub_texture_width[idx as usize] + j2) as usize]);
                } else {
                    texture.push(bg);
                }
            }
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
            texture_size: ((max_x - min_x) as u32, (max_y - min_y) as u32),
        }
    }
}

impl UiElement for AnkerLabel {
    fn render(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
        print!("{} {} {}", self.texture.len(), self.texture_size.0, self.texture_size.1);
        let w = self.texture_size.0;
        for i in 0..self.texture_size.1 {
            for j in 0..self.texture_size.0 {
                canvas.set_draw_color(self.texture[(i * w + j) as usize]);
                canvas.draw_point(sdl2::rect::Point::new(j  as i32 + x, i  as i32 + y)).unwrap();
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