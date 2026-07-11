use std::fs::File;

use binrw::{BinRead, BinResult, BinWrite};
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

    let mut save_file = read_save_file(&args.input_save_file).expect("Failed to read save file");

    if save_file.checksum != save_file.calculate_checksum().unwrap() {
        save_file.update_checksum().unwrap();

        write_save_file(&args.input_save_file, &save_file).unwrap();
    }
}

fn read_save_file(filepath: &String) -> BinResult<SaveFile> {
    let mut reader = File::open(filepath)?;
    SaveFile::read(&mut reader)
}

fn write_save_file(filepath: &String, save_file: &SaveFile) -> BinResult<()> {
    let mut writer = File::create(filepath)?;
    save_file.write(&mut writer)
}
