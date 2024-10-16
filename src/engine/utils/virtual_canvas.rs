use sdl2::{rect::Point, render::Canvas, video::Window};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

pub struct VirtualCanvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl VirtualCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        VirtualCanvas {
            width,
            height,
            pixels: vec![0; width * height * 4]
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        let pos = (x + y * (self.width)) * 4;
        self.pixels[pos + 0] = color.r;
        self.pixels[pos + 1] = color.g;
        self.pixels[pos + 2] = color.b;
        self.pixels[pos + 3] = color.a;
    }

    pub fn get_sdl2_color(&self, x: usize, y: usize) -> sdl2::pixels::Color {
        let pos = (x + y * (self.width)) * 4;
        sdl2::pixels::Color::RGBA(
            self.pixels[pos + 0],
            self.pixels[pos + 1],
            self.pixels[pos + 2],
            self.pixels[pos + 3]
        )
    }
}