use colored::{ColoredString, Colorize};
use std::io;
use std::io::Write;

pub trait Logging {
    fn info(&self, headline: &str, message: &str);
    fn warn(&self, headline: &str, message: &str);
    fn error(&self, headline: &str, message: &str);
    fn log(&self, headline: &str, message: &str);
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum LoggingVerbosity {
    Error,
    Warn,
    Info,
}

impl Default for LoggingVerbosity {
    fn default() -> Self { LoggingVerbosity::Error }
}

#[derive(Debug, Default)]
pub struct Logger {
    verbosity: LoggingVerbosity,
}

impl Logger {
    fn print(status: ColoredString, message: &str) {
        println!("{:>12} {}", status, message);
    }

    pub fn remove_last(&self) {
        //TODO fix this
        print!("\r");
        io::stdout().write_all(b"\x1B[K").ok();
    }
}

impl Logging for Logger {
    fn info(&self, headline: &str, message: &str) {
        if self.verbosity <= LoggingVerbosity::Info {
            Self::print(headline.green().bold(), message);
        }
    }

    fn warn(&self, headline: &str, message: &str) {
        if self.verbosity <= LoggingVerbosity::Warn {
            Self::print(headline.yellow().bold(), message);
        }
    }

    fn error(&self, headline: &str, message: &str) {
        if self.verbosity <= LoggingVerbosity::Error {
            Self::print(headline.red().bold(), message);
        }
    }

    fn log(&self, headline: &str, message: &str) { Self::print(headline.bold(), message); }
}
