use std::fs::File;

use binrw::{BinRead, BinResult};
use clap::Parser;

use dqhrs_save_editor::save_file::SaveFile;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_save_file: String,
}

fn main() {
    let args = Args::parse();

    let save_file = read_save_file(&args.input_save_file).expect("Failed to read save file");

    println!("gold: {}", save_file.gold);
}

fn read_save_file(filepath: &String) -> BinResult<SaveFile> {
    let mut reader = File::open(filepath)?;
    SaveFile::read(&mut reader)
}
