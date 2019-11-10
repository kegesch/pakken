#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde;

use crate::project::Project;
use clap::{load_yaml, App, AppSettings::ColoredHelp, AppSettings::SubcommandRequired, ArgMatches};
use colored::Colorize;
use generator::{Generator, Target};
use parser::parse;
use std::fs::{create_dir, remove_dir, File};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io, process};
use util::error::{PakError, PakResult};
use util::{GENERATOR_FILE_ENDING, PAKKEN_FILE_ENDING};

pub mod project;

macro_rules! status {
    ($x:expr) => {
        print!("\r{}", $x)
    };
}

fn main() {
    let yaml = load_yaml!("pakken.yml");
    let matches = App::from_yaml(yaml)
        .settings(&[ColoredHelp, SubcommandRequired])
        .version(&crate_version!()[..])
        .author(&crate_authors!()[..])
        .set_term_width(80)
        .get_matches();

    match pakken(&matches) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("{}: {}", "fatal".red(), err.to_string());
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

            new(name, path.as_path(), sub.1.unwrap())
        },
        "gen" => {
            let target = sub.1.unwrap().value_of("target").unwrap();
            generate(target, sub.1.unwrap())
        },
        _ => {
            let path = Path::new("./parser/test/example.pakken");
            status!(format!("{}", path.display()));
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
        status!("The folder already exists. ");
        if ask_for_override(path) {
            if let Err(err) = remove_dir(path) {
                eprintln!("The folder could not be removed. Please use another location.");
                return Err(PakError::from(err));
            }
        }
    }

    create_dir(path)?;

    // Boilerplate
    let project = Project::from(name);
    project.save()?;

    let mut pakken_file_name: String = String::from(name);
    pakken_file_name.push_str(PAKKEN_FILE_ENDING);
    let pakken_file = path.join(pakken_file_name);
    status!(format!("Create file {}", pakken_file.display()));
    if let Err(err) = File::create(pakken_file) {
        return Err(PakError::from(err));
    }

    if matches.is_present("git") {
        status!("Initializing git repo");
        git_init(name);
    }

    status!(format!("Done. Project created at {}", project.path.display()));
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

pub fn generate(target: &str, matches: &ArgMatches) -> PakResult<()> {
    // This should create a genmodel file which basically binds the ast to the target model and resolved if something should be overwritten or not
    let project = Project::read()?;
    let mut generator_file = String::from(target);
    generator_file.push_str(GENERATOR_FILE_ENDING);
    let path_to_generator = project.path.join(generator_file);

    if !path_to_generator.exists() || matches.is_present("force") {
        status!("Creating generator.");
        let generator = Generator::new(project.model, Target::from(target));
        generator.save()?;
        status!("Generating code.");
        generator.generate()?;
    } else {
        let generator = Generator::from(path_to_generator.as_path())?;
        status!("Generating code.");
        generator.generate()?;
    }

    status!(format!("Code generated for Target {}", target));

    Ok(())
}
