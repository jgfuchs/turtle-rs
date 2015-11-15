extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

#[derive(Debug)]
enum TurtleOp {
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
    ops: Vec<TurtleOp>,
}

impl Turtle {
    pub fn new() -> Turtle {
        Turtle {
            x: 0.0,
            y: 0.0,
            h: 0.0,
            ops: Vec::new(),
        }
    }

    pub fn forward(&mut self, dist: i32) {
        self.x += (dist as f32) * f32::cos(d2r(self.h));
        self.y += (dist as f32) * f32::sin(d2r(self.h));
        self.ops.push(TurtleOp::LineTo(self.x, self.y));
    }

    pub fn turn(&mut self, amt: f32) {
        self.h += amt;
    }

    pub fn move_to(&mut self, nx: i32, ny: i32) {
        self.x = nx as f32;
        self.y = ny as f32;
        self.ops.push(TurtleOp::MoveTo(self.x, self.y));
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.ops.push(TurtleOp::SetColor(r, g, b));
    }

    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn get_heading(&self) -> f32 {
        self.h
    }

    pub fn lines(&self) -> Lines {
        Lines {
            i: self.ops.iter(),
            x: 0,
            y: 0,
            color: (255, 255, 255),
        }
    }

    pub fn draw_sdl(&self) -> SdlTurtle {
        SdlTurtle::new(&self)
    }
}

pub struct Lines<'a> {
    i: std::slice::Iter<'a, TurtleOp>,
    x: i32,
    y: i32,
    color: (u8, u8, u8),
}

pub struct Line {
    start: (i32, i32),
    end: (i32, i32),
    color: Option<(u8, u8, u8)>,
}

impl<'a> Iterator for Lines<'a> {
    type Item = Line;

    fn next(&mut self) -> Option<Line> {
        let mut colorchanged = false;
        loop {
            match self.i.next() {
                Some(&TurtleOp::MoveTo(tx, ty)) => {
                    self.x = tx as i32;
                    self.y = ty as i32;
                }
                Some(&TurtleOp::LineTo(tx, ty)) => {
                    let lastx = self.x;
                    let lasty = self.y;

                    self.x = tx as i32;
                    self.y = ty as i32;

                    return Some(Line {
                        start: (lastx, lasty),
                        end: (self.x, self.y),
                        color: if colorchanged {
                            Some(self.color)
                        } else {
                            None
                        },
                    });
                }
                Some(&TurtleOp::SetColor(r, g, b)) => {
                    self.color = (r, g, b);
                    colorchanged = true;
                }
                None => {
                    return None
                }
            }
        }
    }
}

pub struct SdlTurtle<'a> {
    title: String,
    dims: (u32, u32),
    interactive: bool,
    speed: f32,
    turtle: &'a Turtle,
}

impl<'a> SdlTurtle<'a> {
    fn new(turtle: &Turtle) -> SdlTurtle {
        SdlTurtle {
            title: "turtle-rs".to_string(),
            dims: (512, 512),
            interactive: true,
            speed: 60.0,
            turtle: turtle,
        }
    }

    pub fn title(&'a mut self, new_title: &str) -> &mut SdlTurtle  {
        self.title = new_title.to_string();
        self
    }

    pub fn size(&'a mut self, width: u32, height: u32) -> &mut SdlTurtle {
        self.dims = (width, height);
        self
    }

    pub fn interactive(&'a mut self, inter: bool) -> &mut SdlTurtle {
        self.interactive = inter;
        self
    }

    pub fn speed(&'a mut self, new_speed: f32) -> &mut SdlTurtle {
        self.speed = new_speed;
        self
    }

    pub fn show(&self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window(&self.title, self.dims.0, self.dims.1)
                                   .position_centered()
                                   .build()
                                   .unwrap();

        let mut renderer = window.renderer().build().unwrap();
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.set_draw_color(Color::RGB(255, 255, 255));

        let mut paused = false;
        let mut step = false;
        let mut line_iter = self.turtle.lines();
        let mut delay = (1000.0 / self.speed) as u32;

        let mut event_pump = sdl_context.event_pump().unwrap();

        loop {
           if !paused || step {
               step = false;
               if let Some(line) = line_iter.next() {
                   if let Some((r, g, b)) = line.color {
                       renderer.set_draw_color(Color::RGB(r, g, b));
                   }

                   renderer.draw_line(Point::new(line.start.0, line.start.1),
                                      Point::new(line.end.0, line.end.1));

                   renderer.present();
               } else {
                   paused = true;
               }
           }

           for event in event_pump.poll_iter() {
               match event {
                   Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                       return;
                   }
                   Event::KeyDown { keycode: Some(keycode), .. } => {
                       if !self.interactive {
                           continue;
                       }

                       match keycode {
                           Keycode::Space => {
                               paused = !paused;
                           }
                           Keycode::R => {
                               paused = false;
                               line_iter = self.turtle.lines();
                               renderer.set_draw_color(Color::RGB(0, 0, 0));
                               renderer.clear();
                               renderer.set_draw_color(Color::RGB(255, 255, 255));
                           }
                           Keycode::S => {
                               step = true;
                           }
                           Keycode::LeftBracket => {
                               delay += 1;
                           }
                           Keycode::RightBracket => {
                               if delay > 0 {
                                   delay -= 1;
                               }
                           }
                           _ => {}
                       }
                   }
                   _ => {}
               }
           }

           std::thread::sleep_ms(delay);
        }
    }
}
