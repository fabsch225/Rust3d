use sdl2::{pixels::Color, render::{Canvas, Texture}, video::Window};

use crate::engine::utils::{rendering::{RenderObjects, Renderable}, transformation::Transformable};
use crate::geometry::point::Point as V3;

pub struct AnkerLabel {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub text: String,
    pub size: i32,
    pub color: Color,
    pub background_color: Color,
    pub font: String,
    pub visible: bool,
    pub texture_size: (u32, u32),
    pub texture: Vec<Color>,
}

impl AnkerLabel {
    pub fn new(x_: f64, y_: f64, z_: f64, text_: String, font_ : String, canvas_ : &Canvas<Window>) -> Self {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let mut font: sdl2::ttf::Font<'_, 'static> = ttf_context.load_font(font_, 16).unwrap();
        font.set_style(sdl2::ttf::FontStyle::BOLD);
        
        // render a surface, and convert it to a texture bound to the canvas
        let surface: sdl2::surface::Surface<'_> = font
            .render("Hello Rust!")
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e| e.to_string()).unwrap();
        
        let size = surface.size();
        let x = surface.without_lock().unwrap();

        let mut texture = Vec::new();

        for mut i in 0..x.len() - 2 {
            texture.push(Color::RGB(x[i], x[i + 1], x[i + 2]));
            i += 2;
        }

        //todo: refactor this

        AnkerLabel {
            x: x_,
            y: y_,
            z: z_,
            text: text_,
            size: 16,
            color: Color::BLACK,
            background_color: Color::WHITE,
            font: "assets/fonts/Roboto-Regular.ttf".to_string(),
            visible: true,
            texture: texture,
            texture_size: size,
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