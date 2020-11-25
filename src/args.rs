use async_std::path::PathBuf;
use clap::{crate_authors, crate_version, Clap};

#[derive(Clone)]
pub struct Flags {
    pub copy: bool,
    pub verbose: bool,
    pub interactive: bool,
}

impl Flags {
    pub fn set(matches: &Cli) -> Flags {
        let mut default = Flags {
            copy: true,
            verbose: false,
            interactive: false,
        };

        if matches.verbose {
            default.verbose = true;
        }
        if matches.r#move {
            default.copy = false;
        }
        if matches.interactive {
            default.interactive = true;
        }
        default
    }
}

/// Copy source to destination
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!() )]
pub struct Cli {
    /// Use this directory instead of DEST
    #[clap(value_name = "DIR", short, long)]
    pub target_directory: Option<PathBuf>,
    /// Treat the destination as a regular file
    #[clap(short = 'T', long, conflicts_with = "target-directory")]
    pub no_target_directory: bool,
    /// Recursively copy all directories. Turned on by default
    #[clap(short, long)]
    pub recrusive: bool,
    /// Overwrite any existing files or folders in the destination without confirmation
    #[clap(short, long, conflicts_with = "interactive")]
    pub no_clobber: bool,
    /// Confirm before overwriting any existing files or folders in the destination
    #[clap(short, long)]
    pub interactive: bool,
    /// Show verbose output
    #[clap(short, long)]
    pub verbose: bool,
    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,
    /// Move the files instead of copying them. This option is hard deprecated and it's usage is strongly discouraged
    #[clap(short, long)]
    pub r#move: bool,
    /// Follow symbolic links given in the command line. It is turned on by default
    #[clap(short = 'H')]
    pub follow_cli_links: bool,
    /// Follow symbolic links in sources. It is turned on by default
    #[clap(short = 'L', long = "dereference")]
    pub follow_source_links: bool,
    /// Make symbolic links instead of copying
    #[clap(short, long)]
    pub symbolic_links: bool,
    /// Make symbolic links instead of copying
    #[clap(short = 'l', long = "link")]
    pub hard_link: bool,
    /// The paths that need to be copied. The last argument is considered as the destination unless -t is given
    #[clap(value_name = "PATHS")]
    pub paths: Vec<PathBuf>,
}
#[derive(Clap)]
pub enum SubCommand {
    // #[clap(version = crate_version!())]
    Completion(Completion),
}
/// Generate completions for a shell
#[derive(Clap)]
pub struct Completion {
    /// Generate completions for this SHELL
    #[clap(short, long, value_name = "SHELL", possible_values = &["bash", "fish", "zsh", "elvish", "powershell"])]
    pub shell: String,
    /// Write completion to this file, if this option is omitted, take stdout into account
    #[clap(short, long, value_name = "OUTPUT")]
    pub output: Option<String>,
}
