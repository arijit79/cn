use clap::{App, Arg, SubCommand};

#[derive(Clone)]
pub struct Flags {
    pub copy: bool,
    pub verbose: bool,
    pub interactive: bool,
}

impl Flags {
    pub fn set(matches: &clap::ArgMatches) -> Flags {
        let mut default = Flags {
            copy: true,
            verbose: false,
            interactive: false,
        };

        if matches.is_present("verbose") {
            default.verbose = true;
        }
        if matches.is_present("move") {
            default.copy = false;
        }
        if matches.is_present("interactive") {
            default.interactive = true;
        }
        default
    }
}

pub fn cli() -> App<'static, 'static> {
    // Get the command-line matches
    App::new("cn")
        .version("2.0.0")
        .author("Arijit Dey <arijid79@gmail.com>")
        .about("Copy SOURCE to DESTINATION")
        .arg(
            Arg::with_name("recursive")
                .long("recursive")
                .short("r")
                .help("Recursively copy directories. Turned on by default"),
        )
        .arg(
            Arg::with_name("no-clobber")
            .short("n")
            .conflicts_with("interactive")
            .long("no-clobber")
            .help("Overwrite any existing files or folders in the destination without confirmation")
        )
        .arg(
            Arg::with_name("interactive")
            .short("i")
            .long("interactive")
            .help("Confirm before overwriting any existing files or folders in the destination")
        )
        .arg(
            Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Show verbose output")
        )
        .subcommand(
            SubCommand::with_name("completion")
            .arg(
                Arg::with_name("shell")
                .help("Generate completions for this SHELL")
                .takes_value(true)
                .short("s")
                .long("shell")
                .value_name("SHELL")
                .required(true)
                .possible_values(&["bash", "fish", "zsh", "elvish", "powershell"])
            )
            .arg(
                Arg::with_name("output")
                .help("Location to dump the output. If the flag is omitted, the result will be printed to stdout")
                .short("o")
                .long("output")
                .takes_value(true)
                .value_name("OUTPUT")
            )
        )
        .arg(
            Arg::with_name("move")
                .long("move")
                .short("m")
                .help("Move source to destination rather than copying them"),
        )
        .arg(
            Arg::with_name("follow-cli-symlinks")
                .short("H")
                .help("Follow symbolic links directly passed in command line")
        )
        .arg(
            Arg::with_name("follow-source-symlinks")
                .short("L")
                .long("dereference")
                .help("Follow symbolic links in sources")
        )
        .arg(
            Arg::with_name("target-directory")
                .short("t")
                .long("target-directory")
                .help("Copy the sources into the this directory")
                .takes_value(true)
                .require_equals(true)
                .value_name("DEST")
        )
        .arg(
            Arg::with_name("no-target-directory")
            .short("T")
            .long("no-target-directory")
            .help("Treat destination as a regular file")
        )
        .arg(
            Arg::with_name("hard-link")
            .short("l")
            .long("link")
            .help("Make hard links instead of copying")
            .conflicts_with("symbolic-links")
        )
        .arg(
            Arg::with_name("symbolic-link")
            .short("s")
            .long("symbolic-link")
            .help("Make symbolic links instead of copying")
        )
        .arg(
            Arg::with_name("paths")
            .takes_value(true)
            .value_name("PATHS")
            .multiple(true)
            .help("The paths that needs to be copied. The last argument is taken as the destination unless -t is given")
        )
}
