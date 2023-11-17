use std::fmt::Write;
use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let message = input_line.trim_matches('\n').to_string();

    let binary = ascii_to_binary_string(message);
    eprintln!("{:?}", binary);

    let encoded = binary_string_to_zero_encoded(binary);
    eprintln!("{:?}", encoded);

    println!("{}", encoded);
}

/// a function to convert ascii characters to a string representing the 7-bit ascii values
fn ascii_to_binary_string(ascii: String) -> String {
    ascii.chars().fold(String::new(), |mut output, c| {
        let _ = write!(output, "{:07b}", c as u8);
        output
    })
}

/// Convert a string representing the 7-bit ascii values in binaryto zero-encoded strings
///
/// The encoded output message consists of blocks of 0
/// A block is separated from another block by a space
/// Two consecutive blocks are used to produce a series of same value bits (only 1 or 0 values):
/// - First block: it is always 0 or 00. If it is 0, then the series contains 1, if not, it contains 0
/// - Second block: the number of 0 in this block is the number of bits in the series
fn binary_string_to_zero_encoded(input: String) -> String {
    let mut current_char = '0';
    let mut current_char_count = 0;
    let mut output = String::new();

    input.chars().for_each(|c| {
        if current_char == c {
            current_char_count += 1;
        } else {
            if current_char_count > 0 {
                // append last char's encoding to output
                if output != *"" {
                    output += " ";
                }
                output += if current_char == '0' { "00" } else { "0" };
                output += " ";
                for _ in 0..current_char_count {
                    output += "0";
                }
            }

            current_char = c;
            current_char_count = 1;
        }
    });

    output += " ";
    output += if current_char == '0' { "00" } else { "0" };
    output += " ";
    for _ in 0..current_char_count {
        output += "0";
    }

    output
}
