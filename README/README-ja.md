# Qwato (くぁと)
English: [README.md](../README.md)
```bash
qwa
```
**指定したファイルにメモを追加するコマンドラインツール**

Obsidian QuickAdd Captureを使ってデイリーノートに書きこんでいる人向けです。わざわざObsidianを立ち上げずともターミナルからコマンドで書き込める利便性を味わってください。

## ⭐ 特徴 ⭐
- 一日一ページを基本とした日記ツール
- Obsidian QuickAdd Captureを再現した一行追記
- ObsidianのThinoを再現したタイムライン表示

## 💻 実行環境 💻
### OS
#### 検証済
- [x] Linux
- [x] Windows 11
#### 未検証
- [ ] Mac

## 📦 インストール 📦
### Binary
- [Windows - zip](https://github.com/c0b23092db/qwato/releases/download/v0.3.0/qwa-windows-x86_64.zip)
- [Windows - exe](https://github.com/c0b23092db/qwato/releases/download/v0.3.0/qwa.exe)
- [Linux - tar.gz](https://github.com/c0b23092db/qwato/releases/download/v0.3.0/qwa-linux-x86_64.tar.gz)
- [Mac - tar.gz](https://github.com/c0b23092db/qwato/releases/download/v0.3.0/qwa-macos-x86_64.tar.gz)
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
### eget
```bash
eget c0b23092db/qwato
```

## 📖 コマンド 📖
```
> qwa --help

CLI: Write a Memo for your notes in the terminal.

Usage: qwa.exe [OPTIONS] [arguments]...

Arguments:
  [arguments]...  Additional arguments

Options:
  -a, --add                Add a new message
  -c, --checkbox           Add a new checkbox
      --last-edit          Edit the last inserted memo
  -s, --summary            Show summary of messages updated today
  -l, --list               List all commands
  -n, --note               List all notes
  -t, --task               List all tasks
      --all                List all messages, including messages without a time format
      --tag <TAG>...       Tags for the message
      --link <LINK>        Link for the message
      --from <YYYY-MM-DD>  Filter from date (inclusive, format: YYYY-MM-DD)
      --to <YYYY-MM-DD>    Filter to date (inclusive, format: YYYY-MM-DD)
      --limit <LIMIT>      Show the limit of list messages
      --command            Check to Use Command
      --config <path>      Config File Path
  -h, --help               Print help
  -V, --version            Print version
```

### 引数指定の考え方
#### `--add` / `--checkbox`
一つ目の引数はコマンド、二つ目の引数以降はメッセージとして扱う。
```bash
[command] [message] [message] ...
```
#### それ以外
すべての引数をコマンドとして扱う。
```bash
[command] [command] ...
```

### テキスト入力

#### `--add`
```bash
qwa [message] ...
qwa [command] [message] [message] ...
qwa --add [message] ...
qwa --add [command] [message] [message] ...
```
指定したファイルにメモを追記する。二つ目の引数以降はすべて一行として扱われる。

#### `--checkbox`
```bash
qwa --checkbox [message] ...
qwa --checkbox [command] [message] [message] ...
```
指定したファイルにチェックボックス付きでメモを追記する。二つ目の引数以降はすべて一行として扱われる。

#### `--last-edit`
```bash
qwa --last-edit [message] ...
```
最後に追記したメモを編集する。二つ目の引数以降はすべて一行として扱われる。

### リスト表示

#### `--summary`
```bash
qwa --summary
```
今日更新されたメモのサマリーを表示する。

#### `--list`
```bash
qwa --list
qwa --list [command] [command] ...
```
リストとチェックボックスを表示する。

#### `--note`
```bash
qwa --note
qwa --note [command] [command] ...
```
リストを表示する。

#### `--task`
```bash
qwa --task
qwa --task [command] [command] ...
```
チェックボックスを表示する。

#### `--all`
```bash
qwa --all
```
時刻フォーマットのないリストも含め全て表示する。

### 動作変更

#### `--from` / `--to`
```bash
qwa --list --from <YYYY-MM-DD> --to <YYYY-MM-DD>
```
指定した期間のリストを表示する。`--from`は指定した日付以降、`--to`は指定した日付以前のリストを表示する。

#### `--tag` or `--tags`
```bash
qwa --tag <TAG>
```
`tag1,tag2,tag3`のようにカンマ区切りで指定する。`--tags`がエイリアスとして使える。
```bash
qwa --add [message] --tag <TAG>...
qwa --checkbox [message] --tag <TAG>...
```
コマンドで登録したタグと一緒に指定したタグを付与する。
```bash
qwa --list --tag <TAG>...
qwa --note --tag <TAG>...
qwa --task --tag <TAG>...
qwa --all --tag <TAG>...
```
指定したタグのいずれかを検索し表示する。

#### `--link`
```bash
qwa --link <LINK>
```
メモにリンクを付与する。

#### `--limit`
```bash
qwa --list --limit 10
```
表示する数を変更する。

#### `--config`
```bash
qwa --config <path>
```
読み込む設定ファイルを指定する。
ディレクトリであれば、ディレクトリ内にある設定ファイルを読み込む。

### 設定表示

#### `--command`
```bash
qwa --command
```
使用可能なコマンドを簡易表示する。
```bash
qwa --command [command] [command]...
```
指定したコマンドを詳細表示する。

### 非表示のオプション

#### デバッグ出力

##### `--utc-offset-time`
```bash
qwa --utc-offset-time
```
コマンド実行時に取得した時間を返す。
```
2026-08-21 00:15:55.451988700 +09:00
```

##### `--colon-sharp-question`
```bash
qwa --colon-sharp-question
```
読み込んだ設定ファイルを`"{:#?}"`の形式で表示する。

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
- 優先度の高い順に、指定した設定ファイル、`./qwato.toml`、`~/.config/qwato`にあるtomlファイルを読み込む。
- Rustの[chrono](https://docs.rs/chrono)で日付を扱う。

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
保管庫となるディレクトリ。
設定しない場合、ホームディレクトリが指定される。
##### `default_command`
コマンドの指定を行わない場合、デフォルトで使われるコマンドを指定する。
設定しない場合、一番目の引数に必ずコマンドを要求する。
##### `no_global`
trueの場合、グローバル読み込みの対象外になる。
優先度が一番に設定されている設定ファイルは常に読み込まれる。
##### `auto_command`
trueの場合、強制的に`default_command`を実行するようにする。
##### `time_format`
リストの先頭に登録されるchrono形式のフォーマット。
##### `data_format`
取得するファイルの形式。Obsidianのデイリーノートを参考にしている。
- `line`: YYYY-MM-DD.md
- `slash`: YYYY/MM/DD.md
- `slash_line`: YYYY/MM/DD/YYYY-MM-DD.md
- `header`: file-name.md / Header for YYYY-MM-DD
- `onefile`: file-name.md
#### **list**
##### `limit`
表示するリストの数を指定する。
#### **created**
MarkdownにYAMLかつfieldが存在する場合、ファイルの作成日に更新する。
##### `field`
ファイルの作成日を登録するフィールド名。
指定しない場合、フィールドは自動で更新されない。
##### `format`
フィールドに入力する文字をchrono形式で指定する。
#### **modified**
MarkdownにYAMLかつfieldが存在する場合、ファイルの更新時刻に更新する。
##### `field`
ファイルの更新日を登録するフィールド名。
指定しない場合、フィールドは自動で更新されない。
##### `format`
フィールドに入力する文字をchrono形式で指定する。
#### **command**
##### `auto_create`
ファイルが存在しない場合、作成するかを決定する。
##### `date_format`
取得するファイルの形式。Globalの設定を上書きする。
- `line`: YYYY-MM-DD.md
- `slash`: YYYY/MM/DD.md
- `slash_line`: YYYY/MM/DD/YYYY-MM-DD.md
- `header`: file-name.md / Header for YYYY-MM-DD
- `onefile`: file-name.md
##### `template`
ファイルの新規作成時、コピー元となるファイルのパス。
##### `directory`
`base_directory`から見るディレクトリの場所を指定する。指定しない場合、`base_directory`を使う。
chronoが使用可能。
##### `file`
ファイルの名前を指定する。
chronoが使用可能。
##### `insert`
chronoが使用可能。
挿入位置の基準となる行。完全一致で探す。
`insert`が設定されていて、指定行が存在しない場合はエラーになる。
- `end_line`がtrueの場合、セクション末尾へ追加する。
- `end_line`がfalseの場合、セクション先頭へ追加する。

設定されていない場合は以下のように動作する。
- `end_line`がtrueの場合、ファイル末尾へ追加する。
- `end_line`がfalseの場合、frontmatterの後、本文先頭へ追加する。
##### `tags`
```toml
tags = ["tag1", "tag2"]
```
タグを指定する。複数指定する場合はカンマ区切りで指定する。
##### `not_format`
- false: 時刻を挿入する
- true: リストのみにする
##### `end_line`
- false: 先頭へ追加する。
- true: 末尾へ追加する。

### 設定例
#### Obsidianとの連携
Obsidianと連携する場合、`base_directory`をObsidianのVaultに設定する。
次に、`command.daily`の`template`、`directory`、`file`を設定することで、Obsidianのデイリーノートと同じように使える。
Thinoのようにリスト表示する場合は、`--list`を使うことで、ObsidianのThinoと同じように使える。
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
inbox.mdのように一つのファイルに追記する場合、`file`を指定することで、指定したファイルに追記することができる。
```toml
[command.memo]
auto_create = true
file = "inbox.md"
end_line = false
not_format = true
```

もしソフトウェアを検索する人ならば、`tag`に`Software`を付与することで、検索しやすくなる。
`--all`を使うことで、時刻フォーマットのないリストを表示することができる。
```toml
[command.memo]
auto_create = true
file = "inbox.md"
tags = ["Software"]
not_format = true
```

#### コマンドを強制的に使わせる場合
`default_command`を設定しない場合、コマンドを要求してきます。
```toml
base_directory = "~/Documents/Obsidian"
# default_command = "daily"
```
コマンドを入力しないで実行すると`Not Config: defualt_command`が表示されます。

## 📰 今後 📰
今後の開発は[開発案.md](./開発案.md)を確認してください。
**Obsidianを使う制作者のために開発しているため、メンテナンスは消極的です**

## 💡 着想 💡
- [Obsidian](https://obsidian.md/): 日記の継続
- [QuickAdd](https://community.obsidian.md/plugins/quickadd): 簡易的なコマンド
- [Thino](https://community.obsidian.md/plugins/obsidian-memos): リスト表示

## 🔌 デバッグ 🔌
- **Global Configは`no_global = true`で読み込み停止を行ってください**

## 📜 LICENSE 📜
[Apache License Version 2.0](../LICENSE) / http://www.apache.org/licenses/LICENSE-2.0
