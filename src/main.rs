mod characters;

use arboard::Clipboard;
use clap::{ArgAction, Parser};
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about = None, long_about = None)]
struct Args {
    /// Text to convert into an emoji representation
    #[arg(long, short)]
    input: Option<String>,

    /// Width of spaces between emoji characters
    #[arg(long, short)]
    space_width: usize,

    /// Emoji used for the foreground (text)
    #[arg(long, short)]
    foreground_emoji: Option<char>,

    /// Emoji used for the background (empty spaces)
    #[arg(long, short)]
    background_emoji: Option<char>,

    /// Extra spacing between characters (in units of space_width)
    #[arg(long, short)]
    character_spacing: Option<usize>,

    /// Print the generated emoji output to the console
    #[arg(long, short, action = ArgAction::SetTrue)]
    print_output: bool,
}

fn main() {
    let args = Args::parse();

    let mut clipboard = Clipboard::new().unwrap();

    let mut input = String::new();
    if let Some(args_input) = args.input {
        input = args_input;
        println!("Text to emojify: {}", input.trim());
    } else {
        println!("Enter text to emojify:");
        io::stdin().read_line(&mut input).unwrap();
    }
    let input = input.trim();

    let emoji = args
        .foreground_emoji
        .unwrap_or_else(|| clipboard.get_text().unwrap().chars().next().unwrap())
        .to_string();
    let background = args
        .background_emoji
        .unwrap_or(' ')
        .to_string()
        .repeat(args.space_width);

    let mut matrices: Vec<Vec<Vec<String>>> = Vec::new();

    for ch in input.chars() {
        if let Some(matrix) = characters::map().get(&ch.to_ascii_uppercase().to_string().as_str()) {
            let modified_matrix: Vec<Vec<String>> = matrix
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|&x| {
                            if x == 1 {
                                emoji.clone()
                            } else {
                                background.clone()
                            }
                        })
                        .collect()
                })
                .collect();

            matrices.push(modified_matrix);
        }
    }

    if matrices.is_empty() {
        eprintln!("No characters recognized in input");
        return;
    }

    let spacing = args
        .background_emoji
        .unwrap_or(' ')
        .to_string()
        .repeat(args.character_spacing.unwrap_or(args.space_width));

    let output_rows: Vec<String> = (0..matrices[0].len())
        .map(|i| {
            matrices
                .iter()
                .map(|m| m[i].concat())
                .collect::<Vec<_>>()
                .join(&spacing)
        })
        .collect();

    let result = output_rows.join("\n");
    clipboard.set_text(&result).unwrap();

    println!("Copied generated emoji to clipboard");

    if args.print_output {
        println!("{}", result);
    }
}
