use clap::{App, Arg};
use std::fs::copy;
use std::path::Path;

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

    let r = copy(source.to_str().unwrap(), dest.to_str().unwrap());

    match r {
        Ok(_) => println!("{} -> {}", source.display(), dest.display()),
        Err(e) => eprintln!("Copying aborted...\n\n{}", e),
    }
}
