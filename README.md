日本語で読みたい場合は [README_J.md](README_J.md) をご覧ください。

# envvar

envvar is a command-line based an environment variable management tool with JSON.

## Available options

### Common

| Option                | Comment                | Default |
| --------------------- | ---------------------- | ------- |
| `--version`           | Display version        | N/A     |
| `--export=<filepath>` | Export to _filepath_   | N/A     |
| `--import=<filepath>` | Import from _filepath_ | N/A     |
| `--dry-run`           | Set dry run mode       | False   |
| `--no-color`          | Set no color mode      | False   |

### Linux Only

| Option            | Comment                                                      | Default                |
| ----------------- | ------------------------------------------------------------ | ---------------------- |
| `--rc=<filepath>` | Output scripts that set environment variables to _filepath_. | `.envvar_<shell name>` |
| `--shell=<name>`  | Type of `rc` file (e.g., `bash`, `zsh`, ...)                 | `bash`                 |

## Usage

### Display

Like a `env` command with colors.

```sh
envvar
```

If color is not required:

```sh
envvar --no-color
```

### Export

Export the currently environment variables to a JSON file. The following command line is an example of exporting to `output.json`.

```sh
envvar --export=output.json
```

### Import

Import from a JSON file and apply to the environment variables. Difference by platform are as follows.

#### Windows

Set to the registry. The following command line is an example of importing from `input.json`.

```sh
envvar --import=input.json
```

### Linux

Output to `rc` file. The following command line is an example of importing from `input.json` and outputting to `~/.envvarrc_bash` for bash.

```sh
envvar --import=input.json --shell=bash --rc=~/.envvar_bash
```

In advance, add the following that to your shell `rc` file (e.g., `~/.profile`, `~/.bashrc`). Example for `bash`:

```sh
source ~/.envvarrc_bash
```
