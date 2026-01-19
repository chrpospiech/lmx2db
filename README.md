# lmx2db

A Rust tool for converting LMX (Location Message Exchange) data to database format.

This is the boiler plate version auto-created by github copilot.
This needs to be corrected and adapted.

## Features

- Parse LMX format files
- Convert location data to database records
- Support for multiple database backends

## Installation

The tool is written in [Rust](https://rust-lang.org/),
which is also required for installing the tool. The recommended
way to install `Rust` is by using
[rustup](https://rust-lang.org/tools/install/).

Once `Rust` is installed, `fix_local_mail` can be installed
with the following command into `<install_prefix>/bin`.

```bash
cargo install --path [<project_dir>|.] [--root <install_prefix>]
```

## Usage

```bash
lmx2db --input <input.lmx> --output <database>
```

## License

See LICENSE file for details.

## Hints for the Developer

The distribution contains linter configuration files `rustfmt.toml`,
`.codespell.dictionary` and `.pre-commit-config.yaml`. These are meant
for the use of `pre-commit`. Every developer is strongly encouraged to
use `pre-commit` to maintain code quality.

To enable `pre-commit` using `uv`, please proceed as follows.

- `uv init`
- `rm .python-version main.py`
- `uv add pre-commit`
- `source .venv/bin/activate`
- `pre-commit run --all`
- `pre-commit install`

## Modules File

The database moduledefs.db has been discontinued in favor of a YAML file
with the following proposed structure.

```yaml
EasyBuild:
    gompi/2023a:
        compiler: GNU
        compiler_version: 12.3.0
        mpilib: OpenMPI
        mpilib_version: 4.1.5
    gompi/2024a:
        compiler: GNU
        compiler_version: 13.3.0
        mpilib: OpenMPI
        mpilib_version: 5.0.3
Lenox:
    aocc/5.0.0:
        compiler: AOCC
        compiler_version 5.0.0
    openmpi/4.1.6:
        mpilib: OpenMPI
        mpilib_version: 4.1.6
Other_cluster:
    GCC/14.1.0:
        compiler: GNU
        compiler_version 14.1.0
```

The file doesn't need to exist. If the file is provided, an attempt will be made
to update the columns `compiler`, `compiler_version`, `mpilib` and `mpilib_version`
of table `runs` by inspecting the list of loaded modules as recorded in
the `environ` section of the LMX_summary file.

The file has to be provided (and maintained) by the user. The user has to take
care of the unique spelling (including use of capital letters) of compiler and
MPI library names.
