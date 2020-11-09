use crate::argmod;
use std::process;
use std::fs;
use std::path::PathBuf;
use dirs;

#[path = "config.rs"]
mod config;

pub fn main (mut args: argmod::Arguments) {
    if ! args.directory.exists() {
        println!("Error: {:?} does not exist.", args.directory);
        process::exit(1);
    }
    if ! args.directory.is_dir() {
        println!("Error: {:?} is not a directory.", args.directory);
        process::exit(1);
    }

    args.directory = args.directory.canonicalize().unwrap();
    println!("Info: Directory = {:?}", args.directory);

    // Find files to copy

    // let mut copyvec = config::parse();
    let mut configfile = args.directory.clone();
    configfile.push("dotfiles.conf");

    let filelist = config::parse(configfile, dirs::home_dir().unwrap());
    println!("Debug: filelist = {:?}", filelist);

    // Copy files over

    for file in filelist {
        let dest = file.clone();
        let mut src = file.strip_prefix(dirs::home_dir().unwrap().to_str().unwrap()).unwrap().to_path_buf();
        src = PathBuf::from(args.directory.clone()).join(src);

        println!("Copying {:?}", file);

        match fs::create_dir_all(dest.parent().unwrap()) {
            Ok(_) => (),
            _ => {
                println!("Error: Creating directory {:?} failed.", dest.parent().unwrap());
                process::exit(1);
            }
        }

        match fs::copy(src, dest) {
            Ok(_) => (),
            _ => {
                println!("Error: Copying {:?} failed.", file);
                process::exit(1);
            }
        }
    }
}
