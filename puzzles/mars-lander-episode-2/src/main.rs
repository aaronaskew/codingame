#![allow(unused)]
use std::f64::consts::PI;
use std::io;
use vec2::*;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

static ACCELERATION_DUE_TO_GRAVITY: Vec2 = Vec2 { x: 0.0, y: -3.711 };

mod vec2 {
    use std::f64::consts::PI;
    use std::ops::{Add, Div, Mul, Sub};

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct Vec2 {
        pub x: f64,
        pub y: f64,
    }
    impl Vec2 {
        pub fn new(x: f64, y: f64) -> Self {
            Vec2 { x, y }
        }

        pub fn magnitude(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }

        pub fn direction(&self) -> i32 {
            let radians = self.y.atan2(self.x);
            let degrees = radians * (180.0 / PI);
            degrees.round() as i32
        }

        pub fn normalized(&self) -> Self {
            *self / self.magnitude()
        }

        pub fn to_polar(self) -> (f64, f64) {
            let r = (self.x * self.x + self.y * self.y).sqrt();
            let theta = self.y.atan2(self.x) * (180.0 / PI); // Convert radians to degrees
            (r, theta)
        }

        pub fn clamp_magnitude(&mut self, min: f64, max: f64) {
            let mag = self.magnitude();

            if mag < min {
                let normalized = self.normalized();
                self.x = normalized.x * min;
                self.y = normalized.y * min;
            } else if mag > max {
                let normalized = self.normalized();
                self.x = normalized.x * max;
                self.y = normalized.y * max;
            }
        }

        pub fn rotate(&self, angle_degrees: f64) -> Vec2 {
            let theta = angle_degrees * PI / 180.0; // Convert to radians
            let cos_theta = theta.cos();
            let sin_theta = theta.sin();

            Vec2 {
                x: self.x * cos_theta - self.y * sin_theta,
                y: self.x * sin_theta + self.y * cos_theta,
            }
        }
    }
    impl Add for Vec2 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Vec2 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    impl Sub for Vec2 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Vec2 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }
    impl Mul<f64> for Vec2 {
        type Output = Self;

        fn mul(self, rhs: f64) -> Self::Output {
            Vec2 {
                x: rhs * self.x,
                y: rhs * self.y,
            }
        }
    }
    impl Mul<Vec2> for f64 {
        type Output = Vec2;

        fn mul(self, rhs: Vec2) -> Self::Output {
            Vec2 {
                x: self * rhs.x,
                y: self * rhs.y,
            }
        }
    }
    impl Div<f64> for Vec2 {
        type Output = Self;

        fn div(self, rhs: f64) -> Self::Output {
            Vec2 {
                x: self.x / rhs,
                y: self.y / rhs,
            }
        }
    }
}

#[derive(Debug)]
struct Surface {
    data: Vec<(i32, i32)>,
}

impl Surface {
    fn new() -> Self {
        Self { data: Vec::new() }
    }
}

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

    /// Transform the `rotation` (in degrees) from worldspace cartesion
    /// coordinate system to the thrust polar coordinate system by subtracting 90
    /// degrees.
    fn world_to_thrust_rot(rotation: i32) -> i32 {
        rotation - 90
    }

    /// Transform the `rotation` (in degrees) from worldspace cartesion
    /// coordinate system to the thrust polar coordinate system by adding
    /// 90 degrees.
    fn thrust_to_world_rot(rotation: i32) -> i32 {
        rotation + 90
    }
}

/// # Members
/// - `time`: mission time
/// - `position`: Vec2 (world-space)
/// - `rotation`: f64 (world-space)
/// - `velocity`: Vec2 (world-space)
/// - `acceleration`: Vec2 (world-space)
/// - `thrust_angle`: i32 (thrust-space)
/// - `thrust_power`: i32
/// - `fuel`: i32
/// - `target_position`: Vec2 (world-space)
/// - `active_burn`: Burn
#[derive(Debug)]
struct Lander {
    time: u32,
    position: Vec2, // from MC
    rotation: f64,
    velocity: Vec2, // from MC
    acceleration: Vec2,
    thrust_angle: i32, // from MC
    thrust_power: i32, // from MC
    fuel: i32,         // from MC
    target_position: Vec2,
    active_burn: Burn,
    surface: Surface,
}

impl Lander {
    fn get_commands(&mut self) -> String {
        let rotate_cmd = self.active_burn.rotate;
        let power_cmd = self.active_burn.power;

        format!("{} {}", rotate_cmd, power_cmd).to_string()
    }

    /// Do the physics calculations every tick and set the commands for the
    /// next turn.
    fn physics_update(&mut self) {

        // set acceleration
        //   acceleration due to thrust + gravity

        self.acceleration = {
            Vec2::new(
                self.thrust_power as f64
                    * (Burn::thrust_to_world_rot(self.thrust_angle) as f64)
                        .to_radians()
                        .sin(),
                self.thrust_power as f64
                    * (Burn::thrust_to_world_rot(self.thrust_angle) as f64)
                        .to_radians()
                        .cos(),
            ) + ACCELERATION_DUE_TO_GRAVITY
        };

        let mut burn = Burn::new();

        // get the needed thrust
        let thrust = self.calculate_thrust(30.0);

        burn.rotate = thrust.direction();
        burn.power = thrust.magnitude().round() as i32;

        // Determine the time needed to do this burn
        // let duration = Self::time_of_burn_for_x(
        //     self.velocity.x,
        //     Self::pow_rot_to_accel_x(burn.power, burn.rotate),
        // );
        // burn.duration = duration;

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
        assert!(self.position.x != -1.0 && self.position.y != -1.0);
        assert!(self.target_position.x == -1.0 && self.target_position.y == -1.0);

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

        if self.position.y > flat_surface_points[0].1 as f64
            && self.position.x >= flat_surface_points[0].0 as f64
            && self.position.x <= flat_surface_points[1].0 as f64
        {
            assert!(self.position.y > flat_surface_points[0].1 as f64);
            assert!(self.position.x <= flat_surface_points[1].0 as f64);
            assert!(self.position.x >= flat_surface_points[0].0 as f64);

            // the lander is above the flat surface
            // target is (self.position.x, flat_surface_point.y)
            self.target_position.x = self.position.x;
            self.target_position.y = flat_surface_points[0].1 as f64;
        } else {
            // we know the target y, get the closest target x
            flat_surface_points.sort_by(|(ax, _), (bx, _)| {
                (self.position.x as i32 - *ax)
                    .abs()
                    .cmp(&(self.position.x as i32 - *bx).abs())
            });

            self.target_position.x = flat_surface_points[0].0 as f64;
            self.target_position.y = flat_surface_points[0].1 as f64;
        }
    }

    fn time_of_burn_for_x(v: f64, a: f64) -> i32 {
        // t = -(2 v0)/a and a!=0
        // t = 0

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

    /// This uses the formula:
    /// - `2.0 * (tp - p - v * t) / (t * t)`
    ///   to calculate the thrust needed this turn.
    ///
    /// - `t` is the time variable used in the formula. It essentially acts as a
    ///   look-ahead amount allowing the function to better predict the outcome for
    ///   higher values. Needs to be explored.
    ///
    /// Returns the thrust as a Vec2 in thrust-space coordinates and clamped to
    /// the allowed values of the thruster.
    fn calculate_thrust(&self, t: f64) -> Vec2 {
        let tp = self.target_position;
        let p = self.position;
        let v = self.velocity;

        let mut thrust = match t {
            // if time is greater than zero, calculate thrust
            t if t > 0.0 => {
                let mut target_accel = 2.0 * (tp - p - v * t) / (t * t);

                // add to our current acceleration
                target_accel = target_accel + self.acceleration;

                eprintln!(
                    "accel: the acceleration computed to go from position to target_position"
                );
                eprintln!("\taccel init:     {:?}", target_accel);

                // eprintln!("\tgravity:        {:?}", ACCELERATION_DUE_TO_GRAVITY);
                // account for gravity
                //accel = accel - ACCELERATION_DUE_TO_GRAVITY;

                //eprintln!("\taccel-grav:     {:?}", accel);

                // prevent downward thrust
                target_accel.y = target_accel.y.clamp(0.0, 4.0);
                eprintln!("\taccel-no ↓:     {:?}", target_accel);

                target_accel.clamp_magnitude(0.0, 4.0);
                eprintln!("\taccel-clmp mag: {:?}", target_accel);

                //convert to thrust space
                target_accel = target_accel.rotate(-90.0);
                eprintln!("\taccel thr sp:   {:?}", target_accel);

                target_accel
            }
            _ => Vec2::new(0.0, 0.0),
        };
        // eprintln!("thrust: {:?}", thrust);
        // eprintln!("thrust.mag {:?}", thrust.magnitude());
        // eprintln!("thrust.dir(thrust-space) {:?}", thrust.direction());
        // eprintln!(
        //     "thrust.dir(world-space) {:?}",
        //     thrust.rotate(90.0).direction()
        // );

        {
            //assertion check
            assert!(thrust.direction() >= -90);
            assert!(thrust.direction() <= 90);
            assert!(thrust.magnitude() <= 4.0);
            assert!(thrust.magnitude() >= 0.0);
        }

        thrust
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
    lander.position.x = x as f64;
    lander.position.y = y as f64;
    lander.velocity.x = h_speed as f64;
    lander.velocity.y = v_speed as f64;
    lander.fuel = fuel;
    lander.thrust_angle = rotate;
    lander.rotation = Burn::thrust_to_world_rot(rotate) as f64;
    lander.thrust_power = power;
}

fn main() {
    let mut lander = Lander {
        time: 0,
        position: Vec2 { x: -1.0, y: -1.0 },
        rotation: 0.0,
        velocity: Vec2 { x: 0.0, y: 0.0 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        fuel: -1,
        thrust_power: -1,
        thrust_angle: 0,
        active_burn: Burn::new(),
        target_position: Vec2 { x: -1.0, y: -1.0 },
        surface: Surface::new(),
    };

    let mut surface_data: Vec<(i32, i32)> = vec![];

    game_init(&mut surface_data);
    game_loop(&mut lander, &mut surface_data);
}

fn game_init(surface_data: &mut Vec<(i32, i32)>) {
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

        if lander.target_position.x == -1.0 && lander.target_position.y == -1.0 {
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
