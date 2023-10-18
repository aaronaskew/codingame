#![allow(unused)]
use phys::*;
use std::arch::x86_64::_CMP_TRUE_UQ;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::io;
use std::process::{exit, ExitCode};
use vec2::*;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

static ACCELERATION_DUE_TO_GRAVITY: Vec2 = Vec2 { x: 0.0, y: -3.711 };

mod phys {
    use crate::vec2::Vec2;

    /// Calculate the position at time `t` given the initial position `p`,
    /// initial velocity `v`, and constant acceleration `a`.
    pub fn kinematic_position(p: Vec2, v: Vec2, a: Vec2, t: f64) -> Vec2 {
        p + v * t + 0.5 * a * t * t
    }

    /// Calculate the velocity at time `t` given the initial velocity `v` and
    /// constant acceleration `a`.
    pub fn kinematic_velocity(v: Vec2, a: Vec2, t: f64) -> Vec2 {
        v + a * t
    }

    /// Calculate the acceleration needed to go from `p` to `tp` in `t` seconds
    /// given the initial velocity `v`.
    pub fn kinematic_acceleration(tp: Vec2, p: Vec2, v: Vec2, t: f64) -> Vec2 {
        2.0 * (tp - p - v * t) / (t * t)
    }
}

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

        pub fn distance_between(&self, other: &Vec2) -> f64 {
            ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y))
                .sqrt()
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
    vertical_distance_to_surface: f64,
    init_data: Vec<Vec2>,
    /// The surface elevation for every x-pos, `data[x-pos] = y-pos`
    data: Vec<f64>,
}

impl Surface {
    fn new() -> Self {
        Self {
            vertical_distance_to_surface: 0.0,
            init_data: Vec::new(),
            data: Vec::new(),
        }
    }

    /// Buildout self.data from self.init_data
    fn build_data(&mut self, init_data: Vec<Vec2>) {
        self.data = vec![0.0; 7000];

        // fill self.data with the calculated surface elevation given by
        // the points in self.init_data
        for i in 0..init_data.len() - 1 {
            let p1 = init_data[i];
            let p2 = init_data[i + 1];

            // calculate the slope of the line between p1 and p2
            let m = (p2.y - p1.y) / (p2.x - p1.x);

            // calculate the y-intercept of the line between p1 and p2
            let b = p1.y - m * p1.x;

            // calculate the surface elevation for every x-pos
            for x in p1.x as usize..p2.x as usize {
                self.data[x] = m * x as f64 + b;
            }
        }

        //eprintln!("self.data: {:?}", self.data);

    }

    /// Find the two horizontal points that we are between. Define a line
    /// between those two points. Return the vertical distance between
    /// the line and `position`.
    fn vert_dist_from(&self, position: Vec2) {
        todo!("vert_dist_from")
    }

    // / Find the two horizontal points that we are between. Define a line
    // / between those two points. Return the vertical distance between
    // / the line and `position`.
    // fn vert_dist_from_old(&self, position: Vec2) -> f64 {
    //     let mut distance = Vec2::new(f64::MAX, f64::MAX);

    //     // Find the two closest points

    //     let mut points = self.init_data.clone();
    //     points.sort_by(|a, b| a.0.cmp(&b.0));

    //     let mut surrounding_points: [(i32, i32); 2] = [points[0], points[1]];

    //     for point in points.iter().skip(2) {
    //         if (point.0 - position.x as i32).abs()
    //             < (surrounding_points[0].0 - position.x as i32).abs()
    //             && point.0 as f64 <= position.x
    //         {
    //             surrounding_points[1] = surrounding_points[0];
    //             surrounding_points[0] = *point;
    //         } else if (point.0 - position.x as i32).abs()
    //             < (surrounding_points[1].0 - position.x as i32).abs()
    //             && point.0 as f64 >= position.x
    //         {
    //             surrounding_points[1] = *point;
    //         }
    //     }
    //     // eprintln!("surrounding_points: {:#?}", surrounding_points);
    //     // eprintln!(
    //     //     "distance to surrounding_points[0]: {}",
    //     //     Vec2::new(
    //     //         surrounding_points[0].0 as f64,
    //     //         surrounding_points[0].1 as f64
    //     //     )
    //     //     .distance_between(&position)
    //     // );
    //     assert!(surrounding_points.len() == 2);
    //     assert!(surrounding_points[0].0 <= position.x as i32);
    //     assert!(surrounding_points[1].0 >= position.x as i32);

    //     distance.y = {
    //         fn vertical_distance(p1: &Vec2, p2: &Vec2, l: &Vec2) -> f64 {
    //             // Calculate the slope of the hypotenuse
    //             let m = (p2.y - p1.y) / (p2.x - p1.x);

    //             match m {
    //                 m if m == 0.0 => {
    //                     // If the slope is 0, then the hypotenuse is horizontal and the
    //                     // vertical distance is the difference between the y-coordinate
    //                     // of the hypotenuse and the y-coordinate of the intersection
    //                     // point (y_I)
    //                     (l.y - p1.y).abs()
    //                 }
    //                 _ => {
    //                     // Calculate the x-coordinate of the intersection point (x_I)
    //                     let x_i = (l.x / m + l.y - p1.y + m * p1.x) / (m + 1.0 / m);

    //                     // Calculate the y-coordinate of the intersection point (y_I) using the equation of the hypotenuse
    //                     let y_i = p1.y + m * (x_i - p1.x);

    //                     // Return the absolute vertical distance
    //                     (l.y - y_i).abs()
    //                 }
    //             }
    //         }

    //         vertical_distance(
    //             &Vec2::new(
    //                 surrounding_points[0].0 as f64,
    //                 surrounding_points[0].1 as f64,
    //             ),
    //             &Vec2::new(
    //                 surrounding_points[1].0 as f64,
    //                 surrounding_points[1].1 as f64,
    //             ),
    //             &position,
    //         )
    //     };

    //     distance.y
    // }
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
    fn determine_target(&mut self) {
        assert!(self.position.x != -1.0 && self.position.y != -1.0);
        assert!(self.target_position.x == -1.0 && self.target_position.y == -1.0);

        //eprintln!("entering determine_target()");

        let mut flat_surface_points: Vec<Vec2> = Vec::new();

        flat_surface_points.append(
            &mut self
                .surface
                .init_data
                .clone()
                // Gets pairs at a time from surface
                .windows(2)
                // Ensures that the iter will only contain points of flat surface
                // by checking that the delta_y between two points is 0
                .filter(|window| window[0].y == window[1].y)
                // Converts the iter of pairs to an iter of points
                .flat_map(|window| vec![window[0], window[1]])
                // Collects the iter of points into a Vec
                .collect::<Vec<Vec2>>(),
        );

        //eprintln!("flat_surface_points: {:#?}", flat_surface_points);

        assert!(flat_surface_points.len() == 2);

        // find the closest point to the lander
        // 1) check if the lander is directly above a flat surface point
        // 2) if not, find the closest point to the lander
        // 3) set the target

        if self.position.y > flat_surface_points[0].y
            && self.position.x >= flat_surface_points[0].x
            && self.position.x <= flat_surface_points[1].x
        {
            assert!(self.position.y > flat_surface_points[0].y);
            assert!(self.position.x <= flat_surface_points[1].x);
            assert!(self.position.x >= flat_surface_points[0].x);

            // the lander is above the flat surface
            // target is (self.position.x, flat_surface_point.y)
            self.target_position.x = self.position.x;
            self.target_position.y = flat_surface_points[0].y;
        } else {
            // we know the target y, get the closest target x
            flat_surface_points.sort_by(|a, b| {
                (self.position.x as i32 - a.x as i32)
                    .abs()
                    .cmp(&(self.position.x as i32 - b.x as i32).abs())
            });

            self.target_position.x = flat_surface_points[0].x;
            self.target_position.y = flat_surface_points[0].y;
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
    fn calculate_thrust(&mut self, t: f64) -> Vec2 {
        let mut thrust_tgt_pos: Vec2;
        let p = self.position;
        let v = self.velocity;
        let mut t = t;

        //todo!("fix vert_dist_from");
        //self.surface.vertical_distance_to_surface = self.surface.vert_dist_from(self.position);

        let mut thrust: Vec2 = match &mut t {
            // if time is greater than zero, calculate thrust
            t if *t > 0.0 => {
                // start with the target position as our target
                thrust_tgt_pos = self.target_position;

                let mut target_accel = Vec2::new(0.0, 0.0);

                #[derive(Debug, PartialEq)]
                enum SanityCheckFailure {
                    WouldCrash,
                    TooFastWhenLanding,
                    WouldRunOutOfFuel,
                    None,
                }

                let mut failure = SanityCheckFailure::None;
                let mut failed = false;

                // loop until we pass all sanity checks, adjusting the thrust_tgt_pos
                // and t as needed
                loop {
                    match (&mut failed, &mut failure) {
                        (false, _) => {
                            target_accel = kinematic_acceleration(thrust_tgt_pos, p, v, *t);

                            // add our current acceleration
                            target_accel = target_accel + self.acceleration;

                            // prevent downward thrust
                            target_accel.y = target_accel.y.clamp(0.0, 4.0);
                            // eprintln!("\taccel-no ↓:     {:?}", target_accel);

                            target_accel.clamp_magnitude(0.0, 4.0);
                            // eprintln!("\taccel-clmp mag: {:?}", target_accel);

                            // check if we are going to crash into the ground
                            // if new position determined by the kinematic equation is below the
                            // surface, then we are going to crash
                            // ====================================================================
                            // {
                            //     let new_position = kinematic_position(p, v, target_accel, *t);
                            //     if new_position.y < self.surface.vert_dist_from(new_position) {
                            //         failed = true;
                            //         failure = SanityCheckFailure::WouldCrash;

                            //         eprintln!(
                            //             "WouldCrash:\ntp: {:?} p: {:?} v: {:?} a: {:?} t: {}",
                            //             thrust_tgt_pos, p, v, target_accel, t
                            //         );

                            //         continue;
                            //     }
                            // }

                            // do more checks

                            // if everything looks good, then break out of the loop
                            break;
                        }
                        (true, SanityCheckFailure::WouldCrash) => {
                            //eprintln!("WouldCrash: tp: {:?}", thrust_tgt_pos);

                            // fix crash

                            // if we are going to crash, then we need to adjust the target
                            // position to be above the surface
                            //thrust_tgt_pos.y *= 1.5;
                            thrust_tgt_pos.y = self.position.y;

                            failed = false;
                            failure = SanityCheckFailure::None;
                            continue;
                        }
                        (true, SanityCheckFailure::TooFastWhenLanding) => todo!(),
                        (true, SanityCheckFailure::WouldRunOutOfFuel) => todo!(),
                        (true, SanityCheckFailure::None) => exit(1),
                    }

                    assert!(!failed);
                    assert!(failure == SanityCheckFailure::None);

                    // eprintln!(
                    //     "accel: the acceleration computed to go from position to target_position"
                    // );
                    // eprintln!("\taccel init:     {:?}", target_accel);
                }

                //convert to thrust space
                target_accel = target_accel.rotate(-90.0);
                // eprintln!("\taccel thr sp:   {:?}", target_accel);
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

    game_init(&mut lander);
    game_loop(&mut lander);
}

fn game_init(lander: &mut Lander) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let surface_n = parse_input!(input_line, i32);

    lander.surface.init_data = vec![Vec2::new(-1.0, -1.0); surface_n as usize];

    // the number of points used to draw the surface of Mars.
    #[allow(clippy::needless_range_loop)]
    for i in 0..surface_n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let land_x = parse_input!(inputs[0], i32); // X coordinate of a surface point. (0 to 6999)
        let land_y = parse_input!(inputs[1], i32); // Y coordinate of a surface point. By linking all the points together in a sequential fashion, you form the surface of Mars.
        (lander.surface.init_data[i].x, lander.surface.init_data[i].y) =
            (land_x as f64, land_y as f64);
    }

    lander.surface.build_data(lander.surface.init_data.clone());
}

fn game_loop(lander: &mut Lander) {
    // game loop

    loop {
        read_lander_data(lander);

        if lander.target_position.x == -1.0 && lander.target_position.y == -1.0 {
            lander.determine_target();
        }

        lander.physics_update();

        // get commands
        println!("{}", lander.get_commands());

        //eprintln!("lander: {:#?}", lander);

        //increment time
        lander.time += 1;
    }
}
