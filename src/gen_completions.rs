use crate::utils::senderr;
use async_std::path::PathBuf;
use clap::{App, Shell};
use std::io::stdout;
use std::fs::File;
use std::process::exit;

#[derive(PartialEq)]
pub enum Output {
    File(PathBuf),
    Stdout,
}

pub fn write_completions(output: Output, shell: Shell, mut app: App<'static, 'static>) {
    match output {
        Output::File(p) => {
            let rfile = File::create(&p);
            if let Err(e) = rfile {
                senderr(format!("Unable to create file {}\n{:?}", p.display(), e.kind()));
                exit(crate::STATUS_ERR);
            } else {
                app.gen_completions_to("cn", shell, &mut rfile.unwrap());
            }
        },
        Output::Stdout => {
            app.gen_completions_to("cn", shell, &mut stdout());
        }
    }
}

pub fn generate_completions(app: App<'static, 'static>) {
    let app_clone = app.clone();
    let argmatches = app.get_matches();
    let matches = argmatches.subcommand_matches("completion").unwrap();

    let shell = matches.value_of("shell").unwrap();
    
    let output = if matches.is_present("output") {
        Output::File(PathBuf::from(matches.value_of("output").unwrap()))
    } else {
        Output::Stdout
    };

    match shell.to_ascii_lowercase().as_str() {
        "bash" => write_completions(output, Shell::Bash, app_clone),
        "zsh" => write_completions(output, Shell::Zsh, app_clone),
        "fish" => write_completions(output, Shell::Fish, app_clone),
        "powershell" => write_completions(output, Shell::PowerShell, app_clone),
        "elvish" => write_completions(output, Shell::Elvish, app_clone),
        _ => {
            senderr("The given shell is not supported for autocompletions");
            std::process::exit(crate::STATUS_ERR);
        }
    }
}
