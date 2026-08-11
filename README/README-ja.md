# Qwato (くぁと)
English: [README.md](../README.md)
```bash
qwa
```
指定したファイルにメモを追加するコマンドラインツール

## ⭐ 特徴 ⭐
- ObsidianのQuickAddで登録できるCaptureを再現
- ObsidianのThinoのようにタイムラインを表示

## 💻 実行環境 💻
### OS
#### 検証済
- [x] Windows 11
#### 未検証
- [ ] Linux
- [ ] Mac

## 📦 インストール 📦
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

## 📖 コマンド 📖
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
      --limit <LIMIT>  Show the limit of list messages
      --load           Check to Load Config File
  -h, --help           Print help
  -V, --version        Print version
```

### `--add`
指定したファイルにメモを追記する。
二つ目の引数以降はすべて一行で処理される。
```bash
qwa <message>
qwa <command> <message>
qwa --add <message>
qwa --add <command> <message>
```

### `--check`
指定したファイルにチェックボックス付きでメモを追記する。
二つ目の引数以降はすべて一行で処理される。
```bash
qwa --check <message>
qwa --check <command> <message>
```

### `--list`
```bash
qwa --list
qwa --list <command> <command> ...
```
コマンドで指定しているディレクトリのファイル群を参照し、リストとチェックボックスを表示する。

### `--note`
```bash
qwa --note
qwa --note <command> <command> ...
```
コマンドで指定しているディレクトリのファイル群を参照し、リストを表示する。

### `--task`
```bash
qwa --task
qwa --task <command> <command> ...
```
コマンドで指定しているディレクトリのファイル群を参照し、チェックボックスを表示する。

### `--limit`
```bash
qwa --list --limit 10
```
表示する数を指定する。

### `--load`
```bash
qwa --load
```
設定ファイルをRustの`"{:#?}"`の形式で表示する。

### `--help`
```bash
qwa --help
```
コマンドのヘルプを表示する。

### `--version`
```bash
qwa --version
```
コマンドのバージョンを表示する。

## ⚙ 設定ファイル ⚙
**注意事項**
- どのOSでも`~/.config/qwato`を参照します。
- Rustの[chrono](https://docs.rs/chrono)で日付を扱います。

### デフォルト設定
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

### 設定項目
#### **General**
##### `base_directory`
基準となるディレクトリ。
##### `default_command`
コマンドの指定を行わない場合、デフォルトで使われるコマンドを指定する。
##### `auto_command`
- true: 強制的に`default_command`を実行するようにする。
##### `time_format`
リストの先頭に登録されるフォーマット。
chronoが使用可能。
#### **list**
##### `limit`
`--list`、`--note`、`--task`のみ機能する。表示するリストの数。
#### **created**
MarkdownにYAMLかつfieldが存在する場合、ファイルの作成日に更新する。
##### `field`
ファイルの作成日を登録するフィールド。
指定しない場合、フィールドは自動で更新されない。
##### `format`
フィールドに入力する文字。
chronoが使用可能。
#### **modified**
MarkdownにYAMLかつfieldが存在する場合、ファイルの更新時刻に更新する。
##### `field`
ファイルの更新日を登録するフィールド。
指定しない場合、フィールドは自動で更新されない。
##### `format`
フィールドに入力する文字。
chronoが使用可能。
#### **command**
##### `auto_create`
ファイルが存在しない場合、作成するかを決定する。
##### `template`
ファイルの新規作成時、コピー元となるファイルのパス。
##### `directory`
`base_directory`から見るディレクトリの場所。指定しない場合、`base_directory`を使う。
chronoが使用可能。
##### `file`
ファイルの名前。
chronoが使用可能。
##### `end_line`
- true: セクション末尾へ追加する。
- false: セクション先頭へ追加する。
##### `insert`
chronoが使用可能。
挿入位置の基準となる行。完全一致で探す。
`insert`が設定されていて、指定行が存在しない場合はエラーになる。
- `end_line`がtrueの場合、セクション末尾へ追加する。
- `end_line`がfalseの場合、セクション先頭へ追加する。

設定されていない場合は以下のように動作する。
- `end_line`がtrueの場合、ファイル末尾へ追加する。
- `end_line`がfalseの場合、frontmatterの後、本文先頭へ追加する。
##### `not_format`
- true: リストのみにする
- false: 時刻を挿入する

### 設定例
#### Obsidianとの連携
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

#### メモ用途で使う場合
```toml
[command.memo]
auto_create = true
file = "index.md"
end_line = false
not_format = true
```

## 💡 着想 💡
- [Obsidian](https://obsidian.md/): 日記の継続
- [QuickAdd](https://community.obsidian.md/plugins/quickadd): 簡易的なコマンド
- [Thino](https://community.obsidian.md/plugins/obsidian-memos): リスト表示

## 📜 LICENSE 📜
[Apache License Version 2.0](../LICENSE) / http://www.apache.org/licenses/LICENSE-2.0
