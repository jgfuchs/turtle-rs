extern crate rand;
extern crate sdl2;

use std::collections::HashMap;

use rand::Rng;

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

struct Turtle {
    x: f32,
    y: f32,
    h: f32,
    wdim: (u32, u32),
    color: (u8, u8, u8),
    ops: Vec<TurtleOps>,
}

impl Turtle {
    fn new(width: u32, height: u32) -> Turtle {
        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;

        Turtle {
            x: cx as f32,
            y: cy,
            h: 0.0,
            wdim: (width, height),
            color: (255, 255, 255),
            ops: vec![TurtleOps::MoveTo(cx, cy)],
        }
    }

    fn forward(&mut self, dist: i32) {
        self.x += (dist as f32) * f32::cos(d2r(self.h));
        self.y += (dist as f32) * f32::sin(d2r(self.h));
        self.ops.push(TurtleOps::LineTo(self.x, self.y));
    }

    fn turn(&mut self, amt: f32) {
        self.h += amt;
    }

    fn move_to(&mut self, nx: i32, ny: i32) {
        self.x = nx as f32;
        self.y = ny as f32;
        self.ops.push(TurtleOps::MoveTo(self.x, self.y));
    }

    fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.color = (r, g, b);
        self.ops.push(TurtleOps::SetColor(r, g, b));
    }

    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_heading(&self) -> f32 {
        self.h
    }

    fn get_color(&self) -> (u8, u8, u8) {
        self.color
    }
}

fn turtle_draw_sdl(turtle: &Turtle) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rs-turtle", turtle.wdim.0, turtle.wdim.1)
        .position_centered()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();
    renderer.set_draw_color(Color::RGB(255, 255, 255));

    let mut playing = true;
    let mut complete = false;
    let mut op_iter = turtle.ops.iter();
    let mut x = 0i32;
    let mut y = 0i32;

    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();

    let delay = 20_000 / turtle.ops.len() as u32;
    println!("{}", delay);

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

fn main() {
    let mut rules = HashMap::new();
    rules.insert('A', "+B-A-B+");
    rules.insert('B', "-A+B+A-");
    let mut state = "A".to_string();
    for _ in 0..8 {
        state = l_system_step(state, &rules);
    }
    println!("{}", state.len());

    let mut rng = rand::thread_rng();

    let mut t = Turtle::new(600, 600);

    t.move_to(40, 500);
    t.set_color(rng.gen(), rng.gen(), rng.gen());
    for c in state.chars() {
        match c {
            'A' | 'B' => { t.forward(2); },
            '+' => { t.turn(-60.0); },
            '-' => { t.turn(60.0); },
            _ => {}
        }

        if rng.gen::<f32>() < 0.005 {
            t.set_color(rng.gen(), rng.gen(), rng.gen());
        }
    }
    turtle_draw_sdl(&t);
}


fn l_system_step(state: String, rules: &HashMap<char, &str>) -> String {
    let mut result = String::new();
    for c in state.chars() {
        if let Some(s) = rules.get(&c) {
            result.push_str(s);
        } else {
            result.push(c);
        }
    }
    result
}

