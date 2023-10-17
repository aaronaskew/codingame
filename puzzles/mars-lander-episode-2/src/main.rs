#![allow(unused)]
use std::{borrow::BorrowMut, io};
macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

static ACCELERATION_DUE_TO_GRAVITY: f64 = -3.711;

#[derive(Debug)]
struct Burn {
    power: i32,
    rotate: i32,
    duration: i32,
    // don't think i need this at the moment
    // time_remaining: i32,
}

impl Burn {
    fn new() -> Self {
        Self {
            power: 0,
            rotate: 0,
            duration: 0,
        }
    }
}

#[derive(Debug)]
struct Lander {
    time: u32,
    x: i32,  // from MC
    y: i32,  // from MC
    vx: i32, // from MC
    vy: i32, // from MC
    ax: f64,
    ay: f64,
    angle: i32, // from MC
    power: i32, // from MC
    fuel: i32,  // from MC
    target_x: i32,
    target_y: i32,

    //burn_queue: Vec<Burn>,
    active_burn: Burn,
}

impl Lander {
    fn get_commands(&mut self) -> String {
        let rotate_cmd = self.active_burn.rotate;
        let power_cmd = self.active_burn.power;

        format!("{} {}", rotate_cmd, power_cmd).to_string()
    }

    ///
    ///
    /// Do the physics calculations every tick and set the commands for the
    /// next turn.
    fn physics_update(&mut self) {
        //update acceleration
        self.ax = Self::pow_rot_to_accel_x(self.power, self.angle);
        self.ay = Self::pow_rot_to_accel_y(self.power, self.angle) + ACCELERATION_DUE_TO_GRAVITY;

        let mut burn = Burn::new();

        // focus solely on the x-dimension for now

        // get the needed acceleration
        let a = Self::accel(self.target_x, self.x, self.vx, 30);
        eprintln!("accel needed: {}", a);

        match (self.target_x - self.x) {
            1.. => {
                //go right
                burn.rotate = -90;
                burn.power = 4;
            }
            0 => {
                //stay put
                burn.rotate = 0;
                burn.power = 0;
            }
            ..=-1 => {
                //go left
                burn.rotate = 90;
                burn.power = 4;
            }
        }

        // Determine the time needed to do this burn
        let duration =
            Self::time_of_burn_for_x(self.vx, Self::pow_rot_to_accel_x(burn.power, burn.rotate));

        burn.duration = duration;

        self.active_burn = burn;

        //eprintln!("calculated ax: {}", ax);
    }

    /// Take the `surface` data and use that to set the target location
    /// find target such that the landing surface is flat (delta_y between two points is 0)
    /// and is the closest possible point that is on one of the flat surfaces
    ///
    /// Assumptions:
    ///    - the vx of the lander is or near 0. If vx is not near 0, then the lander
    ///     should account for the projected position of the lander when determining
    ///     the distance function
    fn determine_target(&mut self, surface_data: &mut [(i32, i32)]) {
        assert!(self.x != -1 && self.y != -1);
        assert!(self.target_x == -1 && self.target_y == -1);

        //eprintln!("entering determine_target()");

        let mut flat_surface_points: Vec<(i32, i32)> = Vec::new();

        flat_surface_points.append(
            &mut surface_data
                // Gets pairs at a time from surface
                .windows(2)
                // Ensures that the iter will only contain points of flat surface
                // by checking that the delta_y between two points is 0
                .filter(|window| window[0].1 == window[1].1)
                // Converts the iter of pairs to an iter of points
                .flat_map(|window| vec![window[0], window[1]])
                // Collects the iter of points into a Vec
                .collect::<Vec<(i32, i32)>>(),
        );

        //eprintln!("flat_surface_points: {:#?}", flat_surface_points);

        assert!(flat_surface_points.len() == 2);

        // find the closest point to the lander
        // 1) check if the lander is directly above a flat surface point
        // 2) if not, find the closest point to the lander
        // 3) set the target

        if self.y > flat_surface_points[0].1
            && self.x >= flat_surface_points[0].0
            && self.x <= flat_surface_points[1].0
        {
            assert!(self.y > flat_surface_points[0].1);
            assert!(self.x <= flat_surface_points[1].0);
            assert!(self.x >= flat_surface_points[0].0);

            // the lander is above the flat surface
            // target is (self.x, flat_surface_point.y)
            self.target_x = self.x;
            self.target_y = flat_surface_points[0].1;
        } else {
            // we know the target y, get the closest target x
            flat_surface_points
                .sort_by(|(ax, _), (bx, _)| (self.x - *ax).abs().cmp(&(self.x - *bx).abs()));

            self.target_x = flat_surface_points[0].0;
            self.target_y = flat_surface_points[0].1;
        }
    }

    fn pos(pos_0: f64, v_0: f64, a: f64, t: f64) -> f64 {
        pos_0 + v_0 * t + 0.5 * a * t * t
    }

    fn distance(x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
        ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt().abs()
    }

    fn time_of_burn_for_x(v: i32, a: f64) -> i32 {
        // t = -(2 v0)/a and a!=0
        // t = 0

        //convert ints to floats for clarity
        let v = v as f64;

        // if the burn accel is 0, burn for zero seconds
        if a == 0.0 {
            return 0;
        }
        //no div by 0!
        assert!(a != 0.0);

        let time = -2.0 * v / a;

        eprintln!(
            "time_of_burn_for_x\n\tf64: {}\n\tf64.clamp: {}\n\tf64.round: {}\n\ti32: {}",
            time,
            time.clamp(0.0, f64::MAX),
            time.clamp(0.0, f64::MAX).round(),
            time.clamp(0.0, f64::MAX).round() as i32
        );

        time.clamp(0.0, f64::MAX).round() as i32
    }

    /// .
    fn pow_rot_to_accel_x(power: i32, angle: i32) -> f64 {
        power as f64 * (angle as f64 + 180.0).to_radians().sin()
    }
    /// .
    fn pow_rot_to_accel_y(power: i32, angle: i32) -> f64 {
        power as f64 * (angle as f64 + 180.0).to_radians().cos()
    }

    fn accel(x: i32, x0: i32, vx0: i32, t: i32) -> f64 {
        let x = x as f64;
        let x0 = x0 as f64;
        let vx0 = vx0 as f64;
        let t = t as f64;

        let mut acceleration = match t {
            t_test if t_test > 0.0 => 2.0 * (x - x0 - vx0 * t) / (t * t),
            _ => 0.0,
        };

        // clamp it to our allowed thrust
        acceleration = acceleration.clamp(-4.0, 4.0);

        acceleration
    }
}

fn read_lander_data(lander: &mut Lander) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(' ').collect::<Vec<_>>();
    let x = parse_input!(inputs[0], i32);
    let y = parse_input!(inputs[1], i32);
    let h_speed = parse_input!(inputs[2], i32);
    // the horizontal speed (in m/s), can be negative.
    let v_speed = parse_input!(inputs[3], i32);
    // the vertical speed (in m/s), can be negative.
    let fuel = parse_input!(inputs[4], i32);
    // the quantity of remaining fuel in liters.
    let rotate = parse_input!(inputs[5], i32);
    // the rotation angle in degrees (-90 to 90).
    let power = parse_input!(inputs[6], i32);
    // the thrust power (0 to 4).

    //update lander data from input
    lander.x = x;
    lander.y = y;
    lander.vx = h_speed;
    lander.vy = v_speed;
    lander.fuel = fuel;
    lander.angle = rotate;
    lander.power = power;
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut lander = Lander {
        x: -1,
        y: -1,
        vx: 0,
        vy: 0,
        fuel: -1,
        power: -1,
        ax: 0.0,
        ay: 0.0,
        time: 0,
        target_x: -1,
        target_y: -1,
        angle: 0,
        //burn_queue: vec![],
        active_burn: Burn::new(),
    };

    let mut surface_data: Vec<(i32, i32)> = vec![];

    game_init(&lander, &mut surface_data);
    game_loop(&mut lander, &mut surface_data);
}

fn game_init(lander: &Lander, surface_data: &mut Vec<(i32, i32)>) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let surface_n = parse_input!(input_line, i32);

    *surface_data = vec![(-1, -1); surface_n as usize];

    // the number of points used to draw the surface of Mars.
    #[allow(clippy::needless_range_loop)]
    for i in 0..surface_n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let land_x = parse_input!(inputs[0], i32); // X coordinate of a surface point. (0 to 6999)
        let land_y = parse_input!(inputs[1], i32); // Y coordinate of a surface point. By linking all the points together in a sequential fashion, you form the surface of Mars.
        (surface_data[i].0, surface_data[i].1) = (land_x, land_y);
    }

    // eprintln!("surface_data: {:#?}", surface_data);
}

fn game_loop(lander: &mut Lander, surface_data: &mut [(i32, i32)]) {
    // game loop

    loop {
        read_lander_data(lander);

        if lander.target_x == -1 && lander.target_y == -1 {
            lander.determine_target(surface_data);
        }

        lander.physics_update();

        // get commands
        println!("{}", lander.get_commands());

        eprintln!("lander: {:#?}", lander);

        //increment time
        lander.time += 1;
    }
}
