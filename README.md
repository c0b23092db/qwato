# Qwato (くぁと)
This English version was translated from the official Japanese documentation with the assistance of Perplexity AI.

日本語/Japanese　documentation: [README-ja.md](./README/README-ja.md)

```bash
qwa
```

**A command-line tool for adding memos to specified files.**

Qwato is intended for users who write in daily notes with Obsidian QuickAdd Capture. Enjoy the convenience of writing from the terminal with a command, without having to launch Obsidian.

## ⭐ Features ⭐

- One-line appending that reproduces Obsidian QuickAdd Capture
- Timeline display that reproduces Obsidian Thino
- A journaling tool primarily based on one page per day

## 💻 Supported Environments 💻

### Operating Systems

#### Tested

- [x] Linux
- [x] Windows 11

#### Not tested

- [ ] macOS

## 📦 Installation 📦

### Binary

#### Windows

[Download qwa.exe v0.2.0](https://github.com/c0b23092db/qwato/releases/download/v0.2.0/qwa.exe)

### cargo

#### cargo install

```bash
cargo install qwato
```

#### cargo install --git

```bash
cargo install --git https://github.com/c0b23092db/qwato
```

#### cargo binstall

```bash
cargo binstall qwato
```

## 📖 Commands 📖

```text
> qwa --help

CLI: Write a Memo for your notes in the terminal.

Usage: qwa.exe [OPTIONS] [arguments]...

Arguments:
  [arguments]...  Additional arguments

Options:
  -a, --add                   Add a new message
  -c, --checkbox              Add a new checkbox
  -l, --list                  List all commands
  -n, --note                  List all notes
  -t, --task                  List all tasks
      --all                   List all messages, including messages without a time format
      --tag <TAG>...          Tags for the message
      --limit <LIMIT>         Show the limit of list messages
      --command               Check to Use Command
      --config <config_path>  Config File Path
      --utc-offset-time       Debug: Show UTC Offset Time
      --colon-sharp-question  Debug: Show Load Config File
  -h, --help                  Print help
  -V, --version               Print version
```

### Argument conventions

#### `--add` / `--checkbox`

The first argument is treated as a command, and the second and subsequent arguments are treated as the message.

```text
<command> <message>
```

#### Other options

All arguments are treated as commands.

```text
<command> <command>
```

### Text input

#### `--add`

```bash
qwa <message>...
qwa <command> <message> <message>...
qwa --add <message>...
qwa --add <command> <message> <message>...
```

Appends a memo to the specified file. All arguments from the second argument onward are treated as one line.

#### `--checkbox`

```bash
qwa --checkbox <message>...
qwa --checkbox <command> <message> <message>...
```

Appends a memo with a checkbox to the specified file. All arguments from the second argument onward are treated as one line.

### List display

#### `--list`

```bash
qwa --list
qwa --list <command> <command>...
```

Displays list items and checkboxes.

#### `--note`

```bash
qwa --note
qwa --note <command> <command>...
```

Displays list items.

#### `--task`

```bash
qwa --task
qwa --task <command> <command>...
```

Displays checkboxes.

#### `--all`

```bash
qwa --all
```

Displays all entries, including list items without a time format.

### Behavior modifiers

#### `--tag`

```bash
qwa --tag <TAG>...
```

Specify tags separated by commas, such as `tag1,tag2,tag3`.

```bash
qwa --add <command>... --tag <TAG>...
qwa --checkbox <command>... --tag <TAG>...
```

The specified tags are added together with the tags registered for the command.

```bash
qwa --list --tag <TAG>...
qwa --note --tag <TAG>...
qwa --task --tag <TAG>...
qwa --all --tag <TAG>...
```

Searches for and displays entries containing any of the specified tags.

#### `--limit`

```bash
qwa --list --limit 10
```

Changes the number of entries displayed.

#### `--config`

```bash
qwa --config <config_path>
```

Specifies the configuration file to load.

### Configuration display

#### `--command`

```bash
qwa --command
```

Displays a simple list of available commands.

```bash
qwa --command <command> <command>...
```

Displays detailed information about the specified commands.

### Debug output

#### `--colon-sharp-question`

```bash
qwa --colon-sharp-question
```

Displays the loaded configuration in the `"{:#?}"` format.

#### `--utc-offset-time`

```bash
qwa --utc-offset-time
```

Returns the time obtained when the command was executed.

```text
2026-08-21 00:15:55.451988700 +09:00
```

### `--help`

```bash
qwa --help
```

Displays command help.

### `--version`

```bash
qwa --version
```

Displays the command version.

## ⚙ Configuration File ⚙

**Notes**

- Configuration files are loaded in the following order of priority: the specified configuration file, `./qwato.toml`, then `~/.config/qwato/config.toml`.
- Dates use the Rust [chrono](https://docs.rs/chrono) crate.

### Default configuration

```toml
base_directory = "~/Documents/Qwato"
default_command = "default"
time_format = "%H:%M:%S"

[list]
limit = 10

[created]
format = "%Y-%m-%d %H:%M:%S"

[modified]
format = "%Y-%m-%d %H:%M:%S"

[command.default]
auto_create = true
file = "%Y-%m-%d.md"
end_line = false
not_format = false
```

### Configuration options

#### **General**

##### `base_directory`

The directory used as the storage root.

If omitted, the home directory is used.

##### `default_command`

Specifies the default command used when no command is provided.

If omitted, a command is always required as the first argument.

##### `no_global`

If `true`, the configuration is excluded from global loading.

The configuration file with the highest priority is always loaded.

##### `auto_command`

If `true`, `default_command` is always executed forcibly.

##### `time_format`

The chrono-format string inserted at the beginning of list items.

#### **list**

##### `limit`

Specifies the number of list items to display.

#### **created**

If the Markdown file contains YAML frontmatter and the specified field exists, it is updated with the file creation date.

##### `field`

The field name in which to store the file creation date.

If omitted, the field is not updated automatically.

##### `format`

The chrono-format string used for the field value.

#### **modified**

If the Markdown file contains YAML frontmatter and the specified field exists, it is updated with the file modification time.

##### `field`

The field name in which to store the file modification date.

If omitted, the field is not updated automatically.

##### `format`

The chrono-format string used for the field value.

#### **command**

##### `auto_create`

Determines whether the file should be created if it does not exist.

##### `template`

The path of the source file to copy when creating a new file.

##### `directory`

Specifies the directory path relative to `base_directory`. If omitted, `base_directory` itself is used.

The chrono format is supported.

##### `file`

Specifies the file name.

The chrono format is supported.

##### `insert`

The chrono format is supported.

Specifies the line used as the insertion reference. The line is matched exactly.

If `insert` is configured but the specified line does not exist, an error is returned.

- If `end_line` is `true`, the entry is added to the end of the section.
- If `end_line` is `false`, the entry is added to the beginning of the section.

If `insert` is not configured:

- If `end_line` is `true`, the entry is added to the end of the file.
- If `end_line` is `false`, the entry is added to the beginning of the body, after the frontmatter.

##### `tags`

```toml
tags = ["tag1", "tag2"]
```

Specifies tags. To specify multiple tags, provide them as a comma-separated list.

##### `not_format`

- `false`: Inserts the time.
- `true`: Outputs list items only.

##### `end_line`

- `false`: Adds the entry to the beginning.
- `true`: Adds the entry to the end.

### Configuration examples

#### Integration with Obsidian

To integrate with Obsidian, set `base_directory` to your Obsidian Vault.

Then configure `template`, `directory`, and `file` under `command.daily` to use it like an Obsidian Daily Note.

To display a list like Thino, use `--list` to reproduce an Obsidian Thino-like view.

```toml
base_directory = "~/Documents/Obsidian"
default_command = "daily"
time_format = "%H:%M:%S"

[created]
field = "createdAt"
format = "%Y-%m-%d %H:%M:%S"

[modified]
field = "updatedAt"
format = "%Y-%m-%d %H:%M:%S"

[command.daily]
template = "Template/daily.md"
auto_create = true
directory = "daily"
file = "%Y-%m-%d.md"
insert = "## Journal"
end_line = false
```

#### Using Qwato for memos

To append to a single file such as `inbox.md`, specify the file with `file`.

```toml
[command.memo]
auto_create = true
file = "inbox.md"
end_line = false
not_format = true
```

If you are searching for software, adding `Software` to `tags` makes it easier to search.

Using `--all` displays list items without a time format.

```toml
[command.memo]
auto_create = true
file = "inbox.md"
tags = ["Software"]
not_format = true
```

#### Requiring a command

If `default_command` is not configured, Qwato requires a command.

```toml
base_directory = "~/Documents/Obsidian"
# default_command = "daily"
```

If you run Qwato without entering a command, it displays `Not Config: defualt_command`.

## 📰 Future Plans 📰

See [開発案.md](./README/開発案.md) for future development plans.

**Qwato is developed for its creator, who uses Obsidian, so maintenance is intentionally limited.**

## 💡 Inspiration 💡

- [Obsidian](https://obsidian.md/): Maintaining a journal
- [QuickAdd](https://community.obsidian.md/plugins/quickadd): Simple commands
- [Thino](https://community.obsidian.md/plugins/obsidian-memos): Timeline display

## 🔌 Debugging 🔌

- **Set `no_global = true` to disable loading the global configuration.**

## 📜 LICENSE 📜

[Apache License Version 2.0](./LICENSE) / http://www.apache.org/licenses/LICENSE-2.0