mod daily;

use crate::daily::add_line;
use clap::{crate_description, crate_name, crate_version, App, Arg, ArgSettings};
use clap_generate::{
    generate,
    generators::{Bash, Zsh},
};
use simple_logger::SimpleLogger;
use std::io;

fn main() {
    let crate_name = crate_name!();
    let mut app = App::new(crate_name)
        .about(crate_description!())
        .version(crate_version!())
        .arg(
            Arg::new("verbose")
                .short('v')
                .about("Change verbosity level (multiple, default to info)")
                .setting(ArgSettings::MultipleOccurrences),
        )
        .subcommand(
            App::new("completion")
                .about("Because a CLI without completion is like cheese without bread")
                .arg(
                    Arg::new("shell")
                        .short('s')
                        .long("shell")
                        .required(true)
                        .possible_values(&["bash", "zsh"]),
                ),
        )
        .subcommand(
            App::new("add").about("Add a line").arg(
                Arg::new("text")
                    .takes_value(true)
                    .about("add a text line to your daily notes"),
            ),
        );
    let matches = &app.get_matches_mut();
    let level = match matches.occurrences_of("verbose") {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    SimpleLogger::new().with_level(level).init().unwrap();
    if let Some(ref matches) = matches.subcommand_matches("completion") {
        match matches.value_of("shell") {
            Some("bash") => generate::<Bash, _>(&mut app, crate_name, &mut io::stdout()),
            Some("zsh") => generate::<Zsh, _>(&mut app, crate_name, &mut io::stdout()),
            Some(shell) => log::error!("Unknown shell {}", shell),
            None => log::error!("A shell is needed for completion"),
        }
    } else if let Some(ref matches) = matches.subcommand_matches("add") {
        match matches.value_of("text") {
            Some(text) => match add_line(text) {
                Ok(_) => {}
                Err(err) => {
                    log::error!("Could not write the daily line: {}", err)
                }
            },
            None => log::debug!("No text provided"),
        }
    } else {
        app.print_help().unwrap();
    }
}
