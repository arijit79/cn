use clap::{App, Arg};

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

pub fn matches() -> clap::ArgMatches<'static> {
    // Get the command-line matches
    App::new("cn")
        .version("1.0.0")
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
            Arg::with_name("source")
            .takes_value(true)
            .value_name("SOURCE")
            .multiple(true)
            .help("The paths that needs to be copied")
            .required(true)
            .min_values(1)
        )
        .arg(
            Arg::with_name("dest")
            .takes_value(true)
            .value_name("DESTINATION")
            .help("The path where the sources need to be placed")
            .required(true)
        )
        .get_matches()
}
