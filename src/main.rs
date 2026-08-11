mod config;
use config::Config;
use config::load_config;
mod command;
use command::add::append_message;
use command::list::list_entries;
mod tool;
mod utils;

use anyhow::{Result, anyhow};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    arg_required_else_help = false,
    allow_negative_numbers = true
)]
struct Args {
    /// Add a new message
    #[arg(short, long)]
    add: bool,
    /// Add a new checkbox
    #[arg(short, long)]
    check: bool,
    /// List all commands
    #[arg(short, long)]
    list: bool,
    /// List all notes
    #[arg(short, long)]
    note: bool,
    /// List all tasks
    #[arg(short, long)]
    task: bool,
    /// Show the limit of list messages
    #[arg(long)]
    limit: Option<usize>,
    /// Check to Load Config File
    #[arg(long)]
    load: bool,
    /// Additional arguments
    #[arg(value_name = "arguments")]
    argument: Vec<String>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();
    let mut config = load_config()?;
    config.set_limit(
        args.limit
            .unwrap_or(config.list.as_ref().map(|list| list.limit).unwrap_or(10)),
    );
    match args {
        args if args.list => {
            return list_entries(&config, &args.argument, false, false);
        }
        args if args.note || args.task => {
            return list_entries(&config, &args.argument, args.note, args.task);
        }
        args if args.load => {
            println!("{:#?}", config);
            return Ok(());
        }
        _ => {
            // args if args.add || args.check (Add,Check)
            let (command, message) = check_argument_count(&config, &args)?;
            return append_message(&config, &command, &message, args.check);
        }
    }
}

/// Check: 引数の数
/// Returns: (command:String, message:String)
fn check_argument_count(config: &Config, args: &Args) -> Result<(String, String)> {
    if args.argument.is_empty() {
        return Err(anyhow!("No message provided"));
    } else if config.auto_command || args.argument.len() == 1 {
        let command = config.default_command.clone();
        let message = args.argument[0].clone();
        Ok((command, message))
    } else {
        let command = args.argument[0].clone();
        let message = args.argument[1..].join(" ");
        Ok((command, message))
    }
}
