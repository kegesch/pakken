#[macro_use]
extern crate clap;

use clap::{load_yaml, App, AppSettings::ColoredHelp, AppSettings::SubcommandRequired, ArgMatches};
use colored::Colorize;
use generator::{Generator, GeneratorBuilder};
use once_cell::sync::Lazy;
use parser::parse;
use std::fs::{create_dir, remove_dir, File};
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use std::{fs, io, process};
use targets::graphql::GraphQLTarget;
use targets::typescript::TypeScriptTarget;
use util::error::{PakError, PakResult};
use util::log::{Logger, Logging};
use util::project::Project;
use util::target::TargetRepository;
use util::{GENERATOR_FILE_ENDING, PAKKEN_FILE_ENDING};

static TARGET_REPO: Lazy<Mutex<TargetRepository>> =
    Lazy::new(|| Mutex::new(TargetRepository::default()));

static LOGGER: Lazy<Logger> = Lazy::new(|| Logger::default());

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
            LOGGER.error("Fatal", err.to_string().as_str());
            process::exit(1);
        },
    };
}

fn pakken(matches: &ArgMatches) -> PakResult<()> {
    load_targets()?;

    let sub = matches.subcommand();

    match sub.0 {
        "new" => {
            let name_args = sub.1.unwrap().value_of("name").unwrap();
            let replaced_name = name_args.replace("/", "-");
            let name = replaced_name.as_str();
            let path = Path::new("./").join(name);

            new(name, path.as_path(), sub.1.unwrap())
        },
        "gen" => generate(sub.1.unwrap()),
        _ => {
            let path = Path::new("./parser/test/example.pakken");
            let file = fs::read_to_string(path.canonicalize().unwrap());
            if let Ok(code) = file {
                LOGGER.info("Parsing", path.display().to_string().as_str());
                parse(code.as_str())?;
                LOGGER.remove_last();
                LOGGER.info("Parsing", "done");
                Ok(())
            } else {
                let message = format!("Could not read file {}", path.display());
                Err(PakError::CustomError(message))
            }
        },
    }
}

fn load_targets() -> PakResult<()> {
    // TODO handle this error
    LOGGER.info("Loading", "targets");
    let mut repo = TARGET_REPO.lock().unwrap();
    repo.add(Box::from(GraphQLTarget::default()))?;
    repo.add(Box::from(TypeScriptTarget::default()))?;
    LOGGER.remove_last();
    LOGGER.info("Done", "targets loaded");
    Ok(())
}

fn new(name: &str, path: &Path, matches: &ArgMatches) -> PakResult<()> {
    if path.exists() {
        LOGGER.warn("Create", "The folder already exists!");
        if ask_for_override(path) {
            if let Err(_err) = remove_dir(path) {
                // eprintln!("The folder could not be removed. Please use another location.");
                let message = format!(
                    "The folder {} could not be removed. Please use another location.",
                    path.display()
                );
                return Err(PakError::CustomError(message));
            }
        }
    }
    LOGGER.info("New", "creating project dir");
    create_dir(path)?;

    // Boilerplate
    let project = Project::from(name);
    LOGGER.remove_last();
    LOGGER.info("New", "saving project structure");
    project.save(path)?;

    let mut pakken_file_name: String = String::from(name);
    pakken_file_name.push_str(PAKKEN_FILE_ENDING);
    let pakken_file = path.join(pakken_file_name);

    LOGGER.remove_last();
    LOGGER.info("New", "creating pakken project file");

    if let Err(err) = File::create(pakken_file) {
        return Err(PakError::from(err));
    }

    if matches.is_present("git") {
        LOGGER.remove_last();
        LOGGER.info("New", "initializing git repository");
        git_init(name);
    }

    let message = format!("project created at {}", path.display());
    LOGGER.remove_last();
    LOGGER.info("Done", message.as_str());
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
        //TODO log error correct and return result
        eprintln!("{}, git failed to initialize. Is git on your path?", "Error".red());
        std::process::exit(0x0f01);
    }
}

pub fn generate(matches: &ArgMatches) -> PakResult<()> {
    if matches.is_present("list") {
        LOGGER.info("Targets", "following targets are installed");
        let repo = &TARGET_REPO.lock().expect("Should have a value!");
        for target in repo.list() {
            LOGGER.log("", target.as_str());
        }
        let help_message =
            format!("To generate code against a target use: {}", "pakken gen <target>".italic());
        LOGGER.log("Help", help_message.as_str());

        return Ok(());
    }

    if let None = matches.value_of("target") {
        return Err(PakError::TargetNotFound("NotSpecified".to_owned()));
    }
    let target = matches.value_of("target").unwrap();

    // This should create a genmodel file which basically binds the ast to the target model and resolved if something should be overwritten or not
    let mut generator_file = String::from(target);
    generator_file.push_str(GENERATOR_FILE_ENDING);
    let path_to_generator = Path::new("./").join(generator_file);

    if !path_to_generator.exists() || matches.is_present("force") {
        LOGGER.info("Generate", "creating generator");
        let out_dir = Path::new("./");
        let generator = GeneratorBuilder::new(target).build(out_dir);
        generator.save()?;
        LOGGER.remove_last();
        LOGGER.info("Generate", "generating code");
        generator.generate(&TARGET_REPO.lock().unwrap())?;
    } else {
        let generator = Generator::from(path_to_generator.as_path())?;
        LOGGER.remove_last();
        LOGGER.info("Generate", "generating code");
        generator.generate(&TARGET_REPO.lock().unwrap())?;
    }

    let message = format!("code generated for target {}", target);

    LOGGER.remove_last();
    LOGGER.info("Done", message.as_str());

    Ok(())
}
