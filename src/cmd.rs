use clap::ArgMatches;
use clap::{command, value_parser, Arg, ArgAction, Command};

#[derive(Debug)]
pub struct RemoveArgs {
    files: Vec<String>,
    dry_run: bool,
}

#[derive(Debug)]
pub struct EmptyArgs {
    matches: ArgMatches,
}

#[derive(Debug)]
pub enum Cmd {
    Remove { args: RemoveArgs },
    Empty { args: EmptyArgs },
}

impl Cmd {
    pub fn parse() -> Cmd {
        let matches = command!() // requires `cargo` feature
            .arg(
                Arg::new("files")
                    .action(ArgAction::Append)
                    .value_parser(value_parser!(String)),
            )
            .arg(
                Arg::new("dry")
                    .long("dry")
                    .action(ArgAction::SetTrue)
                    .help("Dry run printing what will be moved before moving"),
            )
            .subcommand(Command::new("empty").about("Empty trash"))
            .get_matches();

        if let Some(_) = matches.subcommand_matches("empty") {
            Cmd::Empty {
                args: EmptyArgs { matches },
            }
        } else {
            Cmd::Remove {
                args: RemoveArgs::parse(&matches),
            }
        }
    }
}

impl RemoveArgs {
    fn parse(matches: &ArgMatches) -> RemoveArgs {
        RemoveArgs {
            dry_run: matches.get_flag("dry"),
            files: matches
                .get_many("files")
                .expect("expected files to delete")
                .cloned()
                .collect(),
        }
    }

    pub fn dry_run(&self) -> bool {
        self.dry_run
    }

    pub fn files(&self) -> &Vec<String> {
        &self.files
    }
}
