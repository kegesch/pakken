#[macro_use]
extern crate clap;

use crate::error::{PakError, PakResult};
use clap::{load_yaml, App, AppSettings::ColoredHelp, AppSettings::SubcommandRequired, ArgMatches};
use colored::Colorize;
use parser::parse;
use std::fs::{create_dir, remove_dir, File};
use std::path::Path;
use std::process::Command;
use std::{fs, io, process};

pub mod error;

fn main() {
    let yaml = load_yaml!("pakken.yml");
    let matches = App::from_yaml(yaml)
        .settings(&[ColoredHelp, SubcommandRequired])
        .version(&crate_version!()[..])
        .set_term_width(80)
        .get_matches();

    match pakken(&matches) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Pakken: error {}", err.to_string());
            process::exit(1);
        },
    };
}

fn pakken(matches: &ArgMatches) -> PakResult<()> {
    let sub = matches.subcommand();

    match sub.0 {
        "new" => {
            let name_args = sub.1.unwrap().value_of("name").unwrap();
            let replaced_name = name_args.replace("/", "-");
            let name = replaced_name.as_str();
            let path = Path::new("./").join(name);

            new(name, path.as_path(), matches)
        },
        "gen" => {
            let target = sub.1.unwrap().value_of("target").unwrap();
            generate(target)
        },
        _ => {
            let path = Path::new("./parser/test/example.pakken");
            println!("{}", path.display());
            let file = fs::read_to_string(path.canonicalize().unwrap());
            println!("{}", path.canonicalize().unwrap().display());
            println!("{:?}", file);
            if let Ok(code) = file {
                println!("Parsing! {:?}", code);
                println!("Result: {:?}", parse(code.as_str()));
                Ok(())
            } else {
                let message = format!("Could not read file {}", path.display());
                Err(PakError::CustomError(message))
            }
        },
    }
}

fn new(name: &str, path: &Path, matches: &ArgMatches) -> PakResult<()> {
    if path.exists() {
        println!("The folder already exists. ");
        if ask_for_override(path) {
            if let Err(err) = remove_dir(path) {
                eprintln!("The folder could not be removed. Please use another location.");
                return Err(PakError::from(err));
            }
        }
    }

    create_dir(path)?;

    // Boilerplate
    let mut pakken_file_name: String = String::from(name);
    pakken_file_name.push_str(".pkn");
    let pakken_file = path.join(pakken_file_name);
    println!("Create file {}", pakken_file.display());
    if let Err(err) = File::create(pakken_file) {
        return Err(PakError::from(err));
    }
    print!(".");

    if matches.is_present("git") {
        println!("Initializing git repo");
        git_init(name);
        print!(".");
    }

    println!("Done.");
    Ok(())
}

fn ask_for_override(file: &Path) -> bool {
    println!("overwrite {}? (y/n [n])", file.display());

    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {},
        Err(_) => {
            return false;
        },
    }
    buffer.starts_with("y")
}

pub fn git_init(name: &str) {
    let mut cmd = "cd ".to_string();
    cmd.push_str(name);
    cmd.push_str("&&");
    cmd.push_str("git init && git add *");
    if let Ok(c) = Command::new("sh").arg("-c").arg(cmd).stdout(std::process::Stdio::null()).spawn()
    {
        c.wait_with_output().expect("failed to wait on child");
    } else {
        eprintln!("{}, git failed to initialize. Is git on your path?", "Error".red());
        std::process::exit(0x0f01);
    }
}

pub fn generate(_target: &str) -> PakResult<()> {
    // This should create a genmodel file which basically binds the ast to the target model and resolved if something should be overwritten or not

    //TODO make sure this is pakken project

    println!("Generating code.");
    Ok(())
}
