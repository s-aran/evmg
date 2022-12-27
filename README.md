日本語で読みたい場合は [README_J.md](README_J.md) をご覧ください。

# envvar

envvar is a command-line based an environment variable management tool with JSON.

## Available options

### Common

| Option                | Comment                                                      | Default                              |
| --------------------- | ------------------------------------------------------------ | ------------------------------------ |
| `--version`           | Display version                                              | N/A                                  |
| `--export=<filepath>` | Export to _filepath_                                         | N/A                                  |
| `--import=<filepath>` | Import from _filepath_                                       | N/A                                  |
| `--dry-run`           | Set dry run mode                                             | False                                |
| `--no-color`          | Set no color mode                                            | False                                |
| `--rc=<filepath>`     | Output scripts that set environment variables to _filepath_. | `.envvar_<shell name>rc`             |
| `--shell=<name>`      | Type of `rc` file (e.g., `powershell`, `bash`, `zsh`, ...)   | Windows: `powershell`, Linux: `bash` |

### Windows Only

| Option       | Comment                                                        | Default |
| ------------ | -------------------------------------------------------------- | ------- |
| `--registry` | Set environment variables to the current user registry (HKCU). | False   |

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

Output to `ps1` file. The following command line is an example of importing from a `input.json` and outputting to `~/.envvar.ps1` for PowerShell:

```ps1
envvar --import=input.json --shell=powershell --rc=~/.envvar.ps1
```

If you specified `--registry` option, Set to the registry. At this time, no output to file is performed. The following command line is an example of importing from `input.json`.

```sh
envvar --import=input.json --registry
```

### Linux

Output to `rc` file. The following command line is an example of importing from `input.json` and outputting to `~/.envvar_bashrc` for bash.

```sh
envvar --import=input.json --shell=bash --rc=~/.envvar_bashrc
```

In advance, add the following that to your shell `rc` file (e.g., `~/.profile`, `~/.bashrc`). Example for `bash`:

```sh
source ~/.envvar_bashrc
```
