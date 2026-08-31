# Qwato (くぁと)
This English version was translated from the official Japanese documentation with the assistance of Perplexity AI.

日本語 / Japanese documentation: [README-ja.md](./README-ja.md)

```bash
qwa
```

**A command-line tool for adding memos to specified files.**

Qwato is intended for users who write daily notes with Obsidian QuickAdd Capture. Enjoy the convenience of writing from the terminal without launching Obsidian.

## ⭐ Features ⭐
- A journaling tool primarily based on one page per day.
- One-line appending inspired by Obsidian QuickAdd Capture.
- Timeline-style display inspired by Obsidian Thino.

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

### Cargo

### Binary
- [Windows - zip](https://github.com/c0b23092db/qwato/releases/download/v0.3.0/qwa-windows-x86_64.zip)
- [Windows - exe](https://github.com/c0b23092db/qwato/releases/download/v0.3.0/qwa.exe)
- [Linux - tar.gz](https://github.com/c0b23092db/qwato/releases/download/v0.3.0/qwa-linux-x86_64.tar.gz)
- [Mac - tar.gz](https://github.com/c0b23092db/qwato/releases/download/v0.3.0/qwa-macos-x86_64.tar.gz)

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

### eget

```bash
eget c0b23092db/qwato
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
      --last-edit             Edit the last inserted memo
  -s, --summary               Show summary of messages updated today
  -l, --list                  List all commands
  -n, --note                  List all notes
  -t, --task                  List all tasks
      --all                   List all messages, including messages without a time format
      --tag <TAG>...          Tags for the message
      --link <LINK>           Link for the message
      --from <YYYY-MM-DD>     Filter from date (inclusive, format: YYYY-MM-DD)
      --to <YYYY-MM-DD>       Filter to date (inclusive, format: YYYY-MM-DD)
      --limit <LIMIT>         Show the limit of list messages
      --command               Check to Use Command
      --config <path>         Config File Path
  -h, --help                  Print help
  -V, --version               Print version
```

### Argument conventions

#### `--add` / `--checkbox`

The first argument is treated as a command, and the second and subsequent arguments are treated as the message.

```bash
[command] [message] [message] ...
```

#### Other options

All arguments are treated as commands.

```bash
[command] [command] ...
```

### Text input

#### `--add`

```bash
qwa [message] ...
qwa [command] [message] [message] ...
qwa --add [message] ...
qwa --add [command] [message] [message] ...
```

Appends a memo to the specified file. All arguments from the second argument onward are treated as one line.

#### `--checkbox`

```bash
qwa --checkbox [message] ...
qwa --checkbox [command] [message] [message] ...
```

Appends a memo with a checkbox to the specified file. All arguments from the second argument onward are treated as one line.

#### `--last-edit`

```bash
qwa --last-edit [message] ...
```

Edits the last memo that was appended. All arguments from the second argument onward are treated as one line.

### List display

#### `--summary`

```bash
qwa --summary
```

Displays a summary of memos updated today.

#### `--list`

```bash
qwa --list
qwa --list [command] [command] ...
```

Displays list items and checkboxes.

#### `--note`

```bash
qwa --note
qwa --note [command] [command] ...
```

Displays list items.

#### `--task`

```bash
qwa --task
qwa --task [command] [command] ...
```

Displays checkboxes.

#### `--all`

```bash
qwa --all
```

Displays all entries, including list items without a time format.

### Behavior modifiers

#### `--from` / `--to`

```bash
qwa --list --from <YYYY-MM-DD> --to <YYYY-MM-DD>
```

Displays entries within the specified period. `--from` includes the specified date and later; `--to` includes the specified date and earlier.

#### `--tag` / `--tags`

```bash
qwa --tag <TAG>
```

Specify tags separated by commas, such as `tag1,tag2,tag3`. `--tags` can also be used as an alias.

```bash
qwa --add [message] --tag <TAG>...
qwa --checkbox [message] --tag <TAG>...
```

The specified tags are added together with the tags registered for the command.

```bash
qwa --list --tag <TAG>...
qwa --note --tag <TAG>...
qwa --task --tag <TAG>...
qwa --all --tag <TAG>...
```

Searches for and displays entries containing any of the specified tags.

#### `--link`

```bash
qwa --link <LINK>
```

Adds a link to the memo.

#### `--limit`

```bash
qwa --list --limit 10
```

Changes the number of entries displayed.

#### `--config`

```bash
qwa --config <path>
```

Specifies the configuration file to load. If a directory is specified, Qwato loads the configuration files contained in that directory.

### Configuration display

#### `--command`

```bash
qwa --command
```

Displays a simple list of available commands.

```bash
qwa --command [command] [command] ...
```

Displays detailed information about the specified commands.

### Debug output

#### `--utc-offset-time`

```bash
qwa --utc-offset-time
```

Returns the time obtained when the command was executed.

```text
2026-08-21 00:15:55.451988700 +09:00
```

#### `--colon-sharp-question`

```bash
qwa --colon-sharp-question
```

Displays the loaded configuration in the `"{:#?}"` format.

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

- Configuration files are loaded in the following priority order: the specified configuration file, `./qwato.toml`, then TOML files in `~/.config/qwato`.
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

#### General

##### `base_directory`

The storage-root directory. If omitted, the home directory is used.

##### `default_command`

Specifies the command used by default when no command is provided. If omitted, a command is always required as the first argument.

##### `no_global`

If `true`, this configuration is excluded from global loading. The highest-priority configuration file is always loaded.

##### `auto_command`

If `true`, `default_command` is always executed forcibly.

##### `time_format`

The chrono-format string inserted at the beginning of list items.

##### `data_format`

Specifies the format used to locate files. The available formats are:

- `line`: `YYYY-MM-DD.md`
- `slash`: `YYYY/MM/DD.md`
- `slash_line`: `YYYY/MM/DD/YYYY-MM-DD.md`
- `header`: `file-name.md / Header for YYYY-MM-DD`
- `onefile`: `file-name.md`

#### `list`

##### `limit`

Specifies the number of list items to display.

#### `created`

If a Markdown file contains YAML frontmatter and the specified field exists, it is updated with the file creation date.

##### `field`

The field name in which to store the file creation date. If omitted, the field is not updated automatically.

##### `format`

The chrono-format string used for the field value.

#### `modified`

If a Markdown file contains YAML frontmatter and the specified field exists, it is updated with the file modification time.

##### `field`

The field name in which to store the file modification time. If omitted, the field is not updated automatically.

##### `format`

The chrono-format string used for the field value.

#### `command`

##### `auto_create`

Determines whether the file should be created if it does not exist.

##### `date_format`

Specifies the file format, overriding the global setting. Available formats are `line`, `slash`, `slash_line`, `header`, and `onefile` as described above.

##### `template`

The path of the source file to copy when creating a new file.

##### `directory`

Specifies the directory path relative to `base_directory`. If omitted, `base_directory` itself is used. Chrono formatting is supported.

##### `file`

Specifies the file name. Chrono formatting is supported.

##### `insert`

Specifies the exact line used as the insertion reference. Chrono formatting is supported. If the configured line does not exist, an error is returned.

- If `end_line` is `true`, the entry is added to the end of the section.
- If `end_line` is `false`, the entry is added to the beginning of the section.

If `insert` is not configured:

- If `end_line` is `true`, the entry is added to the end of the file.
- If `end_line` is `false`, the entry is added to the beginning of the body, after the frontmatter.

##### `tags`

```toml
tags = ["tag1", "tag2"]
```

Specifies tags. Multiple tags can be specified as a comma-separated list.

##### `not_format`

- `false`: Inserts the time.
- `true`: Outputs list items only.

##### `end_line`

- `false`: Adds the entry to the beginning.
- `true`: Adds the entry to the end.

### Configuration examples

#### Integration with Obsidian

Set `base_directory` to your Obsidian Vault. Then configure `template`, `directory`, and `file` under `command.daily` to use Qwato like an Obsidian Daily Note.

To display entries like Thino, use `--list`.

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

To append to a single file such as `inbox.md`, specify `file`.

```toml
[command.memo]
auto_create = true
file = "inbox.md"
end_line = false
not_format = true
```

Adding `Software` to `tags` can make software-related entries easier to find. Use `--all` to display list items without a time format.

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

Running Qwato without a command displays `Not Config: defualt_command`.

## 📰 Future Plans 📰

See [開発案.md](./開発案.md) for future development plans.

**Qwato is developed for its creator, who uses Obsidian, so maintenance is intentionally limited.**

## 💡 Inspiration 💡

- [Obsidian](https://obsidian.md/): Maintaining a journal.
- [QuickAdd](https://community.obsidian.md/plugins/quickadd): Simple commands.
- [Thino](https://community.obsidian.md/plugins/obsidian-memos): Timeline display.

## 🔌 Debugging 🔌

- **Set `no_global = true` to disable loading the global configuration.**

## 📜 LICENSE 📜

[Apache License Version 2.0](./LICENSE) / http://www.apache.org/licenses/LICENSE-2.0
