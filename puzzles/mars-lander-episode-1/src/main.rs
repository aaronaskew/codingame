#![allow(unused)]
use std::io;
macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

static GRAVITY: f64 = -3.711;
static mut TIME: u32 = 0;

#[derive(Debug)]
struct Lander {
    time: u32,
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
    ax: f64,
    ay: f64,
    fuel: i32,
    rotate: i32,
    prev_rotate: i32,
    power: i32,
    prev_power: i32,
}

impl Lander {
    fn get_commands(&self) -> String {
        // y = y_0 + vy * t + 1/2 * a * t**2
        // time (t) is assumed to be 1 (second) ::
        // y = y_0 + vy + ay/2

        // ay = power + GRAVITY

        // y = y_0 + vy + (power + GRAVITY)/2
        // we want vy=0 as we approach y of 0

        let cmd_rotation = 0;
        let cmd_thrust = match self.vy {
            0..=500 => 0,
            -10..=-1 => 1,
            -20..=-11 => 2,
            -30..=-21 => 3,
            _ => 4,
        };

        format!("{} {}", cmd_rotation, cmd_thrust).to_string()
    }
}

fn game_loop(lander: &mut Lander) {
    // game loop

    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let h_speed = parse_input!(inputs[2], i32); // the horizontal speed (in m/s), can be negative.
        let v_speed = parse_input!(inputs[3], i32); // the vertical speed (in m/s), can be negative.
        let fuel = parse_input!(inputs[4], i32); // the quantity of remaining fuel in liters.
        let rotate = parse_input!(inputs[5], i32); // the rotation angle in degrees (-90 to 90).
        let power = parse_input!(inputs[6], i32); // the thrust power (0 to 4).

        //update lander data from input
        lander.x = x;
        lander.y = y;
        lander.vx = h_speed;
        lander.vy = v_speed;
        lander.fuel = fuel;
        lander.rotate = rotate;
        lander.power = power;

        // calclulate next move
        println!("{}", lander.get_commands());

        eprintln!("t: {}, vy: {}", lander.time, lander.vy);

        // 2 integers: rotate power. rotate is the desired rotation angle (should be 0 for level 1), power is the desired thrust power (0 to 4).
        //println!("0 3");

        //increment time
        lander.time += 1;
    }
}

fn game_init() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let surface_n = parse_input!(input_line, i32);
    // the number of points used to draw the surface of Mars.
    for i in 0..surface_n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let land_x = parse_input!(inputs[0], i32); // X coordinate of a surface point. (0 to 6999)
        let land_y = parse_input!(inputs[1], i32); // Y coordinate of a surface point. By linking all the points together in a sequential fashion, you form the surface of Mars.
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut lander = Lander {
        x: 0,
        y: 0,
        vx: 0,
        vy: 0,
        fuel: 0,
        rotate: 0,
        prev_rotate: 0,
        power: 0,
        prev_power: 0,
        ax: 0.0,
        ay: 0.0,
        time: 0,
    };

    game_init();
    game_loop(&mut lander);
}
