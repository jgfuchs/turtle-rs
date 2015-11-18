extern crate turtle;

use turtle::Turtle;

fn koch(t: &mut Turtle, n: u32, d: f32) {
    if n == 0 {
        t.forward(d as i32);
    } else {
        koch(t, n - 1, d);
        t.turn(-60.0);
        koch(t, n - 1, d);
        t.turn(120.0);
        koch(t, n - 1, d);
        t.turn(-60.0);
        koch(t, n - 1, d);
    }
}

fn main() {
    let n = 3;
    let d = 500.0 / 1.2 / u32::pow(3, n) as f32;

    let mut t = Turtle::new();

    t.move_to(50, 370);

    t.turn(-60.0);
    koch(&mut t, n, d);
    t.turn(120.0);
    koch(&mut t, n, d);
    t.turn(120.0);
    koch(&mut t, n, d);

    t.draw_png().background(60, 60, 60).save("snowflake.png");
}
