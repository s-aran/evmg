If you prefer in English, see [README.md](README.md).

# envvar

envvar はコマンドラインベースの JSON による環境変数管理ツールです。

## 利用可能なオプション

### 共通

| オプション            | コメント                  | デフォルト |
| --------------------- | ------------------------- | ---------- |
| `--version`           | バージョンを表示          | なし       |
| `--export=<filepath>` | _filepath_ にエクスポート | なし       |
| `--import=<filepath>` | _filepath_ からインポート | なし       |
| `--dry-run`           | ドライランにする          | False      |
| `--no-color`          | 着色しない                | False      |

### Linux 限定

| オプション        | コメント                                         | デフォルト             |
| ----------------- | ------------------------------------------------ | ---------------------- |
| `--rc=<filepath>` | 環境変数を設定するスクリプトを _filepath_ に出力 | `.envvar_<shell name>` |
| `--shell=<name>`  | `rc` ファイルの種類 (例: `bash`, `zsh`, ...)     | `bash`                 |

## 使い方

### 表示

`env` コマンドの結果に色を付けたような出力をします。

```sh
envvar
```

色が不要であれば以下の指定をします。

```sh
envvar --no-color
```

### エクスポート

現在の環境変数を JSON ファイルにエクスポートします。以下のコマンドラインは `output.json` にエクスポートする例です。

```sh
envvar --export=output.json
```

### インポート

JSON ファイルからインポートして環境変数に適用します。プラットフォームによる違いは以下の通りです。

#### Windows

レジストリーに設定します。以下のコマンドラインは `input.json` からインポートする例です。

```sh
envvar --import=input.json
```

### Linux

`rc` ファイルに出力します。以下のコマンドは `input.json` からインポートして bash 用に `~/.envvar_bash` に出力する例です。

```sh
envvar --import=input.json --shell=bash --rc=~/.envvar_bash
```

あらかじめ，お使いのシェルの `rc` ファイル (例: `~/.profile`, `~/.bashrc`)に以下を追記してください。bash の場合の例です。

```sh
source ~/.envvarrc_bash
```
