use clap::{App, Arg};
use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufWriter};
use regex::Regex;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let matches = App::new("Whitespace Remover")
        .version("1.0")
        .arg(
            Arg::with_name("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Sets the input file to use")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .help("Sets the output file name")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input_dir")
                .short('i')
                .long("input-dir")
                .value_name("INPUT_DIR")
                .help("Sets the input directory")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output_dir")
                .short('d')
                .long("output-dir")
                .value_name("OUTPUT_DIR")
                .help("Sets the output directory")
                .required(false)
                .takes_value(false),
        )
        .get_matches();
    

        let mut input_file_path = PathBuf::from(matches.value_of("input_dir").unwrap_or("./template"));
        input_file_path.push(matches.value_of("file").unwrap());


        let file = File::open(&input_file_path)?;
        let reader = BufReader::new(file);
    
        let mut output = Vec::new();
    
        let re = Regex::new(r"\{|\}|\[|\]|\,").unwrap();
    
        // Process each line from the input file
        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => continue,
            };
    
            let mut in_quotes = false;
            let mut processed_line = String::new();
    
            // Process each character from the line
            for char in line.chars() {
                match char {
                    '"' | '\'' => {
                        in_quotes = !in_quotes;
                        processed_line.push(char);
                    },
                    ' ' | '\n' | '\t' => {
                        if in_quotes {
                            processed_line.push(char);
                        }
                    },
                    _ => {
                        processed_line.push(char);
                    }
                }
            }
    
            // Use regex to process the line further if needed
            let processed_line = re.replace_all(&processed_line, |caps: &regex::Captures| {
                format!("{}", &caps[0])
            });
    
            output.extend_from_slice(processed_line.as_bytes());
        }


        let mut output_file_path  = PathBuf::from(matches.value_of("output_dir").unwrap_or("./output"));
        let output_file_name = format!("{}.pdl", matches.value_of("output").unwrap_or("output"));
        output_file_path.push(output_file_name);
        let mut output_file = BufWriter::new(File::create(output_file_path)?);
        output_file.write_all(&output)?;

        Ok(())
}
