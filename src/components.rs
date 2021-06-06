// use crate::entity::Entity;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub trait Component {
    fn get_name(&self) -> &str;

    // fn set_owner(&self, entity: Entity);

    // fn get_owner(&self) -> Entity;

    fn init(&self);

    fn render(&self, canvas: &mut WindowCanvas);

    fn update(&mut self, delta_time: f32);
}

pub struct TransformComponent {
    pub scale: u32,
    pub width: u32,
    pub height: u32,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
}

impl Component for TransformComponent {
    fn get_name(&self) -> &str {
        return "TransformComponent";
    }

    fn init(&self) {}

    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .fill_rect(Rect::new(
                self.position.0 as i32,
                self.position.1 as i32,
                self.width * self.scale,
                self.height * self.scale,
            ))
            .expect("[error]: Could not fill rectangle!");
    }

    fn update(&mut self, delta_time: f32) {
        self.position.0 = self.position.0 + self.velocity.0 * delta_time;
        self.position.1 = self.position.1 + self.velocity.1 * delta_time;
    }
}
