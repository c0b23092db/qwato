mod config;
use config::Config;
use config::load_config;
mod command;
use command::add::append_message;
use command::command::format_command_list;
use command::list::list_entries;
mod tool;
mod utils;

use anyhow::{Context, Result, anyhow};
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
    checkbox: bool,
    /// Tags for the message
    #[arg(long)]
    tag: Option<String>,
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
    /// Check to Use Command
    #[arg(long)]
    command: bool,
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
        args if args.load => {
            println!("{:#?}", config);
            Ok(())
        }
        args if args.command => format_command_list(&config, &args.argument),
        args if args.list => list_entries(&config, &args.argument, false, false),
        args if args.note || args.task => {
            list_entries(&config, &args.argument, args.note, args.task)
        }
        _ => {
            // args if args.add || args.checkbox (Add,Checkbox)
            let (command, message) = check_argument_count(&config, &args.argument)?;
            append_message(
                &config,
                &command,
                &message,
                args.tag.as_deref(),
                args.checkbox,
            )
        }
    }
}

/// Check: 引数の数
/// Returns: (command:String, message:String)
fn check_argument_count(config: &Config, argument: &[String]) -> Result<(String, String)> {
    if argument.is_empty() {
        Err(anyhow!("No message provided"))
    } else if config.auto_command || argument.len() == 1 {
        let command = config
            .default_command
            .clone()
            .with_context(|| "Not Config: defualt_command")?;
        let message = argument[0].clone();
        Ok((command, message))
    } else {
        let command = argument[0].clone();
        let message = argument[1..].join(" ");
        Ok((command, message))
    }
}
