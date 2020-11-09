use std::process;
use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;

pub fn parse(configfile: PathBuf, homedir: PathBuf) -> HashSet<PathBuf> {
    // Check that a config exists first.

    if ! configfile.exists() {
        self::create(&configfile);
        println!("Error: No config file exists at {:?}. A default config file has been created. Modify this to your needs, or replace it.", configfile);
        process::exit(1)
    }
    if ! configfile.is_file() {
        println!("Error: {:?} is not a file!", configfile);
        process::exit(1);
    }

    // Begin parsing
    // Step 1: Read file into phrases, ignoring comments.

    let configread = fs::read_to_string(configfile).expect("Error: Couldn't read config file.");
    let mut configlines: Vec<Vec<String>> = Vec::new();
    for line in configread.split('\n') {
        let line = line.to_string();
        if ! line.starts_with('#') && ! line.is_empty() {
            let mut linevec: Vec<String> = Vec::new();
            for phrase in line.split(' ') {
                let phrase = phrase.to_string();
                if phrase.starts_with('#') {
                    break;
                }
                linevec.push(phrase);
            }
            configlines.push(linevec);
        }
    }

    println!("Debug: configlines = {:?}", configlines);

    // Step 2: Begin the parsing!

    let mut addfiles: HashSet<PathBuf> = HashSet::new();

    for line in configlines {
        let mut lineiter = line.iter();
        println!("Debug: lineiter = {:?}", lineiter);
        let phrase = lineiter.next().unwrap().as_str();
        match phrase {
            "diradd" => {
                let argument = self::canonicalize(PathBuf::from(lineiter.next().unwrap().to_string()), &homedir);
                addfiles.extend(self::walkdir(&argument));
            }
            "fileadd" => {
                let argument = self::canonicalize(PathBuf::from(lineiter.next().unwrap().to_string()), &homedir);
                addfiles.insert(argument);
            }
            _ => {
                println!("Error: Parsing failed at {:?}", phrase);
                process::exit(1);
            }
        }
        if lineiter.next() != None {
            println!("Error: To many arguments at: {:?}", line.join(" "));
            process::exit(1);
        }
    }

    addfiles
}

pub fn create(configfile: &PathBuf) {
    if configfile.exists() {
        println!("Error: {:?} exists!", configfile);
        process::exit(1);
    }

    match fs::write(configfile,
"# An example config file. Add directories with `diradd`, or files with `fileadd`.

diradd ~/.config/gtk-2.0/ # Gtk theme
diradd ~/.config/gtk-3.0/
diradd ~/.config/i3/ # i3wm config
fileadd ~/.config/picom/picom.conf # Picom config
diradd ~/.config/polybar/ # Polybar config")
    {
        Ok(()) => println!("Info: Created {:?}", configfile),
        _ => {
            println!("Error: Failed to create {:?}!", configfile);
            process::exit(1);
        }
    }
}

fn walkdir(dir: &PathBuf) -> HashSet<PathBuf> {
    let mut output: HashSet<PathBuf> = HashSet::new();

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            output.extend(walkdir(&path));
        } else {
            output.insert(PathBuf::from(path.to_string_lossy().to_string()));
        }
    }

    output
}

fn canonicalize(mut path: PathBuf, homedir: &PathBuf) -> PathBuf {
    path = PathBuf::from(self::replacetilde(path, homedir.to_str().unwrap()));

    if path.exists() {
        path.canonicalize().unwrap()
    } else {
        println!("Error: {:?} does not exist.", path);
        process::exit(1);
    }
}

fn replacetilde(mut path: PathBuf, replace: &str) -> PathBuf {
    path = path.strip_prefix("~").unwrap().to_path_buf();
    let mut output = PathBuf::from(replace);
    output.push(path);

    output
}
