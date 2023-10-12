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
    let cg_num_characters = parse_input!(input_line, i32);

    let init_data = InitData {
        _first_init_input: first_init_input,
        _second_init_input: second_init_input,
        _num_integer_pairs_per_turn: cg_num_characters, //the number of characters, one of which is the player
    };

    let mut characters: Vec<Character> = Vec::new();
    for _ in 0..cg_num_characters {
        characters.push(Character::new());
    }
    //eprintln!("characters: {characters:#?}"); //dump the characters

    let mut turn_data: Vec<TurnIO> = Vec::new();

    eprintln!("initData: {:?}", init_data);

    let mut turn: usize = 0;

    //Figure out what the commands are
    let commands = MoveCommands {
        up: '?',
        down: '?',
        left: '?',
        right: '?',
        stay_put: 'B',
    };
    //     'A', // moves player somehow?
    //     'B', // hold still
    //     'C', // moves player somehow?
    //     'D', // moves player somehow?
    //     'E', // moves player somehow?

    let mut last_cmd = 'x';

    // let mut test_cmds: Vec<&str> = Vec::new();
    // {
    //     let test_cmds_string = "B";
    //     for cmd in test_cmds_string.split {
    //         test_cmds.push(cmd);
    //     }
    // }

    //Make a sequence of test commands
    let test_cmds = vec![commands.stay_put];

    // let test_cmds: Vec<&str> = test_cmds_string
    //     .char_indices()
    //     .map(|(i, _)| {
    //         let next_char_start = test_cmds_string[i..].chars().next().unwrap().len_utf8();
    //         &test_cmds_string[i..i + next_char_start]
    //     })
    //     .collect();

    let commands = test_cmds;

    // game loop
    loop {
        eprintln!("begin turn: {}", turn);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let char1 = input_line.trim_matches('\n').to_string();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let char2 = input_line.trim_matches('\n').to_string();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let char3 = input_line.trim_matches('\n').to_string();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let char4 = input_line.trim_matches('\n').to_string();

        //process the character positions
        //for i in 0..cg_num_characters as usize {
        for character in characters.iter_mut() {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(' ').collect::<Vec<_>>();
            let character_pos_x = parse_input!(inputs[0], i32);
            let character_pos_y = parse_input!(inputs[1], i32);
            //eprintln!("{} {}", fifth_input, sixth_input);

            character.set_pos(vec![character_pos_x, character_pos_y].into());

            // turnData[turn as usize]
            //     .int_pairs
            //     .push((fifth_input, sixth_input));
        }

        //dump characters
        //eprintln!("characters: {characters:#?}");

        turn_data.push(TurnIO {
            turn,
            cmd: last_cmd.to_string(),
            char1,
            char2,
            char3,
            char4,
            characters: characters.clone(),
        });

        //dump turn data (last 10)
        let i_start: usize = match turn_data.len() {
            x if x < 10 => 0,
            x => x - 10,
        };
        for td in turn_data.iter().skip(i_start) {
            //i_start..turn_data.len() {
            eprintln!("{:?}", td.to_string());
        }
        //eprintln!("turnData: {:#?}", turnData);

        //Send next command
        last_cmd = commands[turn % commands.len()];
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
    characters: Vec<Character>,
}

use std::fmt;

impl fmt::Display for TurnIO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        s.push_str(format!["{:>3?}", &self.turn].as_str());
        s.push(' ');
        s.push_str(&self.cmd);
        //s.push_str(format!["/{:>3?}", &self.cmd.as_bytes()[0]].as_str());
        s.push(' ');
        s.push_str(&self.char1);
        s.push_str(&self.char2);
        s.push_str(&self.char3);
        s.push_str(&self.char4);
        //conv 4 char pattern from binary to decimal
        // s.push_str({
        //     let mut binary_string = String::new();
        //     for c in [&self.char1, &self.char2, &self.char3, &self.char4].iter() {
        //         match c.as_str() {
        //             "_" => binary_string.push_str("0"),
        //             "#" => binary_string.push_str("1"),
        //             _ => binary_string.push_str("x"),
        //         }
        //     }
        //     let binary_val = u32::from_str_radix(&binary_string, 2).unwrap();
        //     format![" {}[{:>2}]", binary_string, binary_val].as_str()
        // });

        // s.push_str(" ");
        // for (i1, i2) in &self.int_pairs {
        //     s.push_str(format!("({:>2} {:>2})", &i1.to_string(), &i2.to_string()).as_str());
        // }

        //print character data
        for (i, c) in self.characters.iter().enumerate() {
            let character_string = format![
                "[c{}{}{}({:>2},{:>2})]",
                i,
                if c._has_ever_moved { "$" } else { " " },
                if c.position_changed() { "!" } else { " " },
                c._position.as_ref().unwrap()[0],
                c._position.as_ref().unwrap()[1],
            ];
            s.push_str(&character_string);
        }

        write!(f, "{s}")
    }
}

#[derive(Debug)]
struct InitData {
    _first_init_input: i32,
    _second_init_input: i32,
    _num_integer_pairs_per_turn: i32,
}

#[derive(Debug, Clone)]
struct Character {
    _last_position: Option<Vec<i32>>,
    _position: Option<Vec<i32>>,
    _has_ever_moved: bool,
    _letter: char,
}

impl Character {
    fn new() -> Self {
        Character {
            _last_position: None,
            _position: None,
            _has_ever_moved: false,
            _letter: '-',
        }
    }

    fn set_pos(&mut self, position: Option<Vec<i32>>) {
        //first set last_position
        self._last_position = match &self._position {
            None => None,
            Some(_) => self._position.clone(),
        };

        //then set position
        self._position = match position {
            None => None,
            Some(_) => position,
        };

        if self.position_changed() {
            self._has_ever_moved = true;
        }
    }

    fn position_changed(&self) -> bool {
        if self._last_position.is_none() {
            return false;
        }

        self._position != self._last_position
    }
}

#[allow(dead_code)]
struct MoveCommands {
    up: char,
    down: char,
    left: char,
    right: char,
    stay_put: char,
}

#[allow(dead_code)]
struct GameBoard {
    width: usize,
    height: usize,
    char0: Character,
    char1: Character,
    char2: Character,
    char3: Character,
    char4: Character,
}

impl GameBoard {
    fn _draw_board(&self, characters: &Vec<Character>) {
        for i in 0..self.width {
            for j in 0..self.height {}
        }
    }
}
