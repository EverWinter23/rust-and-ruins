use crate::components::Component;

use sdl2::render::WindowCanvas;

pub struct Entity {
    is_active: bool,
    components: Vec<Box<dyn Component>>,

    pub name: String,
}

impl Entity {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            is_active: true,
            components: Vec::new(),
        }
    }

    pub fn add_component<T: 'static + Component>(&mut self, component: T) {
        component.init();
        self.components.push(Box::new(component));
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.is_active == false {
            return;
        }

        for component in &mut self.components {
            component.update(delta_time);
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        if self.is_active == false {
            return;
        }

        for component in &self.components {
            component.render(canvas);
        }
    }

    pub fn list_all_components(&mut self) {
        for component in &mut self.components {
            println!("Component<{}>", component.get_name());
        }
    }
}
