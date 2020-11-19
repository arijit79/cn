use crate::utils::senderr;
use async_std::path::PathBuf;
use clap::{App, Shell};
use std::fs::File;
use std::io::stdout;
use std::process::exit;

// Types of output
#[derive(PartialEq)]
pub enum Output {
    File(PathBuf),
    Stdout,
}

// Write the completion data for the shell to the output
pub fn write_completions(output: Output, shell: Shell, mut app: App<'static>) {
    match output {
        Output::File(p) => {
            // Check if the file could be written
            let rfile = File::create(&p);
            if let Err(e) = rfile {
                senderr(format!(
                    "Unable to create file {}\n{:?}",
                    p.display(),
                    e.kind()
                ));
                exit(crate::STATUS_ERR);
            } else {
                // Generate the completions and write to the file
                app.gen_completions_to("cn", shell, &mut rfile.unwrap());
            }
        }
        Output::Stdout => {
            // Generate the completions and write to stdout
            app.gen_completions_to("cn", shell, &mut stdout());
        }
    }
}

pub fn generate_completions(app: App<'static>) {
    // Clone the app here, since `get_matches()` will consume it
    let app_clone = app.clone();
    let argmatches = app.get_matches();
    // Get all matches in the subcommand
    let matches = argmatches.subcommand_matches("completion").unwrap();
    // Get the shell and output
    let shell = matches.value_of("shell").unwrap();
    let output = if matches.is_present("output") {
        // if output is presnt, make a PathBuf from it
        Output::File(PathBuf::from(matches.value_of("output").unwrap()))
    } else {
        // Else take the stdout into account
        Output::Stdout
    };

    // Match the correct shell and write the completions for it
    // Make sure to convert the shell into lowercase &str,
    // so that we can accept any case
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
