use std::{
    fs, io,
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_name = "FILE1", parse(from_os_str), required = true)]
    file1: Option<PathBuf>,

    #[clap(value_name = "FILE2", parse(from_os_str), required = true)]
    file2: Option<PathBuf>,
}

fn swap_files(file1: &Path, file2: &Path) -> Result<(), io::Error> {
    let tmp_file = file1.with_file_name(".corautils-swap.tmp");
    // TODO: Has to remove tmp_file when falling in the fs::copy path

    fs::rename(file1, &tmp_file)?;

    match fs::rename(file2, file1) {
        Ok(_) => {
            // Everything is fine, continue
        }
        Err(_) => {
            // Something went wrong on moving, try to copy
            fs::copy(file2, file1)?;
        }
    }

    match fs::rename(&tmp_file, file2) {
        Ok(_) => {
            // Everything is fine, continue
        }
        Err(_) => {
            // Something went wrong on moving, try to copy
            fs::copy(tmp_file, file2)?;
        }
    }

    return Ok(());
}

fn main() {
    let cli = Cli::parse();
    let file1 = cli.file1.unwrap();
    let file2 = cli.file2.unwrap();

    if !file1.exists() || !file2.exists() {
        println!("Both files must exist");
        return;
    }

    match swap_files(&file1, &file2) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }
}
