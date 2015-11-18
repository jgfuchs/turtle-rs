extern crate sdl2;
extern crate image;

use std::fs::File;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use image::{RgbImage, Rgb, Pixel};

pub struct Turtle {
    x: f32,
    y: f32,
    h: f32,
    ops: Vec<TurtleOp>,
}

enum TurtleOp {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    SetColor(u8, u8, u8),
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
        let d2r = |deg| deg * std::f32::consts::PI / 180.0;
        self.x += (dist as f32) * f32::cos(d2r(self.h));
        self.y += (dist as f32) * f32::sin(d2r(self.h));
        self.ops.push(TurtleOp::LineTo(self.x, self.y));
    }

    pub fn turn(&mut self, degrees: f32) {
        self.h += degrees;
    }

    pub fn move_to(&mut self, nx: i32, ny: i32) {
        self.x = nx as f32;
        self.y = ny as f32;
        self.ops.push(TurtleOp::MoveTo(self.x, self.y));
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.ops.push(TurtleOp::SetColor(r, g, b));
    }

    pub fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn heading(&self) -> f32 {
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

    pub fn draw_png(&self) -> PngTurtle {
        PngTurtle::new(&self)
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
    color: (u8, u8, u8),
}

impl<'a> Iterator for Lines<'a> {
    type Item = Line;

    fn next(&mut self) -> Option<Line> {
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
                        color: self.color,
                    });
                }
                Some(&TurtleOp::SetColor(r, g, b)) => {
                    self.color = (r, g, b);
                }
                None => {
                    return None;
                }
            }
        }
    }
}

pub struct PngTurtle<'a> {
    size: (u32, u32),
    antialias: bool,
    bg: (u8, u8, u8),
    turtle: &'a Turtle,
}

impl<'a> PngTurtle<'a> {
    fn new(turtle: &Turtle) -> PngTurtle {
        PngTurtle {
            size: (500, 500),
            antialias: false,
            bg: (0, 0, 0),
            turtle: &turtle,
        }
    }

    pub fn size(&'a mut self, width: u32, height: u32) -> &mut PngTurtle {
        self.size = (width, height);
        self
    }

    pub fn antialias(&'a mut self, aa: bool) -> &mut PngTurtle {
        self.antialias = aa;
        self
    }

    pub fn background(&'a mut self, r: u8, g: u8, b: u8) -> &mut PngTurtle {
        self.bg = (r, g, b);
        self
    }

    pub fn save(&'a self, fname: &str) {
        let bgpix = Rgb::from_channels(self.bg.0, self.bg.1, self.bg.2, 0);
        let mut img = RgbImage::from_pixel(self.size.0, self.size.1, bgpix);
        for line in self.turtle.lines() {
            draw_line_img(&mut img, line)
        }

        let ref mut fout = File::create(fname).unwrap();
        image::ImageRgb8(img).save(fout, image::PNG).unwrap();
    }
}

fn draw_line_img(img: &mut RgbImage, line: Line) {
    let w = img.width();
    let h = img.height();
    let inbounds = |x, y| x >= 0 && y >= 0 && x < w as i32 && y < h as i32;

    let (x0, y0) = (line.start.0, line.start.1);
    let (x1, y1) = (line.end.0, line.end.1);

    let dx = i32::abs(x1 - x0);
    let dy = -i32::abs(y1 - y0);
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let px = Rgb::from_channels(line.color.0, line.color.1, line.color.2, 0);

    let mut x = x0;
    let mut y = y0;

    loop {
        if !inbounds(x, y) {
            break;
        }

        img.put_pixel(x as u32, y as u32, px);

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2*err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

pub struct SdlTurtle<'a> {
    title: String,
    size: (u32, u32),
    interactive: bool,
    speed: f32,
    bg: (u8, u8, u8),
    turtle: &'a Turtle,
}

impl<'a> SdlTurtle<'a> {
    fn new(turtle: &Turtle) -> SdlTurtle {
        SdlTurtle {
            title: "turtle-rs".to_string(),
            size: (500, 500),
            interactive: true,
            speed: 60.0,
            bg: (0, 0, 0),
            turtle: turtle,
        }
    }

    pub fn title(&'a mut self, new_title: &str) -> &mut SdlTurtle {
        self.title = new_title.to_string();
        self
    }

    pub fn size(&'a mut self, width: u32, height: u32) -> &mut SdlTurtle {
        self.size = (width, height);
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

    pub fn background(&'a mut self, r: u8, g: u8, b: u8) -> &mut SdlTurtle {
        self.bg = (r, g, b);
        self
    }

    pub fn show(&self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window(&self.title, self.size.0, self.size.1)
                                    .position_centered()
                                    .build()
                                    .unwrap();

        let mut renderer = window.renderer().build().unwrap();

        let bgcolor = Color::RGB(self.bg.0, self.bg.1, self.bg.2);

        renderer.set_draw_color(bgcolor);
        renderer.clear();

        let mut paused = false;
        let mut step = false;
        let mut line_iter = self.turtle.lines();
        let mut delay = (1000.0 / self.speed) as u32;

        let mut event_pump = sdl_context.event_pump().unwrap();

        loop {
            if !paused || step {
                step = false;
                if let Some(line) = line_iter.next() {
                    renderer.set_draw_color(Color::RGB(line.color.0, line.color.1, line.color.2));
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
                                renderer.set_draw_color(bgcolor);
                                renderer.clear();
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
