use std::env;
use std::process;

mod import;
mod export;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut args = args.clone();
    let mut args = args.drain(..);
    let _progname = args.next().unwrap();
    let argslen = args.len();
    println!("{:?}", args);

    let mut runmode = argmod::parsearg();

    if argslen == 0 {
        runmode.help = true;
    }
    println!("{:?}", runmode);
    if runmode.help {
        println!("{}", usage::get(&runmode));
    } else {
        match runmode.mode {
            argmod::Mode::Import => import::main(runmode),
            argmod::Mode::Export => export::main(runmode),
            argmod::Mode::None => println!("Something went wrong..."),
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

    pub enum CMDarg {
        Import,
        Export,
        Git,
        Help,
        Verbose,
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
        pub verbose: bool,
    }

    impl Arguments {
        pub fn new() -> Arguments {
            return Arguments {
                name: PathBuf::from(env::args().collect::<Vec<String>>()[0].clone()),
                mode: self::Mode::None,
                directory: PathBuf::new(),
                git: false,
                help: false,
                verbose: false,
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
                for i in parseout {
                    match i {
                        self::CMDarg::Import => retmode.mode = self::Mode::Import,
                        self::CMDarg::Export => retmode.mode = self::Mode::Export,
                        self::CMDarg::Git => retmode.git = true,
                        self::CMDarg::Help => retmode.help = true,
                        self::CMDarg::Verbose => retmode.verbose = true,
                        self::CMDarg::Error(s) => {
                            println!("Error: Failed to parse: {}", s);
                            process::exit(1)
                        }
                    }
                }
            }
        }

        retmode
    }
}
