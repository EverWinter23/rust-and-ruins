use crate::entity::Entity;

use sdl2::render::WindowCanvas;

pub struct EntityManager {
    pub entities: Vec<Entity>,
}

impl EntityManager {
    pub fn update(&mut self, delta_time: f32) {
        for entity in &mut self.entities {
            entity.update(delta_time);
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        for entity in &self.entities {
            entity.render(canvas);
        }
    }

    pub fn add_entity(&mut self, name: String) -> &mut Entity {
        let entity = Entity::new(name);
        self.entities.push(entity);
        self.entities.last_mut().unwrap()
    }

    pub fn get_entity_count(&self) -> usize {
        self.entities.len()
    }
}
