use clap::Parser;
use std::io::Read;
use std::{fs, io};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct WcCli {
    file: Option<String>,

    #[arg(short, long)]
    lines: bool,

    #[arg(short, long)]
    chars: bool,

    #[arg(short, long)]
    words: bool,
}

fn main() {
    let cli = WcCli::parse();
    let all_counts = !(cli.chars || cli.lines || cli.words);
    let char_count = cli.chars || all_counts;
    let line_count = cli.lines || all_counts;
    let word_count = cli.words || all_counts;

    let data: Vec<u8> = match &cli.file {
        Some(file) => fs::read(file).unwrap(),
        None => read_stdin()
    };

    if line_count {
        print!("{:>8}", count_lines(&data));
    }
    if word_count {
        print!("{:>8}", count_words(&data));
    }
    if char_count {
        print!("{:>8}", count_bytes(&data));
    }
    println!(" {}", cli.file.unwrap_or(String::from("")))
}

fn read_stdin() -> Vec<u8> {
    let mut buffer = Vec::new();
    io::stdin().lock().read_to_end(&mut buffer).expect("Failed to read stdin");
    buffer
}

fn count_words(data: &Vec<u8>) -> usize {
    data.split(|&byte| is_whitespace_byte(byte)).filter(|word| !word.is_empty()).count()
}

fn is_whitespace_byte(byte: u8) -> bool {
    match byte {
        b' ' | b'\t' | b'\n' | b'\r' | b'\x0C' | b'\x0B' => true, // Basic ASCII whitespace
        _ => false,
    }
}

fn count_lines(data: &Vec<u8>) -> usize {
    data.iter().filter(|&x| *x == b'\n').count()
}

fn count_bytes(data: &Vec<u8>) -> usize {
    data.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_words_with_normal_spaces() {
        let c = count_words(&String::from("the quick brow fox jump over the lazy dog").into_bytes());
        assert_eq!(c, 9);
    }

    #[test]
    fn count_words_with_only_spaces() {
        let c = count_words(&String::from("    ").into_bytes());
        assert_eq!(c, 0);
    }
}
