use crate::command_line_handling::launcher::run_command_from_parser;
use crate::command_line_handling::parser::CommandLineArgumentsParser;
use clap::Parser;
use std::io::Error;

/// Simple Version Control System
pub struct Application {
    parser: CommandLineArgumentsParser,
}

// arguments are taken from command line by default
impl Default for Application {
    fn default() -> Self {
        Application {
            parser: CommandLineArgumentsParser::parse(),
        }
    }
}

// or arguments can be provided manually
impl<I> From<I> for Application
where
    I: IntoIterator<Item = &'static str>,
{
    fn from(args_list: I) -> Self {
        Application {
            parser: CommandLineArgumentsParser::parse_from(args_list),
        }
    }
}

impl Application {
    pub fn run(&self) {
        match run_command_from_parser(&self.parser) {
            Ok(s) => {
                println!("{}", s)
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    pub fn try_run(&self) -> Result<(), Error> {
        let output = run_command_from_parser(&self.parser)?;
        println!("{}", output);
        Ok(())
    }
}
