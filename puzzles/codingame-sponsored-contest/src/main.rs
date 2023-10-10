use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Hints:
 * This puzzle is a bit particular, in the sense that in order to solve it you must
 * first understand what you're expected to do. This might be confusing at first, but that is intended.
 *
 * At each turn, you are given 4 characters followed by several lines each containing a pair of integers.
 * These numbers represent the coordinates of the game characters, including yours.
 *
 * Find out what makes the game end and try to keep it running as long as possible.
 *
 * At the end of each game you obtain a score that depends on your movements. If you do not
 * output any invalid action, you'll obtain at least a score of 2. Under certain conditions,
 * you can be rewarded additional points.
 *
 * Actions A, B, C, D and E allow you to move in the four directions or to hold still.
 * If at the end of the game you have a score of 2, you can deduce that you did not succeed to move.
 * Use this to determine what your coordinates are among the list of coordinates, then identify
 * the meaning of the actions.
 */

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
    let num_integer_pairs_per_turn = parse_input!(input_line, i32);

    let init_data = InitData {
        first_init_input,
        second_init_input,
        num_integer_pairs_per_turn, //the number of characters, one of which is the player
    };

    let mut characters: Vec<Character> = Vec::new();
    for _ in 0..num_integer_pairs_per_turn {
        characters.push(Character::new());
    }
    eprintln!("characters: {characters:#?}"); //dump the characters

    let mut turnData: Vec<TurnIO> = Vec::new();

    eprintln!("initData: {:?}", init_data);

    let mut turn: usize = 0;
    let commands = vec![
        'A', // moves player somehow?
        'B', // hold still
        'C', // moves player somehow?
        'D', // moves player somehow?
        'E', // moves player somehow?
    ];
    let mut last_cmd = String::from("x");

    // game loop
    loop {
        eprintln!("begin turn: {}", turn);
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

        turnData.push(TurnIO {
            turn: turn,
            cmd: last_cmd.to_string(),
            char1: first_input,
            char2: second_input,
            char3: third_input,
            char4: fourth_input,
            int_pairs: Vec::new(),
        });

        for i in 0..num_integer_pairs_per_turn as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let fifth_input = parse_input!(inputs[0], i32);
            let sixth_input = parse_input!(inputs[1], i32);
            //eprintln!("{} {}", fifth_input, sixth_input);
            turnData[turn as usize]
                .int_pairs
                .push((fifth_input, sixth_input));
        }

        //dump turn data
        for td in &turnData {
            eprintln!("{:?}", td.to_string());
        }
        //eprintln!("turnData: {:#?}", turnData);

        //Make a sequence of test commands
        let mut test_cmds: Vec<&str> = Vec::new();
        {
            let test_cmds_string = "B";
            for cmd in test_cmds_string.split(" ") {
                test_cmds.push(cmd);
            }
        }

        let commands = test_cmds;

        //Send next command
        last_cmd = commands[turn % commands.len()].to_string();
        //eprintln!("command: {}", last_cmd);
        println!("{}", last_cmd);

        eprintln!("end turn: {}", turn);
        // Increment turn
        turn += 1;

        // if turn > commands.len() {
        //     break;
        // }
    }
}
#[derive(Debug)]
struct TurnIO {
    turn: usize,
    cmd: String,
    char1: String,
    char2: String,
    char3: String,
    char4: String,
    int_pairs: Vec<(i32, i32)>,
}

impl TurnIO {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(format!["{:>3?}", &self.turn].as_str());
        s.push_str(" ");
        s.push_str(&self.cmd);
        //s.push_str(format!["/{:>3?}", &self.cmd.as_bytes()[0]].as_str());
        s.push_str(" ");
        s.push_str(&self.char1);
        s.push_str(&self.char2);
        s.push_str(&self.char3);
        s.push_str(&self.char4);
        //conv 4 char pattern from binary to decimal
        s.push_str({
            let mut binary_string = String::new();
            for c in [&self.char1, &self.char2, &self.char3, &self.char4].iter() {
                match c.as_str() {
                    "_" => binary_string.push_str("0"),
                    "#" => binary_string.push_str("1"),
                    _ => binary_string.push_str("x"),
                }
            }
            let binary_val = u32::from_str_radix(&binary_string, 2).unwrap();
            format![" {}[{:>2}]", binary_string, binary_val].as_str()
        });
        s.push_str(" ");
        for (i1, i2) in &self.int_pairs {
            s.push_str(format!("({:>2} {:>2})", &i1.to_string(), &i2.to_string()).as_str());
        }
        s
    }
}

#[derive(Debug)]
struct InitData {
    first_init_input: i32,
    second_init_input: i32,
    num_integer_pairs_per_turn: i32,
}

#[derive(Debug)]
struct Character {
    last_position: Option<Vec<i32>>,
    position: Option<Vec<i32>>,
}

impl Character {
    fn new() -> Self {
        Character {
            last_position: None,
            position: None,
        }
    }

    fn set_pos(&mut self, position: Option<Vec<i32>>) {
        //first set last_position
        self.last_position = match &self.position {
            None => None,
            Some(_) => self.position.clone(),
        };

        //then set position
        self.position = match position {
            None => None,
            Some(ref p) => position,
        };
    }

    fn position_changed(&self) -> bool {
        self.position != self.last_position
    }
}
