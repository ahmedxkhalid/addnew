use std::collections::HashSet;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufWriter, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1); // Skip the program name

    // Handle arguments: input file and optional output file
    let mut input_file: Option<String> = None;
    let mut output_file: Option<String> = None;
    while let Some(arg) = args.next() {
        if arg == "-o" {
            output_file = Some(args.next().expect("Missing output file after -o"));
        } else {
            input_file = Some(arg);
        }
    }

    // Get input file path
    let input_file_path = match input_file {
        Some(path) => path,
        None => {
            println!("Usage: addnew [ -o output_file ] input_file");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Missing input file",
            )));
        }
    };

    // Open input file for reading
    let input_file = File::open(input_file_path)?;
    let reader = io::BufReader::new(input_file);

    // Use a HashSet for efficient duplicate detection
    let mut seen_lines = HashSet::new();

    // Open output file (or use stdout)
    let mut writer: Box<dyn Write> = Box::new(io::stdout());
    if let Some(output_path) = output_file {
        let output_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_path)?;
        writer = Box::new(BufWriter::new(output_file));
    }

    // Read lines, check for duplicates, and write to output
    for line in reader.lines() {
        let line = line?;
        if !seen_lines.contains(&line) {
            seen_lines.insert(line.clone());
            writeln!(writer, "{}", line)?;
        }
    }

    writer.flush()?;
    Ok(())
}
