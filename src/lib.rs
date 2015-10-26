extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

#[derive(Debug)]
enum TurtleOps {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    SetColor(u8, u8, u8),
}

fn d2r(r: f32) -> f32 {
    r * std::f32::consts::PI / 180.0
}

pub struct Turtle {
    x: f32,
    y: f32,
    h: f32,
    color: (u8, u8, u8),
    ops: Vec<TurtleOps>,
}

impl Turtle {
    pub fn new() -> Turtle {
        Turtle {
            x: 0.0,
            y: 0.0,
            h: 0.0,
            color: (255, 255, 255),
            ops: vec![TurtleOps::MoveTo(0.0, 0.0)],
        }
    }

    pub fn forward(&mut self, dist: i32) {
        self.x += (dist as f32) * f32::cos(d2r(self.h));
        self.y += (dist as f32) * f32::sin(d2r(self.h));
        self.ops.push(TurtleOps::LineTo(self.x, self.y));
    }

    pub fn turn(&mut self, amt: f32) {
        self.h += amt;
    }

    pub fn move_to(&mut self, nx: i32, ny: i32) {
        self.x = nx as f32;
        self.y = ny as f32;
        self.ops.push(TurtleOps::MoveTo(self.x, self.y));
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.color = (r, g, b);
        self.ops.push(TurtleOps::SetColor(r, g, b));
    }

    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn get_heading(&self) -> f32 {
        self.h
    }

    pub fn get_color(&self) -> (u8, u8, u8) {
        self.color
    }

    pub fn draw_sdl(&self, delay: u32, wdim: (u32, u32)) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("rs-turtle", wdim.0, wdim.1)
            .position_centered()
            .build()
            .unwrap();

        let mut renderer = window.renderer().build().unwrap();
        renderer.clear();
        renderer.set_draw_color(Color::RGB(255, 255, 255));

        let mut playing = true;
        let mut complete = false;
        let mut op_iter = self.ops.iter();
        let mut x = 0i32;
        let mut y = 0i32;

        let mut running = true;
        let mut event_pump = sdl_context.event_pump().unwrap();

        while running {
            if !complete && playing {
                if let Some(op) = op_iter.next() {
                    match *op {
                        TurtleOps::MoveTo(tx, ty) => {
                            x = tx as i32;
                            y = ty as i32;
                        },
                        TurtleOps::LineTo(tx, ty) => {
                            renderer.draw_line(Point::new(x, y), Point::new(tx as i32, ty as i32));
                            x = tx as i32;
                            y = ty as i32;
                        }
                        TurtleOps::SetColor(r, g, b) => {
                            renderer.set_draw_color(Color::RGB(r, g, b));
                        }
                    }

                    renderer.present();
                } else {
                    complete = true;
                }
            }

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        running = false
                    },
                    Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                        playing = !playing;
                    }
                    _ => {}

                }
            }

            std::thread::sleep_ms(delay);
        }
    }
}
