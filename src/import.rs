use crate::argmod;
use std::process;
use std::fs;
use std::env;
use std::path::PathBuf;
use dirs;

#[path = "config.rs"]
mod config;

#[path = "git.rs"]
mod git;

pub fn main (mut args: argmod::Arguments) {
    // Check that the directory exists first.

    if args.directory.is_dir() {
        ()
    } else if args.directory.exists() {
        println!("Error: {:?} is not a directory.", args.directory);
        process::exit(1);
    } else {
        match fs::create_dir_all(args.directory.clone()) {
            Ok(_) => (),
            _ => {
                println!("Error: Failed to create directory: {:?}", args.directory);
                process::exit(1);
            }
        }
    }
    args.directory = args.directory.canonicalize().unwrap();
    println!("Info: Directory = {:?}", args.directory);

    // Create Git repository

    if args.git {
        git::initrepo(&args.directory);
    }

    // Find files to copy

    // let mut copyvec = config::parse();
    let mut configfile = args.directory.clone();
    configfile.push("dotfiles.conf");

    let filelist = config::parse(configfile, dirs::home_dir().unwrap());
    println!("Debug: filelist = {:?}", filelist);

    // Copy files over

    for file in filelist {
        let src = file.clone();
        let mut dest = file.strip_prefix(dirs::home_dir().unwrap().to_str().unwrap()).unwrap().to_path_buf();
        dest = PathBuf::from(args.directory.clone()).join(dest);

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

    // Commit to Git repository

    println!("current dir = {:?}", env::current_dir().unwrap());
    println!("relative repo dir = {:?}", args.directory.strip_prefix(env::current_dir().unwrap()).unwrap().to_path_buf());

    git::commitall(&args.directory).expect("Warn: Couldn't commit to repo.");
}
