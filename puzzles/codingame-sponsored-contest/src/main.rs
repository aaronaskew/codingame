#![allow(unused)]

use std::collections::HashMap;
use std::hash::Hash;
use std::io;

static TEST_SETTINGS: TestSettings = TestSettings {
    gameboard_rows: 30,
    gameboard_cols: 30,
    gameboard_unexplored_char: '_',
    gameboard_explored_char: ' ',
    gameboard_wall_char: '#',
    //test_commands: [COMMANDS.right, COMMANDS.right, COMMANDS.right],
    algorithm: "dfs_dumb",
};

//Figure out what the commands are
static COMMANDS: MoveCommand = MoveCommand {
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

    // create enemies vector
    let mut enemies: Vec<Enemy> = Vec::new();
    for i in 0..cg_num_characters - 1 {
        enemies.push(Enemy::new(match i {
            0 => 'α',
            1 => 'β',
            2 => 'γ',
            3 => 'δ',
            _ => '№',
        }));
    }

    let mut pacman = PacMan::new();

    let mut turn_data: Vec<TurnIO> = Vec::new();

    eprintln!("initData: {:?}", init_data);

    let mut turn: usize = 0;

    //build game board for visualization
    let mut gameboard = GameBoard::new(TEST_SETTINGS.gameboard_cols, TEST_SETTINGS.gameboard_rows);

    // let mut last_cmd = 'x';
    // let test_cmds = TEST_SETTINGS.test_commands.to_vec();
    // let commands = test_cmds;

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

        //process the enemy positions
        for enemy in enemies.iter_mut() {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(' ').collect::<Vec<_>>();
            let character_pos_x = parse_input!(inputs[0], i32);
            let character_pos_y = parse_input!(inputs[1], i32);

            enemy.set_pos(vec![character_pos_x, character_pos_y].into());
        }

        //process the pacman position
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(' ').collect::<Vec<_>>();
        let character_pos_x = parse_input!(inputs[0], i32);
        let character_pos_y = parse_input!(inputs[1], i32);
        pacman.set_pos(vec![character_pos_x, character_pos_y].into());

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
            cmd: pacman.last_command.to_string(),
            walls: walls.clone(),
            enemies: enemies.clone(),
            pacman: pacman.clone(),
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
        //last_cmd = commands[turn % commands.len()];
        //eprintln!("command: {}", last_cmd);
        println!("{}", {
            let cmd = pacman.get_next_move(
                &gameboard,
                &enemies,
                pacman
                    .algorithms
                    .iter()
                    .find(|a| a.name == TEST_SETTINGS.algorithm)
                    .unwrap()
                    .function,
            );
            pacman.last_command = match cmd {
                'A' => '⇉',
                'B' => '▣',
                'C' => '⇈',
                'D' => '⇊',
                'E' => '⇇',
                _ => panic!("invalid command"),
            };
            cmd
        });

        //print gameboard
        gameboard.draw_board(&pacman, &enemies, &walls);
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
    enemies: Vec<Enemy>,
    pacman: PacMan,
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
        for c in self.enemies.iter() {
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

        let pacman_string = format![
            "[{}{}{}({:>2},{:>2})]",
            self.pacman.get_letter(),
            if self.pacman._has_ever_moved {
                "$"
            } else {
                " "
            },
            if self.pacman.position_changed() {
                "!"
            } else {
                " "
            },
            self.pacman._position.as_ref().unwrap()[0],
            self.pacman._position.as_ref().unwrap()[1],
        ];
        s.push_str(&pacman_string);

        write!(f, "{s}")
    }
}

#[derive(Debug)]
struct InitData {
    _first_init_input: i32,
    _second_init_input: i32,
    _num_integer_pairs_per_turn: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct MoveCommand {
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
    board: Vec<char>,
    // don't store characters. will retrieve reference on every draw
    // characters: &'static [Character],
}

impl GameBoard {
    fn new(columns: usize, rows: usize) -> Self {
        GameBoard {
            columns,
            rows,
            //initialize board with blanks
            board: vec![TEST_SETTINGS.gameboard_unexplored_char; (columns) * (rows)],
        }
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

    fn draw_board(&mut self, pacman: &PacMan, enemies: &[Enemy], walls: &Walls) {
        // access (i,j) in 1-D array with:
        //   i * cols + j
        //     where i=row & j=col

        // add the enemies to the board
        for enemy in enemies.iter() {
            let row: usize;
            let col: usize;

            match &enemy._position {
                Some(pos) => {
                    col = pos[0] as usize;
                    row = pos[1] as usize;

                    self.board[row * self.columns + col] = enemy._letter;
                }
                None => (),
            }

            // if character's last position != current position, then set the last position to explored
            match &enemy._last_position {
                Some(last_pos) => {
                    if last_pos != enemy._position.as_ref().unwrap() {
                        let last_row = last_pos[1] as usize;
                        let last_col = last_pos[0] as usize;
                        self.board[last_row * self.columns + last_col] =
                            TEST_SETTINGS.gameboard_explored_char;
                    }
                }
                None => (),
            }
        }

        // add pacman to the board
        let row: usize;
        let col: usize;

        match &pacman.get_pos() {
            Some(pos) => {
                col = pos[0] as usize;
                row = pos[1] as usize;

                self.board[row * self.columns + col] = pacman.get_letter();
            }
            None => (),
        }

        // if character's last position != current position, then set the last position to explored
        match &pacman._last_position {
            Some(last_pos) => {
                if last_pos != pacman._position.as_ref().unwrap() {
                    let last_row = last_pos[1] as usize;
                    let last_col = last_pos[0] as usize;
                    self.board[last_row * self.columns + last_col] =
                        TEST_SETTINGS.gameboard_explored_char;
                }
            }
            None => (),
        }

        //add the walls to the board
        match walls.above {
            Wall::Wall => {
                let mut wall_pos = pacman.get_pos().as_ref().unwrap().clone();
                wall_pos[1] -= 1;
                let row = wall_pos[1] as usize;
                let col = wall_pos[0] as usize;
                self.board[row * self.columns + col] = TEST_SETTINGS.gameboard_wall_char;
            }
            Wall::NoWall => (),
        };
        match walls.below {
            Wall::Wall => {
                let mut wall_pos = pacman.get_pos().as_ref().unwrap().clone();
                wall_pos[1] += 1;
                let row = wall_pos[1] as usize;
                let col = wall_pos[0] as usize;
                self.board[row * self.columns + col] = TEST_SETTINGS.gameboard_wall_char;
            }
            Wall::NoWall => (),
        };
        match walls.left {
            Wall::Wall => {
                let mut wall_pos = pacman.get_pos().as_ref().unwrap().clone();
                wall_pos[0] -= 1;
                let row = wall_pos[1] as usize;
                let col = wall_pos[0] as usize;
                self.board[row * self.columns + col] = TEST_SETTINGS.gameboard_wall_char;
            }
            Wall::NoWall => (),
        };
        match walls.right {
            Wall::Wall => {
                let mut wall_pos = pacman.get_pos().as_ref().unwrap().clone();
                wall_pos[0] += 1;
                let row = wall_pos[1] as usize;
                let col = wall_pos[0] as usize;
                self.board[row * self.columns + col] = TEST_SETTINGS.gameboard_wall_char;
            }
            Wall::NoWall => (),
        };

        //build board string and print
        let mut board_string = String::new();
        self.board.iter().enumerate().for_each(|(i, c)| {
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
    //test_commands: [char; 3],
    gameboard_unexplored_char: char,
    gameboard_explored_char: char,
    gameboard_wall_char: char,
    algorithm: &'static str,
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

// struct Player {}

// impl

// struct Enemy {}

trait Character {
    fn set_pos(&mut self, position: Option<Vec<i32>>);
    fn get_pos(&self) -> Option<Vec<i32>>;
    fn get_letter(&self) -> char;
    fn position_changed(&self) -> bool;
}

trait Player: Character {
    /// This needs access to the `gameboard` and the `enemies`
    /// to determine the next move
    ///
    fn get_next_move<F>(&self, gameboard: &GameBoard, enemies: &[Enemy], algorithm: F) -> char
    where
        F: Fn(&Self, &GameBoard, &[Enemy]) -> char;

    fn can_move_in_direction(&self, direction: Direction, gameboard: &GameBoard) -> bool;
}

#[derive(Debug, Clone)]
struct Enemy {
    _last_position: Option<Vec<i32>>,
    _position: Option<Vec<i32>>,
    _has_ever_moved: bool,
    _letter: char,
}

impl Enemy {
    fn new(letter: char) -> Self {
        Enemy {
            _last_position: None,
            _position: None,
            _has_ever_moved: false,
            _letter: letter,
        }
    }
}

impl Character for Enemy {
    fn set_pos(&mut self, position: Option<Vec<i32>>) {
        match position {
            None => (),
            Some(_) => {
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
            }
        };
    }

    fn get_pos(&self) -> Option<Vec<i32>> {
        self._position.clone()
    }

    fn get_letter(&self) -> char {
        self._letter
    }

    fn position_changed(&self) -> bool {
        if self._last_position.is_none() {
            return false;
        }

        self._position != self._last_position
    }
}

#[derive(Debug, Clone)]
struct PacMan {
    _last_position: Option<Vec<i32>>,
    _position: Option<Vec<i32>>,
    _has_ever_moved: bool,
    _letter: char,
    algorithms: Vec<Algorithm>,
    last_command: char,
}
impl PacMan {
    fn new() -> Self {
        PacMan {
            _last_position: None,
            _position: None,
            _has_ever_moved: false,
            _letter: '©',
            algorithms: {
                vec![Algorithm {
                    name: "dfs_dumb",
                    function: Self::dfs_dumb,
                }]
            },
            last_command: 'x',
        }
    }

    //#[allow(dead_code)]
    fn dfs_dumb(&self, gameboard: &GameBoard, enemies: &[Enemy]) -> char {
        // create a hashmap of explored directions
        let mut explored_directions: HashMap<Direction, bool> = HashMap::new();
        // check each direction for exploration
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            explored_directions.insert(direction, self.explored_in_direction(direction, gameboard));
        }

        eprintln!("explored_directions: {:?}", explored_directions);

        if self.can_move_in_direction(Direction::Up, gameboard) && !explored_directions[&Direction::Up] {
            return COMMANDS.up;
        }

        if self.can_move_in_direction(Direction::Down, gameboard)
            && !explored_directions[&Direction::Down]
        {
            return COMMANDS.down;
        }

        if self.can_move_in_direction(Direction::Left, gameboard)
            && !explored_directions[&Direction::Left]
        {
            return COMMANDS.left;
        }

        if self.can_move_in_direction(Direction::Right, gameboard)
            && !explored_directions[&Direction::Right]
        {
            return COMMANDS.right;
        }

        // if we get here, we are stuck
        COMMANDS.stay_put
    }

    fn explored_in_direction(&self, direction: Direction, gameboard: &GameBoard) -> bool {
        let pos = self.get_pos().unwrap();
        let row = pos[1] as usize;
        let col = pos[0] as usize;
        let _wall_char = TEST_SETTINGS.gameboard_wall_char;
        let (test_row, test_col) = match direction {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        };

        let wall_test_char = gameboard.board[test_row * gameboard.columns + test_col];

        wall_test_char == TEST_SETTINGS.gameboard_explored_char
    }
}
impl Character for PacMan {
    fn set_pos(&mut self, position: Option<Vec<i32>>) {
        match position {
            None => (),
            Some(_) => {
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
            }
        };
    }

    fn get_pos(&self) -> Option<Vec<i32>> {
        self._position.clone()
    }

    fn get_letter(&self) -> char {
        self._letter
    }

    fn position_changed(&self) -> bool {
        if self._last_position.is_none() {
            return false;
        }

        self._position != self._last_position
    }
}

impl Player for PacMan {
    fn get_next_move<F>(&self, gameboard: &GameBoard, enemies: &[Enemy], algorithm: F) -> char
    where
        F: Fn(&Self, &GameBoard, &[Enemy]) -> char,
    {
        algorithm(self, gameboard, enemies)
    }

    fn can_move_in_direction(&self, direction: Direction, gameboard: &GameBoard) -> bool {
        let pos = self.get_pos().unwrap();
        let row = pos[1] as usize;
        let col = pos[0] as usize;
        let _wall_char = TEST_SETTINGS.gameboard_wall_char;
        let (test_row, test_col) = match direction {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        };

        let wall_test_char = gameboard.board[test_row * gameboard.columns + test_col];

        // {
        //     eprintln!(
        //         "can_move_in_direction: {:?} {:?}, char_at: {}, _wall_char: {}, is_wall(using matches!): {}, is_wall(using ==): {}",
        //         direction,
        //         (test_row, test_col),
        //         wall_test_char,
        //         _wall_char,
        //         matches!(
        //             wall_test_char,
        //             _wall_char
        //         ),
        //         wall_test_char == _wall_char
        //     );
        // }

        // test for wall
        if _wall_char == wall_test_char {
            eprintln!("wall in the way");
            return false;
        }

        // test for enemy
        if matches!(wall_test_char, 'α' | 'β' | 'γ' | 'δ') {
            eprintln!("enemy in the way");
            return false;
        }

        // else, can move
        true
    }
}

#[derive(Debug, Clone)]
struct Algorithm {
    name: &'static str,
    function: fn(&PacMan, &GameBoard, &[Enemy]) -> char,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
