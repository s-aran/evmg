If you prefer in English, see [README.md](README.md).

# envvar

envvar はコマンドラインベースの JSON による環境変数管理ツールです。

## 利用可能なオプション

### 共通

| 引数                  | 説明                                                       | 初期値                              |
| --------------------- | ---------------------------------------------------------- | ----------------------------------- |
| `--version`           | バージョンを表示                                           | なし                                |
| `--export=<filepath>` | _filepath_ にエクスポート                                  | なし                                |
| `--import=<filepath>` | _filepath_ からインポート                                  | なし                                |
| `--dry-run`           | ドライランにする                                           | False                               |
| `--no-color`          | 着色しない                                                 | False                               |
| `--rc=<filepath>`     | 環境変数を設定するスクリプトを _filepath_ に出力           | `.envvar_<shell name>rc`            |
| `--shell=<name>`      | `rc` ファイルの種類 (例: `powershell`, `bash`, `zsh`, ...) | Windows: `powershell`, Liux: `bash` |

### Windows 限定

| 引数         | 説明                                                     | 初期値 |
| ------------ | -------------------------------------------------------- | ------ |
| `--registry` | 環境変数をカレントユーザーレジストリー（HKCU）に出力する | False  |

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

`ps1` ファイルに出力します。以下のコマンドは `input.json` からインポートして PowerShell 用に `~/.envvar.ps1` に出力する例です。

```ps1
envvar --import=input.json --shell=powershell --rc=~/.envvar.ps1
```

`--registry` オプションを指定した場合はレジストリーに設定します。このとき，ファイル出力は行いません。以下のコマンドラインは `input.json` からインポートする例です。

```sh
envvar --import=input.json
```

### Linux

`rc` ファイルに出力します。以下のコマンドは `input.json` からインポートして bash 用に `~/.envvar_bashrc` に出力する例です。

```sh
envvar --import=input.json --shell=bash --rc=~/.envvar_bashrc
```

あらかじめ，お使いのシェルの `rc` ファイル (例: `~/.profile`, `~/.bashrc`)に以下を追記してください。bash の場合の例です。

```sh
source ~/.envvar_bashrc
```
