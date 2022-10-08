use super::parser::CommandLineArgumentsParser;

pub fn run_command_from_parser(parser: &CommandLineArgumentsParser) -> String {
    use super::parser::VcsCommands::*;
    use crate::vcs_commands::*;

    match &parser.command {
        Init { path } => init::run(path),
        Status => status::run(),
        Commit { message } => commit::run(message),
        Jump {
            branch_name,
            commit_hash,
        } => jump::run(branch_name, commit_hash),
        NewBranch { name } => new_branch::run(name),
        Merge { branch } => merge::run(branch),
        Log => log::run(),
    }
}
