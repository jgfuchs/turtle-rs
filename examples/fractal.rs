extern crate turtle;

use std::collections::HashMap;
use turtle::Turtle;

fn main() {
    let mut rules = HashMap::new();
    rules.insert('A', "+B-A-B+");
    rules.insert('B', "-A+B+A-");
    let mut state = "A".to_string();
    for _ in 0..8 {
        state = l_system_step(state, &rules);
    }

    let mut t = Turtle::new(600, 600);
    t.move_to(40, 500);
    t.set_color(255, 255, 255);
    for c in state.chars() {
        match c {
            'A' | 'B' => { t.forward(2); },
            '+' => { t.turn(-60.0); },
            '-' => { t.turn(60.0); },
            _ => {}
        }
    }
    t.draw_sdl(5);
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
