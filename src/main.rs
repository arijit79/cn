mod checks;

use ansi_term::Colour::Red;
use clap::{App, Arg};
use std::fs::copy;
use std::path::Path;
use std::process::exit;

pub fn senderr(e: String, ec: i32) {
    eprintln!("Copying aborted...\n\n{}", Red.bold().paint(e));
    exit(ec);
}

fn main() {
    let cli = App::new("cn")
        .version("0.1.0")
        .author("Arijit Dey <arijid79@gmail.com>")
        .about("Copy SOURCE to DESTINATION")
        .arg(
            Arg::with_name("source")
                .takes_value(true)
                .value_name("SOURCE")
                .help("The paths that needs to be copied")
                .required(true),
        )
        .arg(
            Arg::with_name("dest")
                .takes_value(true)
                .value_name("DESTINATION")
                .help("The directory where the files need to be placed")
                .required(true),
        )
        .get_matches();

    let source = Path::new(cli.value_of("source").unwrap()).to_owned();
    let mut dest = Path::new(cli.value_of("dest").unwrap()).to_owned();

    if Path::new(&dest).is_dir() {
        dest.push(&source.file_name().unwrap().to_str().unwrap());
    }

    checks::check_all(&source, &dest);
    let r = copy(source.to_str().unwrap(), dest.to_str().unwrap());

    if r.is_err() {
        senderr(format!("'{}' Permission denied", dest.display()), 2)
    }
}
