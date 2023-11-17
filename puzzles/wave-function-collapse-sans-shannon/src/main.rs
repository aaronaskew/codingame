#![allow(clippy::all)]

use std::fmt::*;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

const KERNEL_SIZE: usize = 3;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w1 = parse_input!(inputs[0], i32);
    let h1 = parse_input!(inputs[1], i32);

    let mut rows: Vec<String> = Vec::<String>::new();
    for i in 0..h1 as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_matches('\n').to_string();
        rows.push(row);
    }

    let prototype = AsciiImage {
        width: w1 as usize,
        height: h1 as usize,
        data: rows,
    };

    eprintln!("prototype: {:?}", prototype);

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w2 = parse_input!(inputs[0], i32);
    let h2 = parse_input!(inputs[1], i32);
    let mut rows: Vec<String> = Vec::<String>::new();

    for i in 0..h2 as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_matches('\n').to_string();
        rows.push(row);
    }

    let partial_solution = AsciiImage {
        width: w2 as usize,
        height: h2 as usize,
        data: rows,
    };

    eprintln!("partial_solution: {:?}", partial_solution);

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    eprintln!("generate patches");
    let patches = generate_3x3_patches(prototype);
    eprintln!("num patches: {}", patches.len());

    eprintln!("generate wave");
    let mut wave = Wave::new(partial_solution, &patches);
    eprintln!("{:?}", wave);

    eprintln!("constrain patches");
    constrain_possible_patches(&patches, &mut wave);
    eprintln!("{:?}", wave);

    // println!(r"+----------+");
    // println!(r"|          |");
    // println!(r"|  *       |");
    // println!(r"| \|    *  |");
    // println!(r"|  |/   |/ |");
    // println!(r"|  |    |  |");
    // println!(r"| \|    |  |");
    // println!(r"+----------+");
}

struct AsciiImage {
    width: usize,
    height: usize,
    data: Vec<String>,
}

impl Debug for AsciiImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut data_string = String::from("");

        data_string += &format!("width: {} height: {}\ndata:\n", self.width, self.height);

        self.data
            .iter()
            .for_each(|val| data_string += &format!("{}\n", val));

        write!(f, "{}", data_string)
    }
}

impl AsciiImage {
    fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::<String>::new();

        for _ in 0..height {
            data.push(String::new());
        }

        Self {
            width,
            height,
            data,
        }
    }

    fn char_at(&self, row: usize, col: usize) -> char {
        assert!(col < self.width);
        assert!(row < self.height);

        self.data[row].chars().nth(col).unwrap()
    }
}

struct Wave {
    width: usize,
    height: usize,
    possible_patches_width: usize,
    possible_patches_height: usize,
    possible_chars: Vec<Vec<Vec<char>>>,
    possible_patches: Vec<Vec<Vec<usize>>>,
}

impl Wave {
    fn new(partial_solution: AsciiImage, patches: &Vec<AsciiImage>) -> Self {
        let width = partial_solution.width;
        let height = partial_solution.height;
        let possible_patches_width = width - KERNEL_SIZE + 1;
        let possible_patches_height = height - KERNEL_SIZE + 1;
        let mut possible_chars = vec![vec![vec!['?'; 1]; width]; height];
        let mut possible_patches =
            vec![vec![vec![0; 1]; width - KERNEL_SIZE + 1]; height - KERNEL_SIZE + 1];
        let patch_width = patches[0].width;
        let patch_height = patches[0].height;

        // init possible chars
        for row in 0..height {
            for col in 0..width {
                possible_chars[row][col][0] = partial_solution.char_at(row, col);
            }
        }

        // init possible patches
        for row in 0..possible_patches_height {
            for col in 0..possible_patches_width {
                possible_patches[row][col] = (0..patches.len()).collect();
            }
        }

        Self {
            width,
            height,
            possible_patches_width,
            possible_patches_height,
            possible_chars,
            possible_patches,
        }
    }
}

impl Debug for Wave {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut data_string = String::from("");

        let mut possible_patches_count =
            vec![vec![99_usize; self.possible_patches_width]; self.possible_patches_height];

        for row in 0..self.possible_patches_height {
            for col in 0..self.possible_patches_width {
                let count = if (self.width - KERNEL_SIZE + 1..self.width).contains(&col)
                    || (self.height - KERNEL_SIZE + 1..self.height).contains(&row)
                {
                    0
                } else {
                    self.possible_patches[row][col].len()
                };

                possible_patches_count[row][col] = count;
            }
        }

        for row in 0..self.possible_patches_height {
            for col in 0..self.possible_patches_width {
                data_string += &format!("{:>2} ", possible_patches_count[row][col]);
            }
            data_string += "\n";
        }

        write!(f, "{}", data_string)
    }
}

// 1 * Calculate possible 3x3 patches. A 5x6 prototype would generate 12 patches
fn generate_3x3_patches(prototype: AsciiImage) -> Vec<AsciiImage> {
    let patch_width = 3;
    let patch_height = 3;

    let mut patches = Vec::<AsciiImage>::new();

    // iterate over the top_left values of the possible 3x3 patches in the prototype
    for row in 0..(prototype.height - patch_height) {
        for col in 0..(prototype.width - patch_width) {
            let mut patch = AsciiImage::new(patch_width, patch_height);

            patch.data.push(String::new());
            patch.data.push(String::new());
            patch.data.push(String::new());

            for ii in 0..patch_width {
                for jj in 0..patch_height {
                    patch.data[jj] += &prototype.data[row + jj]
                        .chars()
                        .nth(col + ii)
                        .unwrap()
                        .to_string();
                }
            }

            patches.push(patch);
        }
    }

    patches
}

// 2 * Constrain patches from left to right, then from top to bottom
fn constrain_possible_patches(patches: &Vec<AsciiImage>, wave: &mut Wave) {
    // for each patch section in wave, constrain all possible patches
    // then constrain all possible characters

    for row in 0..wave.height - KERNEL_SIZE + 1 {
        for col in 0..wave.width - KERNEL_SIZE + 1 {
            constrain_patches_in_section(patches, wave, row, col);
        }
    }

    //constrain_possible_characters_in_section(patches, wave, row, col);
}

/// Constrain all possible patches in a section
fn constrain_patches_in_section(
    patches: &Vec<AsciiImage>,
    wave: &mut Wave,
    wave_row: usize,
    wave_col: usize,
) {
    for (patch_idx, patch) in patches.iter().enumerate() {
        if wave.possible_patches[wave_row][wave_col].contains(&patch_idx) {
            for patch_row in 0..KERNEL_SIZE {
                for patch_col in 0..KERNEL_SIZE {
                    eprintln!(
                        "wrow: {} prow: {} wcol: {} pcol: {} wchars {:?} pchar: {} pidx: {}\n{:?}",
                        wave_row,
                        patch_row,
                        wave_col,
                        patch_col,
                        wave.possible_chars[wave_row + patch_row][wave_col + patch_col],
                        patch.char_at(patch_row, patch_col),
                        patch_idx,
                        patch
                    );
                    if !wave.possible_chars[wave_row + patch_row][wave_col + patch_col]
                        .contains(&'?')
                        && !wave.possible_chars[wave_row + patch_row][wave_col + patch_col]
                            .contains(&patch.char_at(patch_row, patch_col))
                    {
                        wave.possible_patches[wave_row][wave_col].retain(|&x| x != patch_idx);
                    }
                }
            }
        }
    }
}

// 3 * After a patch has been constrained, constrain all symbols that are covered by the 3x3 patch

// 4 * If there are still uncertain symbols goto step 2

// ## Algorithm
// 1. Read the input bitmap and count NxN patterns.

// 2. Create an array with the dimensions of the output (called "wave" in the source).
//    Each element of this array represents a state of an NxN region in the output.
//    A state of an NxN region is a superposition of NxN patterns of the input with
//    boolean coefficients (so a state of a pixel in the output is a superposition of
//    input colors with real coefficients). False coefficient means that the corresponding
//    pattern is forbidden, true coefficient means that the corresponding pattern is not yet forbidden.

// 3. Initialize the wave in the completely unobserved state, i.e. with all the boolean coefficients being true.

// 4. Repeat the following steps:
//     1. Observation:
//         1. Find a wave element with the minimal nonzero entropy. If there is no such elements (if all elements have zero or undefined entropy) then break the cycle (4) and go to step (5).
//         2. Collapse this element into a definite state according to its coefficients and the distribution of NxN patterns in the input.
//     2. Propagation: propagate information gained on the previous observation step.

// 5. By now all the wave elements are either in a completely observed state (all the coefficients except one being zero) or in the contradictory state (all the coefficients being zero). In the first case return the output. In the second case finish the work without returning anything.
