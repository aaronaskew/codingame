#![allow(clippy::all)]
#![allow(unused)]

use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Solve this puzzle by writing the shortest code.
 * Whitespaces (spaces, new lines, tabs...) are counted in the total amount of chars.
 * These comments should be burnt after reading!
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let lx = parse_input!(inputs[0], i32); // the X position of the light of power
    let ly = parse_input!(inputs[1], i32); // the Y position of the light of power
    let tx = parse_input!(inputs[2], i32); // Thor's starting X position
    let ty = parse_input!(inputs[3], i32); // Thor's starting Y position

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let remaining_turns = parse_input!(input_line, i32); // The level of Thor's remaining energy, representing the number of moves he can still make.

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // A single line providing the move to be made: N NE E SE S SW W or NW
        println!("SE");
    }
}
