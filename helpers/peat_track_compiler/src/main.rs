mod freq;
mod model;

use std::fs::create_dir;
use std::fs::read_dir;
use std::path::Path;

use clap::Parser;

/// Compiles a folder of .peat files into a folder of matching .beat files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input folder
    #[arg(short, long)]
    input: String,

    /// Output folder
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    let in_dir = Path::new(&args.input);
    assert!(in_dir.is_dir(), "Input folder does not exist");
    let out_dir = Path::new(&args.output);
    if !out_dir.exists() {
        create_dir(out_dir).expect("Could not create output folder");
    }
    for entry in read_dir(in_dir).expect("Could not read input folder") {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                let in_file_string = path.as_os_str().to_str().unwrap().to_owned();
                let in_file_name = path.file_name().unwrap().to_str().unwrap();
                if in_file_string.ends_with(".peat") {
                    // strip only .peat from the end of the file name
                    let out_file_string =
                        in_file_name.strip_suffix(".peat").unwrap().to_owned() + ".beat";
                    let out_file_string =
                        out_dir.join(out_file_string).to_str().unwrap().to_owned();
                    println!("Converting {:?} to {:?}", in_file_string, out_file_string);
                    let model = model::Track::from_filename(&in_file_string);
                    model.write(&out_file_string);
                } else {
                    println!("Skipping non-peat file: {:?}", in_file_string);
                }
            }
            Err(_) => {
                println!("Skipping invalid file: {:?}", entry);
            }
        }
    }
}
