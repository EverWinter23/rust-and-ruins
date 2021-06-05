extern crate sdl2;

use crate::timer::Timer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

pub struct Game {
    pub is_running: bool,
    timer: Option<Timer>,
    events: Option<EventPump>,
    canvas: Option<WindowCanvas>,
    player: (f32, f32),
}

impl Game {
    pub fn new() -> Game {
        Game {
            canvas: None,
            timer: None,
            events: None,
            is_running: false,
            player: (0.0, 0.0),
        }
    }

    pub fn init(&mut self, width: u32, height: u32) {
        let sdl_context = sdl2::init().expect("[error]: Could not intialize SDL!");

        self.events = Some(
            sdl_context
                .event_pump()
                .expect("[error]: Could not obtain EventPump for SDL!"),
        );

        let video_subsystem = sdl_context
            .video()
            .expect("[error]: Could not initialize video subsystem!");

        let window = video_subsystem
            .window("rover", width, height)
            .position_centered()
            .build()
            .expect("[error]: Could not create window!");

        self.canvas = Some(
            window
                .into_canvas()
                .build()
                .expect("[error]: Could not create a canvas from window!"),
        );

        self.timer = Some(Timer::new(
            sdl_context
                .timer()
                .expect("[error]: Could not intialize timer!"),
        ));

        self.is_running = true;
    }

    pub fn render(&mut self) {
        if let Some(canvas) = &mut self.canvas {
            canvas.set_draw_color(Color::RGB(30, 50, 80));
            canvas.clear();
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas
                .fill_rect(Rect::new(
                    self.player.0 as i32,
                    self.player.1 as i32,
                    10,
                    10,
                ))
                .expect("[error]: Could not fill rectangle!");
            canvas.present();
        }
    }

    pub fn update(&mut self) {
        if let Some(timer) = &mut self.timer {
            let (tick, delta) = timer.tick();

            if !tick {
                return;
            }

            self.player.0 += 30.0 * delta;
            self.player.1 += 30.0 * delta;
        }
    }

    pub fn process_input(&mut self) {
        if let Some(events) = &mut self.events {
            for event in events.poll_iter() {
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
}
