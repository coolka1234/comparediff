use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let matches = Command::new("file-diff")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Compares two files line by line")
        .arg(Arg::new("file1")
             .help("First file to compare")
             .required(true)
             .index(1))
        .arg(Arg::new("file2")
             .help("Second file to compare")
             .required(true)
             .index(2))
        .get_matches();

    let file1_path = matches.value_of("file1").unwrap();
    let file2_path = matches.value_of("file2").unwrap();

    let file1 = File::open(file1_path).unwrap_or_else(|err| {
        eprintln!("Error opening file1: {}", err);
        process::exit(1);
    });

    let file2 = File::open(file2_path).unwrap_or_else(|err| {
        eprintln!("Error opening file2: {}", err);
        process::exit(1);
    });

    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);

    compare_files(reader1, reader2);
}

fn compare_files<R: BufRead, S: BufRead>(mut reader1: R, mut reader2: S) {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let mut lines1 = reader1.lines();
    let mut lines2 = reader2.lines();

    loop {
        match (lines1.next(), lines2.next()) {
            (Some(Ok(line1)), Some(Ok(line2))) => {
                if line1 != line2 {
                    print_colored_line(&mut stdout, &line1, Color::Blue);
                    print_colored_line(&mut stdout, &line2, Color::Blue);
                }
            }
            (Some(Ok(line1)), None) => {
                print_colored_line(&mut stdout, &line1, Color::Red);
            }
            (None, Some(Ok(line2))) => {
                print_colored_line(&mut stdout, &line2, Color::Green);
            }
            (None, None) => break,
            (Some(Err(e)), _) | (_, Some(Err(e))) => {
                eprintln!("Error reading line: {}", e);
                process::exit(1);
            }
        }
    }
}

fn print_colored_line(stdout: &mut StandardStream, line: &str, color: Color) {
    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(color));
    stdout.set_color(&color_spec).unwrap();
    writeln!(stdout, "{}", line).unwrap();
    stdout.reset().unwrap();
}



