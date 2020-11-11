#[macro_use] extern crate log;

use std::env;
use std::process;
use simplelog::*;

#[path = "parser.rs"]
mod parser;

mod import;
mod export;

fn main() {
    parser::main();
    process::exit(1);
    let args: Vec<String> = env::args().collect();
    let mut args = args.clone();
    let mut args = args.drain(..);
    let _progname = args.next().unwrap();
    let argslen = args.len();

    let mut runmode = argmod::parsearg();

    // Start logger
    TermLogger::init(
        runmode.verbosity,
        Config::default(),
        TerminalMode::Mixed,
    ).unwrap();


    debug!("Args: {:?}", args);
    debug!("Runmode: {:?}", runmode);

    if argslen == 0 {
        runmode.help = true;
    }

    if runmode.help {
        println!("{}", usage::get(&runmode));
    } else {
        match runmode.mode {
            argmod::Mode::Import => import::main(runmode),
            argmod::Mode::Export => export::main(runmode),
            argmod::Mode::None => error!("No mode specified"),
        }
    }
    process::exit(0);
}

mod usage {
    use crate::argmod;
    use std::env;

    pub fn get(mode: &argmod::Arguments) -> String {
        match mode.mode {
            argmod::Mode::Import => return self::import(),
            argmod::Mode::Export => return self::export(),
            _ => return self::default(),
        }
    }

    fn default() -> String {
        let helpmessage = format!(
            "Usage: {} [options] <import|export> <directory>

import | Copy dotfiles listed in $configfile from the system to a directory.
export | Copy all dotfiles from a directory to the system.

-a   | Create a tar archive compressed with gzip.
-g   | Initialise a Git repository in the directory, if one doesn't already exist.
-h/? | Output a help page.
-v   | Enable verbose output.",
            env::args().collect::<Vec<String>>()[0]
        );
        helpmessage
    }

    fn import() -> String {
        let helpmessage = "[Import help message]".to_string();
        helpmessage
    }

    fn export() -> String {
        let helpmessage = "[Export help message]".to_string();
        helpmessage
    }
}

mod argmod {
    use std::env;
    use std::process;
    use std::path::PathBuf;
    use simplelog::LevelFilter;

    pub enum CMDarg {
        Import,
        Export,
        Git,
        Help,
        Verbose,
        Tar,
        Error(String),
    }

    #[derive(Debug, PartialEq)]
    pub enum Mode {
        None,
        Import,
        Export,
    }

    #[derive(Debug)]
    pub struct Arguments {
        pub name: PathBuf,
        pub mode: self::Mode,
        pub directory: PathBuf,
        pub git: bool,
        pub help: bool,
        pub verbosity: LevelFilter,
        pub tar: bool,
    }

    impl Arguments {
        pub fn new() -> Arguments {
            return Arguments {
                name: PathBuf::from(env::args().collect::<Vec<String>>()[0].clone()),
                mode: self::Mode::None,
                directory: PathBuf::new(),
                git: false,
                help: false,
                verbosity: LevelFilter::Info,
                tar: false,
            };
        }
    }

    pub fn parsearg() -> self::Arguments {
        let mut args: Vec<String> = env::args().collect();
        let args: Vec<String> = args.drain(1..).collect();

        let mut retmode = self::Arguments::new();
        for arg in args {
            if retmode.mode != self::Mode::None && retmode.directory == self::Arguments::new().directory {
                retmode.directory = PathBuf::from(arg.to_string());
            } else {
                let mut parseout = Vec::new();
                match arg.as_str() {
                    "import" => parseout.push(self::CMDarg::Import),
                    "export" => parseout.push(self::CMDarg::Export),
                    _ => {
                        if arg.starts_with("--") {
                            match arg.as_str() {
                                "--git" => parseout.push(self::CMDarg::Git),
                                "--help" => parseout.push(self::CMDarg::Help),
                                "--verbose" => parseout.push(self::CMDarg::Verbose),
                                _ => parseout.push(self::CMDarg::Error(arg)),
                            }
                        } else if arg.starts_with('-') {
                            let mut argdrain = arg.clone();
                            let argdrain = argdrain.drain(..);
                            let mut dashcount = 0;
                            for i in argdrain {
                                if i != '-' {
                                    match i {
                                        'a' => parseout.push(self::CMDarg::Tar),
                                        'g' => parseout.push(self::CMDarg::Git),
                                        'h' => parseout.push(self::CMDarg::Help),
                                        'v' => parseout.push(self::CMDarg::Verbose),
                                        _ => parseout.push(self::CMDarg::Error(arg.clone())),
                                    };
                                } else {
                                    if dashcount > 0 {
                                        parseout.push(self::CMDarg::Error(arg.clone()));
                                    }
                                    dashcount += 1;
                                }
                            }
                        } else {
                            parseout.push(self::CMDarg::Error(arg.clone()))
                        }
                    }
                }
                let mut vcount: i8 = 0;
                for i in parseout {
                    match i {
                        self::CMDarg::Import => retmode.mode = self::Mode::Import,
                        self::CMDarg::Export => retmode.mode = self::Mode::Export,
                        self::CMDarg::Git => retmode.git = true,
                        self::CMDarg::Help => retmode.help = true,
                        self::CMDarg::Verbose => vcount += 1,
                        self::CMDarg::Tar => retmode.tar = true,
                        self::CMDarg::Error(s) => {
                            println!("Error: Failed to parse: {}", s);
                            process::exit(1)
                        }
                    }
                }
                match vcount {
                    v if v <= -2 => retmode.verbosity = LevelFilter::Error,
                    v if v == -1 => retmode.verbosity = LevelFilter::Warn,
                    v if v == 0 => retmode.verbosity = LevelFilter::Info,
                    v if v == 1 => retmode.verbosity = LevelFilter::Debug,
                    v if v >= 2 => retmode.verbosity = LevelFilter::Trace,
                    _ => panic!(),
                }
            }
        }
        retmode
    }
}
