# demo-up

[![Build Status](https://dev.azure.com/modernops/demo-up/_apis/build/status/smurawski.demo-up?branchName=master)](https://dev.azure.com/modernops/demo-up/_build/latest?definitionId=93&branchName=master)

`demo up` is a CLI tool for preparing learning path environments for Microsoft Ignite | The Tour.

## Installation

### Windows

```powershell
$params = @{
    Uri = 'https://github.com/smurawski/demo-up/releases/latest/download/demo.exe'
    UseBasicParsing = $true
    OutFile = 'demo.exe'
}
invoke-restmethod @params
```

### Linux

Requires: **OpenSSL 1.1**

```bash
curl -L 'https://github.com/smurawski/demo-up/releases/latest/download/demo-linux' -o demo
chmod +x demo
```

### Mac

```bash
curl -L 'https://github.com/smurawski/demo-up/releases/latest/download/demo-darwin' -o demo
chmod +x demo
```

## Examples

Download and set up for one talk.

`demo up --azure-subscription 'My Subscription --session-name SRE10`

Download and setup multiple talks.

`demo up --azure-subscription 'My Subscription --session-name DEV10 SRE10`

Download and setup a learning path.

`demo up --azure-subscription 'My Subscription --learning-path SRE`

### `demo`

Sets up or tears down demo environments for Microsoft Ignite | The Tour

```text
USAGE:
demo [OPTIONS] [SUBCOMMAND]
```

```text
FLAGS:                                                                                               -h, --help       Prints help information
-V, --version    Prints version information

SUBCOMMANDS:
fetch    Retrieves a local copy of a configuration file for the demo environment for one or more learning paths or sessions.
help     Prints this message or the help of the given subcommand(s)
up       Sets up the demo environment for one or more learning paths or sessions.
```

### `demo fetch`

Retrieves a local copy of a configuration file for the demo environment for one or more learning paths or sessions.

```text
USAGE:
demo fetch [OUTPUT]
```

```text
FLAGS:
-h, --help Prints help information
-V, --version Prints version information

OPTIONS:
-c, --config-file <config_file>       [default: https://aka.ms/demo-up]

ARGS:
<OUTPUT> Path to write the local configuration file to use. [default: ./demo.yml]
```

### `demo up`

Sets up the demo environment for one or more learning paths or sessions.

```text
USAGE:
demo up [OPTIONS]
```

```text
FLAGS:
-h, --help Prints help information
-V, --version Prints version information

OPTIONS:
-c, --config-file <config_file>       [default: https://aka.ms/demo-up]
-a, --azure-subscription <subscription>
-e, --event <event> Event name (to keep environments unique). Defaults to your local user name.
    --exclude <exclude>... Sections of the session to skip retrieval or exectution. [possible values: Slides,Videos, GitRepos, Commands]
-l, --learning-path <learning_path>... Learning path. Allows multiple [possible values: ALL, DAT, DEV, FUN, HYB, MIG, SRE]
-s, --session-name <session_name>... Session name. Allows multiple. [possible values: DAT10, DAT20, DAT30, DAT40, DAT50, DEV10, DEV20, DEV30, DEV40, DEV50, FUN10, FUN20, FUN30, FUN40, FUN50, HYB10, HYB20, HYB30, HYB40, HYB50, MIG10, MIG20, MIG30, MIG40, MIG50, SRE10, SRE20, SRE30, SRE40, SRE50]
```

## Contributing

### Prerequisites

* [Install Rust](https://rustup.rs/).  Install the latest stable target for your environment.  For windows, use the stable mvsc channel.

* Install rustfmt

    `rustup component add rustfmt`

* clippy

    `rustup component add clippy`

### Before submitting a PR

* [ ] Check your formatting

    `cargo fmt -- --check`

* [ ] Check your correctness

    `cargo clippy -- --deny warnings`

* [ ] Check your code compiles

    `cargo check` **or** `cargo build`

* [ ] Check your tests pass

    `cargo test`

### Other tips

To run the command you built, you can use `cargo run` to run the current state of the codebase.  You can pass commands to the command like

```bash
cargo run -- up
```

Anything after the first `--` will be passed to the `demo` command as an argument.
