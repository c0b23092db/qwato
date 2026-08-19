# Qwato (くぁと)
日本語: [README-ja.md](./README/README-ja.md)

```bash
qwa
```

A command-line tool for adding memos to specified files.

## ⭐ Features ⭐

- Reproduces Captures that can be registered with Obsidian's QuickAdd.
- Displays a timeline similar to Obsidian's Thino.

## 💻 Supported Environments 💻

### Operating Systems

#### Tested

- [x] Windows 11

#### Not Tested

- [ ] Linux
- [ ] Mac

## 📦 Installation 📦

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

```
> qwa --help

CLI: Write a Memo for your notes in the terminal.

Usage: qwa.exe [OPTIONS] [arguments]...

Arguments:
  [arguments]...  Additional arguments

Options:
  -a, --add            Add a new message
  -c, --check          Add a new checkbox
  -l, --list           List all commands
  -n, --note           List all notes
  -t, --task           List all tasks
      --all            List all messages, including messages without a time format
      --limit <LIMIT>  Show the limit of list messages
      --load           Check to Load Config File
  -h, --help           Print help
  -V, --version        Print version
```

### `--add`

Appends a memo to the specified file.
All arguments from the second argument onward are processed as a single line.

```bash
qwa <message>
qwa <command> <message>
qwa --add <message>
qwa --add <command> <message>
```

### `--check`

Appends a memo with a checkbox to the specified file.
All arguments from the second argument onward are processed as a single line.

```bash
qwa --check <message>
qwa --check <command> <message>
```

### `--list`

```bash
qwa --list
qwa --list <command> <command> ...
```

Reads the files in the directories specified by the commands and displays both list items and checkboxes.

### `--note`

```bash
qwa --note
qwa --note <command> <command> ...
```

Reads the files in the directories specified by the commands and displays list items.

### `--task`

```bash
qwa --task
qwa --task <command> <command> ...
```

Reads the files in the directories specified by the commands and displays checkboxes.

### `--limit`

```bash
qwa --list --limit 10
```

Specifies the number of items to display.

### `--all`

```bash
qwa --all
```

Displays items including those without a time format.

### `--load`

```bash
qwa --load
```

Displays the configuration file in Rust's `"{:#?}"` format.

### `--help`

```bash
qwa --help
```

Displays the command help.

### `--version`

```bash
qwa --version
```

Displays the command version.

## ⚙ Configuration File ⚙

**Notes**

- On every operating system, Qwato uses `~/.config/qwato`.
- Dates are handled using Rust's [chrono](https://docs.rs/chrono) crate.

### Default Configuration

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

### Configuration Options

#### **General**

##### `base_directory`

The base directory.

##### `default_command`

Specifies the command used by default when no command is provided.

##### `auto_command`

- `true`: Forces `default_command` to be executed.

##### `time_format`

The format inserted at the beginning of each list item.
The `chrono` format is supported.

#### **list**

##### `limit`

Only applies to `--list`, `--note`, and `--task`.
Specifies the number of list items to display.

#### **created**

If the Markdown file contains YAML frontmatter and the specified field exists, updates it with the file's creation date.

##### `field`

The field in which the file's creation date is stored.
If omitted, the field is not updated automatically.

##### `format`

The format of the value written to the field.
The `chrono` format is supported.

#### **modified**

If the Markdown file contains YAML frontmatter and the specified field exists, updates it with the file's modification time.

##### `field`

The field in which the file's modification date is stored.
If omitted, the field is not updated automatically.

##### `format`

The format of the value written to the field.
The `chrono` format is supported.

#### **command**

##### `auto_create`

Determines whether the file should be created if it does not exist.

##### `template`

The path of the file to copy when creating a new file.

##### `directory`

The directory path relative to `base_directory`.
If omitted, `base_directory` itself is used.
The `chrono` format is supported.

##### `file`

The file name.
The `chrono` format is supported.

##### `end_line`

- `true`: Adds the entry at the end of the section.
- `false`: Adds the entry at the beginning of the section.

##### `insert`

The `chrono` format is supported.
Specifies the line used as the reference point for insertion. The line is matched exactly.
If `insert` is configured but the specified line does not exist, an error is returned.

- If `end_line` is `true`, adds the entry at the end of the section.
- If `end_line` is `false`, adds the entry at the beginning of the section.

If `insert` is not configured, the behavior is as follows:

- If `end_line` is `true`, adds the entry at the end of the file.
- If `end_line` is `false`, adds the entry at the beginning of the body, after the frontmatter.

##### `not_format`

- `true`: Outputs list items only.
- `false`: Inserts the time.

### Configuration Examples

#### Integration with Obsidian

```toml
base_directory = "~/Documents/Obsidian"
default_command = "daily"
format = "%H:%M:%S"

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

#### Using Qwato for Memos

```toml
[command.memo]
auto_create = true
file = "index.md"
end_line = false
not_format = true
```

## 💡 Inspiration 💡

- [Obsidian](https://obsidian.md/): Continuing a journal
- [QuickAdd](https://community.obsidian.md/plugins/quickadd): Simple commands
- [Thino](https://community.obsidian.md/plugins/obsidian-memos): Timeline display

## 📜 LICENSE 📜

[Apache License Version 2.0](./LICENSE) / http://www.apache.org/licenses/LICENSE-2.0