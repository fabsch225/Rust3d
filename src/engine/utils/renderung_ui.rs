use sdl2::{render::Canvas, video::Window};

pub trait UiElement {
    fn render(&self, canvas: &mut Canvas<Window>);
}
