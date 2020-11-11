use crate::argmod;
use std::process;
use std::fs;
use std::path::PathBuf;
use dirs;
use flate2::Compression;
use flate2::write::GzEncoder;

#[path = "config.rs"]
mod config;

#[path = "git.rs"]
mod git;

pub fn main (mut args: argmod::Arguments) {
    // Check that the directory exists first.

    if args.directory.is_dir() {
        ()
    } else if args.directory.exists() {
        error!("{:?} is not a directory.", args.directory);
        process::exit(1);
    } else {
        match fs::create_dir_all(args.directory.clone()) {
            Ok(_) => (),
            _ => {
                error!("Failed to create directory: {:?}", args.directory);
                process::exit(1);
            }
        }
    }
    args.directory = args.directory.canonicalize().unwrap();
    info!("Directory = {:?}", args.directory);

    // Create Git repository

    if args.git {
        git::initrepo(&args.directory);
    }

    // Find files to copy

    // let mut copyvec = config::parse();
    let mut configfile = args.directory.clone();
    configfile.push("dotfiles.conf");

    let filelist = config::parse(configfile, dirs::home_dir().unwrap());
    debug!("filelist = {:?}", filelist);

    // Copy files over

    for file in filelist {
        let src = file.clone();
        let mut dest = file.strip_prefix(dirs::home_dir().unwrap().to_str().unwrap()).unwrap().to_path_buf();
        dest = PathBuf::from(args.directory.clone()).join(dest);

        info!("Copying {:?}", file);

        match fs::create_dir_all(dest.parent().unwrap()) {
            Ok(_) => (),
            _ => {
                error!("Creating directory {:?} failed.", dest.parent().unwrap());
                process::exit(1);
            }
        }

        match fs::copy(src, dest) {
            Ok(_) => (),
            _ => {
                error!("Copying {:?} failed.", file);
                process::exit(1);
            }
        }
    }

    // Commit to Git repository

    match git::commitall(&args.directory) {
        Ok(_) => info!("Commited to Git repo."),
        _ => warn!("No Git repo to commit to."),
    }

    let mut tardir = args.directory.clone();
    tardir.set_extension("tar.gz");
    info!("Tar path = {:?}", tardir);

    // Make tar.gz archive

    if args.tar {
        archive(&mut args.directory);
    }
}

fn archive(path: &mut PathBuf) {
    let mut tarpath = path.clone();
    tarpath.set_extension("tar.gz");
    let tar_gz = fs::File::create(&tarpath).unwrap();
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("dotfiles", &path).unwrap();
}
