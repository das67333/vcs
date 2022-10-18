use clap::{Parser, Subcommand};

/// Simple Version Control System
#[derive(Parser)]
#[command(about, disable_help_subcommand(true))]
pub struct CommandLineArgumentsParser {
    #[command(subcommand)]
    pub command: VcsCommands,
}

#[derive(Subcommand)]
pub enum VcsCommands {
    /// Create a VCS repository
    Init {
        /// Initialize a repository at the given path
        #[arg(short, long, value_name("DIRECTORY_PATH"))]
        path: String,
    },

    /// Show the working tree status
    Status,

    /// Create a new commit. The current commit must be the last one in the branch
    Commit {
        #[arg(short, long)]
        message: String,
    },

    /// Switch branch or restore working tree files
    #[command(arg_required_else_help(true))]
    Jump {
        /// Swich branch to the provided one
        #[arg(
            short,
            long("branch"),
            value_name("BRANCH_NAME"),
            conflicts_with("commit_hash")
        )]
        branch_name: Option<String>,

        /// Restore working tree files
        #[arg(short, long("commit"), value_name("COMMIT_HASH"))]
        commit_hash: Option<String>,
    },

    /// Create a new branch from the current commit in master
    #[command(visible_alias("new_branch"))]
    NewBranch {
        #[arg(short, long, value_name("BRANCH_NAME"))]
        name: String,
    },

    /// Merge the branch into master. The current commit must be the last one in master
    Merge {
        #[arg(short, long, value_name("BRANCH_NAME"))]
        branch: String,
    },

    /// List commits that are reachable by following parent links from current commit
    Log,
}
