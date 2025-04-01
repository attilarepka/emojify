mod characters;

use arboard::Clipboard;
use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about = "csv-compare", long_about = None)]
struct Args {
    /// Space width of output
    #[arg(long)]
    space_width: usize,
}

fn main() {
    let args = Args::parse();

    let mut clipboard = Clipboard::new().unwrap();
    let mut input = String::new();
    println!("Text to emojiify: ");
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let clipboard_text = clipboard.get_text().unwrap();
    let emoji = clipboard_text.chars().next().unwrap().to_string();
    let space = " ".repeat(args.space_width);

    let mut matrices: Vec<Vec<Vec<String>>> = Vec::new();

    for ch in input.chars() {
        if let Some(matrix) = characters::map().get(&ch.to_ascii_uppercase().to_string().as_str()) {
            let modified_matrix: Vec<Vec<String>> = matrix
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|&x| if x == 1 { emoji.clone() } else { space.clone() })
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

    let mut output_rows: Vec<String> = vec![];

    let num_rows = matrices[0].len();
    for row_index in 0..num_rows {
        let mut row_parts: Vec<String> = vec![];

        for matrix in &matrices {
            let row_str: String = matrix[row_index].concat();
            row_parts.push(row_str);
        }

        let joined_row = row_parts.join("  ");
        output_rows.push(joined_row);
    }

    let result = output_rows.join("\n");
    clipboard.set_text(&result).unwrap();
}
