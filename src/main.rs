mod command;
mod config;
mod utils;
use command::add::append_message;
use command::command::format_command_list;
use command::last_edit::edit_last_message;
use command::list::list_entries;
use config::Config;
use config::load_config;

use anyhow::{Context, Result, anyhow};
use chrono::{Local, NaiveDate};
use clap::Parser;
use std::path::PathBuf;

fn parse_date(s: &str) -> std::result::Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(s, "%Y/%m/%d"))
        .map_err(|e| format!("Invalid date format '{}' (expected YYYY-MM-DD): {}", s, e))
}

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
    /// Edit the last inserted memo
    #[arg(long, value_name = "message")]
    last_edit: bool,
    /// Show summary of messages updated today
    #[arg(short, long)]
    summary: bool,
    /// List all commands
    #[arg(short, long)]
    list: bool,
    /// List all notes
    #[arg(short, long)]
    note: bool,
    /// List all tasks
    #[arg(short, long)]
    task: bool,
    /// List all messages, including messages without a time format
    #[arg(long)]
    all: bool,
    /// Tags for the message
    #[arg(long, alias = "tags", value_delimiter = ',', num_args = 1..)]
    tag: Vec<String>,
    /// Link for the message
    #[arg(long)]
    link: Option<String>,
    /// Filter from date (inclusive, format: YYYY-MM-DD)
    #[arg(long, value_name = "YYYY-MM-DD", value_parser = parse_date)]
    from: Option<NaiveDate>,
    /// Filter to date (inclusive, format: YYYY-MM-DD)
    #[arg(long, value_name = "YYYY-MM-DD", value_parser = parse_date)]
    to: Option<NaiveDate>,
    /// Show the limit of list messages
    #[arg(long)]
    limit: Option<usize>,
    /// Check to Use Command
    #[arg(long)]
    command: bool,
    /// Config File Path
    #[arg(long, value_name = "path", value_parser)]
    config: Option<PathBuf>,
    /// Debug: Show UTC Offset Time
    #[arg(long, hide = true)]
    utc_offset_time: bool,
    /// Debug: Show Load Config File
    #[arg(long, hide = true)]
    colon_sharp_question: bool,
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
    let mut config = load_config(&args.config)?;
    config.set_limit(
        args.limit
            .unwrap_or(config.list.as_ref().map(|list| list.limit).unwrap_or(10)),
    );
    match args {
        args if args.colon_sharp_question => {
            println!("{:#?}", config);
            Ok(())
        }
        args if args.command => format_command_list(&config, &args.argument),
        args if args.summary => {
            let today = Local::now().date_naive();
            let _ = list_entries(
                &config,
                &args.argument,
                &args.tag,
                args.note,
                args.task,
                false,
                Some(today),
                Some(today),
            );
            Ok(())
        }
        args if args.list
            || args.all
            || args.note
            || args.task
            || args.from.is_some()
            || args.to.is_some() =>
        {
            list_entries(
                &config,
                &args.argument,
                &args.tag,
                args.note,
                args.task,
                args.all,
                args.from,
                args.to,
            )
        }
        args if args.last_edit => {
            let (command, message) = check_argument_count(&config, &args.argument)?;
            edit_last_message(&config, &command, &message, args.link.as_deref(), &args.tag)
        }
        _ => {
            // args if args.add || args.checkbox (Add,Checkbox)
            let (command, message) = check_argument_count(&config, &args.argument)?;
            append_message(
                &config,
                &command,
                &message,
                args.link.as_deref(),
                &args.tag,
                args.checkbox,
                args.utc_offset_time,
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
