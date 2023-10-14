#![allow(unused)]
use std::{collections::HashMap, io};

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
    let l = parse_input!(input_line, usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let h = parse_input!(input_line, usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let t = input_line.trim_matches('\n').to_string();

    let mut ascii_art_input: Vec<String> = Vec::new();

    for i in 0..h {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_matches('\n').to_string();

        // eprintln!("{}", row);
        ascii_art_input.push(row);
    }

    // Write an answer using println!("message...");
    // To debug: // eprintln!("Debug message...");

    let aa_alphabet = AsciiArtAlphabet::new(l, h, ascii_art_input);

    aa_alphabet.print_aa_text(t.to_uppercase());
}

struct AsciiArtAlphabet {
    letter_width: usize,
    letter_height: usize,
    aa_letters_map: HashMap<char, Vec<String>>,
}

impl AsciiArtAlphabet {
    fn new(letter_width: usize, letter_height: usize, ascii_art_input: Vec<String>) -> Self {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ?";

        // eprintln!("alphabet.len(): {}", alphabet.len());
        // eprintln!("{:#?}", ascii_art_input);

        AsciiArtAlphabet {
            letter_width,
            letter_height,
            aa_letters_map: {
                let mut letters_map: HashMap<char, Vec<String>> = HashMap::new();

                for i in 0..alphabet.len() {
                    let mut aa_letter: Vec<String> = vec![String::new(); letter_height];

                    for jj in 0..letter_height {
                        for ii in 0..letter_width {
                            aa_letter[jj].push(
                                ascii_art_input[jj]
                                    .chars()
                                    .nth(ii + i * letter_width)
                                    .unwrap(),
                            );
                        }
                    }
                    // eprintln!("{:#?}", aa_letter);

                    letters_map.insert(alphabet.chars().nth(i).unwrap(), aa_letter);
                }

                letters_map
            },
        }
    }

    fn print_aa_text(&self, input: String) {
        let mut output_vec: Vec<String> = vec![String::new(); self.letter_height];

        for c in input.chars() {
            //get the ascii art representation of the character
            let aa_char: Vec<String> = match self.aa_letters_map.get(&c) {
                Some(aa_letter) => aa_letter.clone(),
                None => self.aa_letters_map.get(&'?').unwrap().clone(),
            };

            // append it to the output vector
            for (i, s) in aa_char.iter().enumerate() {
                output_vec[i].push_str(s);
            }
        }

        output_vec.iter().for_each(|s| {
            println!("{}", s);
        });
    }
}
