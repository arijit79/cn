use clap::{App, Arg};

fn main() {
    let cli = App::new("cn")
                .version("0.1.0")
                .author("Arijit Dey <arijid79@gmail.com>")
                .about("Copy SOURCE to DESTINATION")
                .arg(Arg::with_name("source").takes_value(true)
                    .value_name("SOURCE")
                    .help("The paths that needs to be copied")
                    .required(true))
                .arg(Arg::with_name("dest").takes_value(true)
                    .value_name("DESTINATION")
                    .help("The directory where the files need to be placed")
                    .required(true)
            )
                .get_matches();
}
