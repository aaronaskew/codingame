use std::io;

static TEST_SETTINGS: TestSettings = TestSettings {
    gameboard_rows: 30,
    gameboard_cols: 30,
    test_commands: [COMMANDS.right, COMMANDS.right, COMMANDS.right],
};

//Figure out what the commands are
static COMMANDS: MoveCommands = MoveCommands {
    up: 'C',
    down: 'D',
    left: 'E',
    right: 'A',
    stay_put: 'B',
};

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

    // create characters vector
    let mut characters: Vec<Character> = Vec::new();
    for _ in 0..cg_num_characters {
        characters.push(Character::new());
    }
    // set Characters' display char
    characters
        .iter_mut()
        .enumerate()
        .for_each(|(i, character)| {
            character._letter = match i {
                0 => 'α',
                1 => 'β',
                2 => 'γ',
                3 => 'δ',
                4 => 'ε',
                _ => '№',
            };
        });

    //eprintln!("characters: {characters:#?}"); //dump the characters

    let mut turn_data: Vec<TurnIO> = Vec::new();

    eprintln!("initData: {:?}", init_data);

    let mut turn: usize = 0;

    //build game board for visualization
    let gameboard = GameBoard::new(TEST_SETTINGS.gameboard_cols, TEST_SETTINGS.gameboard_rows);

    let mut last_cmd = 'x';

    // let mut test_cmds: Vec<&str> = Vec::new();
    // {
    //     let test_cmds_string = "B";
    //     for cmd in test_cmds_string.split {
    //         test_cmds.push(cmd);
    //     }
    // }

    //Make a sequence of test commands
    // let test_cmds = vec![commands.stay_put];

    // test the 'A' command
    // let test_cmds = vec!['A'];

    let test_cmds = TEST_SETTINGS.test_commands.to_vec();

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
        let wall0 = input_line.trim_matches('\n').to_string();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let wall1 = input_line.trim_matches('\n').to_string();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let wall2 = input_line.trim_matches('\n').to_string();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let wall3 = input_line.trim_matches('\n').to_string();

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

        // create walls variable
        let walls = Walls {
            above: match &wall0[..] {
                "#" => Wall::Wall,
                _ => Wall::NoWall,
            },
            right: match &wall1[..] {
                "#" => Wall::Wall,
                _ => Wall::NoWall,
            },
            below: match &wall2[..] {
                "#" => Wall::Wall,
                _ => Wall::NoWall,
            },
            left: match &wall3[..] {
                "#" => Wall::Wall,
                _ => Wall::NoWall,
            },
        };

        turn_data.push(TurnIO {
            turn,
            cmd: last_cmd.to_string(),
            walls: walls.clone(),
            characters: characters.clone(),
        });

        // print single TurnIO instead of 10
        eprintln!("{:?}", turn_data.last().unwrap().to_string());

        // //dump turn data (last 10)
        // let i_start: usize = match turn_data.len() {
        //     x if x < 10 => 0,
        //     x => x - 10,
        // };
        // for td in turn_data.iter().skip(i_start) {
        //     //i_start..turn_data.len() {
        //     eprintln!("{:?}", td.to_string());
        // }

        //eprintln!("turnData: {:#?}", turnData);

        //Send next command
        last_cmd = commands[turn % commands.len()];
        //eprintln!("command: {}", last_cmd);
        println!("{}", last_cmd);

        //print gameboard
        gameboard._draw_board(&characters, &walls);
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
    walls: Walls,
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
        s.push_str(match self.walls.above {
            Wall::Wall => "↑",
            Wall::NoWall => "_",
        });
        s.push_str(match self.walls.below {
            Wall::Wall => "↓",
            Wall::NoWall => "_",
        });
        s.push_str(match self.walls.left {
            Wall::Wall => "←",
            Wall::NoWall => "_",
        });
        s.push_str(match self.walls.right {
            Wall::Wall => "→",
            Wall::NoWall => "_",
        });
        // s.push_str(&self.wall1);
        // s.push_str(&self.wall2);
        // s.push_str(&self.wall3);
        // s.push_str(&self.wall4);
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
        for c in self.characters.iter() {
            let character_string = format![
                "[{}{}{}({:>2},{:>2})]",
                c._letter,
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
    columns: usize,
    rows: usize,
    // don't store characters. will retrieve reference on every draw
    // characters: &'static [Character],
}

impl GameBoard {
    fn new(columns: usize, rows: usize) -> Self {
        GameBoard { columns, rows }
    }

    /// draws the gameboard as ascii art
    ///
    /// coordinate system is zero indexed
    /// positive x is to the right
    /// positive y is down
    ///
    /// +---x---->
    /// |
    /// y
    /// |
    /// \/
    ///

    fn _draw_board(&self, characters: &[Character], walls: &Walls) {
        // access (i,j) in 1-D array with:
        //   i * cols + j
        //     where i=row & j=col

        //initialize board with blanks
        let mut board = vec!['_'; (self.columns) * (self.rows)];

        // add the characters to the board
        for character in characters.iter() {
            let row: usize;
            let col: usize;

            match &character._position {
                Some(pos) => {
                    col = pos[0] as usize;
                    row = pos[1] as usize;

                    board[row * self.columns + col] = character._letter;
                }
                None => (),
            }
        }

        let player = &characters[4];

        //add the walls to the board
        match walls.above {
            Wall::Wall => {
                let mut wall_pos = player._position.as_ref().unwrap().clone();
                wall_pos[1] -= 1;
                let row = wall_pos[1] as usize;
                let col = wall_pos[0] as usize;
                board[row * self.columns + col] = '#';
            }
            Wall::NoWall => (),
        };
        match walls.below {
            Wall::Wall => {
                let mut wall_pos = player._position.as_ref().unwrap().clone();
                wall_pos[1] += 1;
                let row = wall_pos[1] as usize;
                let col = wall_pos[0] as usize;
                board[row * self.columns + col] = '#';
            }
            Wall::NoWall => (),
        };
        match walls.left {
            Wall::Wall => {
                let mut wall_pos = player._position.as_ref().unwrap().clone();
                wall_pos[0] -= 1;
                let row = wall_pos[1] as usize;
                let col = wall_pos[0] as usize;
                board[row * self.columns + col] = '#';
            }
            Wall::NoWall => (),
        };
        match walls.right {
            Wall::Wall => {
                let mut wall_pos = player._position.as_ref().unwrap().clone();
                wall_pos[0] += 1;
                let row = wall_pos[1] as usize;
                let col = wall_pos[0] as usize;
                board[row * self.columns + col] = '#';
            }
            Wall::NoWall => (),
        };

        //build board string and print
        let mut board_string = String::new();
        board.iter().enumerate().for_each(|(i, c)| {
            board_string.push(*c);

            // eprintln!(
            //     "i, columns, (i+1)%columns {}, {}, {}",
            //     i,
            //     self.columns,
            //     (i + 1) % self.columns
            // );

            //if i is the last in a row, add a newline
            if (i + 1) % self.columns == 0 {
                board_string.push('\n');
            }
        });

        eprint!("{}", board_string);
    }
}

struct TestSettings {
    gameboard_rows: usize,
    gameboard_cols: usize,
    test_commands: [char; 3],
}

#[derive(Debug, Clone)]
enum Wall {
    Wall,
    NoWall,
}

#[derive(Debug, Clone)]
struct Walls {
    above: Wall,
    right: Wall,
    below: Wall,
    left: Wall,
}
