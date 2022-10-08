use crate::command_line_handling::{
    launcher::run_command_from_parser, parser::CommandLineArgumentsParser,
};
use clap::Parser;

pub struct Application {
    parser: CommandLineArgumentsParser,
}

// arguments are taken from the command line by default
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
    // fn configure(&mut self) {
    //     self.parser = CommandLineArgumentsParser::parse();
    // }

    pub fn run(&self) {
        println!("{}", run_command_from_parser(&self.parser));
    }
}
