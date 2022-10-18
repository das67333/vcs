use super::parser::CommandLineArgumentsParser;
use crate::util::vcs_state::find_repos_root;
use std::io::Error;
use std::path::Path;

/// Runs a vcs command corresponding to the data from parser
///
/// Forwards exceptions from the command
pub fn run_command_from_parser(parser: &CommandLineArgumentsParser) -> Result<String, Error> {
    use super::parser::VcsCommands::*;
    use crate::vcs_commands::*;

    match &parser.command {
        Init { path } => init::run(Path::new(path)),
        Status => status::run(&find_repos_root()?),
        Commit { message } => commit::run(&find_repos_root()?, message),
        Jump {
            branch_name,
            commit_hash,
        } => jump::run(&find_repos_root()?, branch_name, commit_hash),
        NewBranch { name } => new_branch::run(&find_repos_root()?, name),
        Merge { branch } => merge::run(&find_repos_root()?, branch),
        Log => log::run(&find_repos_root()?),
    }
}
