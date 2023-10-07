use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let first_init_input = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let second_init_input = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let third_init_input = parse_input!(input_line, i32);

    eprintln!(
        "init input: {} {} {}",
        first_init_input, second_init_input, third_init_input
    );

    let mut turn = 0;

    // game loop
    loop {
        eprintln!("turn: {}", turn);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let first_input = input_line.trim_matches('\n').to_string();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let second_input = input_line.trim_matches('\n').to_string();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let third_input = input_line.trim_matches('\n').to_string();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let fourth_input = input_line.trim_matches('\n').to_string();
        eprintln!(
            "{} \n{} \n{} \n{}",
            first_input, second_input, third_input, fourth_input
        );
        for i in 0..third_init_input as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let fifth_input = parse_input!(inputs[0], i32);
            let sixth_input = parse_input!(inputs[1], i32);
            eprintln!("{} {}", fifth_input, sixth_input);
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("A, B, C, D or E");
        turn += 1;
    }
}
