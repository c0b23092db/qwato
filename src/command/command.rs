use crate::config::Config;
use anyhow::Result;
use colored::Colorize;

/// Fomat: Command一覧
/// argsがあれば詳細を表示
pub fn format_command_list(config: &Config, args: &[String]) -> Result<()> {
    if args.is_empty() {
        for command in &config.command {
            println!("{}", command.0);
        }
    } else {
        for command in args {
            if let Some(command_config) = config.command.get(command) {
                println!("{}", command.bold());
                if let Some(directory) = &command_config.directory {
                    println!("  Directory: {}", directory.display());
                }
                if let Some(file) = &command_config.file {
                    println!("  File: {}", file.display());
                }
                if let Some(template) = &command_config.template {
                    println!("  Template: {}", template.display());
                }
                if let Some(tags) = &command_config.tags {
                    println!("  Tags: {}", tags.join(", "));
                }
            }
        }
    }
    Ok(())
}
