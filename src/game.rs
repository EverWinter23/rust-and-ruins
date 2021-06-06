extern crate sdl2;

use crate::components::TransformComponent;
use crate::managers::EntityManager;
use crate::timer::Timer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

pub struct Game {
    pub is_running: bool,
    timer: Timer,
    events: EventPump,
    canvas: WindowCanvas,
    en_manager: EntityManager,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let sdl_context = sdl2::init().expect("[error]: Could not intialize SDL!");

        let events = sdl_context.event_pump()
                        .expect("[error]: Could not obtain EventPump for SDL!");

        let video_subsystem = sdl_context
            .video()
            .expect("[error]: Could not initialize video subsystem!");

        let window = video_subsystem
            .window("rover", width, height)
            .position_centered()
            .build()
            .expect("[error]: Could not create window!");

        let canvas = window
                        .into_canvas()
                        .build()
                        .expect("[error]: Could not create a canvas from window!");

        let timer = Timer::new(
            sdl_context
                .timer()
                .expect("[error]: Could not intialize timer!"),
        );

        Game {
            canvas: canvas,
            timer: timer,
            events: events,
            is_running: true,
            en_manager: EntityManager {
                entities: Vec::new(),
            },
        }
    }

    pub fn init(&mut self) {
        self.load_level();
    }

    pub fn load_level(&mut self) {
        let entity1 = self.en_manager.add_entity("rect".to_string());
        entity1.add_component::<TransformComponent>(TransformComponent {
            scale: 3,
            width: 10,
            height: 10,
            position: (50.0, 50.0),
            velocity: (10.0, 10.0),
        });

        let entity2 = self.en_manager.add_entity("rect".to_string());
        entity2.add_component::<TransformComponent>(TransformComponent {
            scale: 1,
            width: 15,
            height: 15,
            position: (0.0, 0.0),
            velocity: (30.0, 30.0),
        });

        for entity in &mut self.en_manager.entities {
            println!("Entity: {}", entity.name);
            entity.list_all_components();
            println!("");
        }

    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(30, 50, 80));
        self.canvas.clear();
        self.en_manager.render(&mut self.canvas);
        self.canvas.present();
    }

    pub fn update(&mut self) {
        let (tick, delta) = self.timer.tick();

        if !tick || self.en_manager.get_entity_count() == 0 {
            return;
        }

        self.en_manager.update(delta);
    }

    pub fn process_input(&mut self) {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.is_running = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.is_running = false;
                }
                _ => {}
            }
        }
    }
}
