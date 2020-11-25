use crate::utils::senderr;
use async_std::path::PathBuf;
use clap::Shell;
use std::fs::File;
use std::io::stdout;
use std::process::exit;
use crate::args::Cli;

// Types of output
#[derive(PartialEq)]
pub enum Output {
    File(PathBuf),
    Stdout,
}

// Write the completion data for the shell to the output
pub fn write_completions(output: Output, shell: Shell, mut app: Cli) {
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

pub fn generate_completions(cli: Cli) {
    let argmatches = cli.subcmd.unwrap();
    let crate::args::SubCommand::Completion(args) = argmatches;
    // Get the shell and output
    let shell = args.shell;
    let output = if args.output.is_some() {
        // if output is presnt, make a PathBuf from it
        Output::File(PathBuf::from(args.output.unwrap()))
    } else {
        // Else take the stdout into account
        Output::Stdout
    };

    // Match the correct shell and write the completions for it
    // Make sure to convert the shell into lowercase &str,
    // so that we can accept any case
    match shell.to_ascii_lowercase().as_str() {
        "bash" => write_completions(output, Shell::Bash, cli),
        "zsh" => write_completions(output, Shell::Zsh, cli),
        "fish" => write_completions(output, Shell::Fish, cli),
        "powershell" => write_completions(output, Shell::PowerShell, cli),
        "elvish" => write_completions(output, Shell::Elvish, cli),
        _ => {
            senderr("The given shell is not supported for autocompletions");
            std::process::exit(crate::STATUS_ERR);
        }
    }
}
