# lmx2db

Postprocesses the output of the tool `LMX_trace`. `LMX_trace` is an acronym
for *Lightweight MPI traces with eXtensions*. The output consists of files in
YAML format with names like `LMX_summary.76372.0.yml`. Depending on configuration
settings, there might be additional files following the naming schema
`LMX_<xxx>_profile.76372.<yy>.yml`, where `<xxx>` is one of `MPI` or `itimer` and
`<yy>` is an MPI rank. These files are parsed and the extracted data are imported
into a `mariadb` database. If the database cannot be directly accessed, the necessary
SQL queries for importing the data are written to a file.

## Features

- Parse `LMX_trace` YAML output files.
- Import data into MySQL or write SQL files for later ingestion.
- Attribute runs to a project as specified by a file `project.yml`.
  This file is searched for in any super directory of the
  `LMX_summary.*.yml` file and can therefore be shared among several
  runs.
- Check all data types against the database schema before creating
  the SQL queries.
- Optionally provide additional settings for each run through a file
  `settings.yml` in the same directory as the `LMX_summary.*.yml`
  file.
- Optionally determine compiler and MPI versions from the environment
  modules loaded during run time of the job, provided a translation
  table `modules.yml` is provided as detailed below. This file is also
  searched for in any super directory of the `LMX_summary.*.yml` file
  and can therefore be shared among several runs.

## Installation

The tool is written in [Rust](https://rust-lang.org/),
which is also required for installing the tool. The recommended
way to install `Rust` is by using
[rustup](https://rust-lang.org/tools/install/).

Once `Rust` is installed, `lmx2db` can be installed
with the following command into `<install_prefix>/bin`.

```bash
cargo install --path [<project_dir>|.] [--root <install_prefix>]
```

## Usage

Run against one or more directories that contain `LMX_trace` output files:

```bash
lmx2db -u mysql://user:pass@localhost/lmxdb /path/to/runs /path/to/other/runs
```

Common options:

- `-u, --db-url`: MySQL connection string.
- `-t, --sqltypes-file`: SQL types YAML file (default: `sqltypes.yml`).
- `-c, --create-sqltypes`: Create SQL types file from the database and exit.
- `-f, --sql-file`: Output SQL file for import statements (default: `import.sql`).
- `-i, --do-import`: Import unknown foreign keys rather than erroring.
- `-m, --module-file`: Optional modules YAML file (default: `modules.yml`).
- `-s, --settings-file`: Optional settings YAML file (default: `settings.yml`).
- `-p, --project-file`: Project YAML file (default: `project.yml`).
- `-D, --dry-run`: Do not execute DB writes.
- `-v, --verbose`: Verbose output.

## Testing

Tests use `#[sqlx::test]` and require `DATABASE_URL` to be set. The test
runner loads `.env` from the repository root if it exists. Use
`.env.example` as a template for local configuration.

Example `.env`:

```text
DATABASE_URL=mysql://user:pass@127.0.0.1:3306/lmxdb
```

Then run:

```bash
cargo test
```

## License

See LICENSE for details.

## Hints for Developers

The repo contains linter configuration files `rustfmt.toml`,
`.codespell.dictionary`, and `.pre-commit-config.yaml`. Use `pre-commit`
to maintain code quality.

To enable `pre-commit` using `uv`:

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
intel2025.2.1:
  compiler: "Intel"
  compiler_version: "2025.2.1"
  mpilib: "Intel"
  mpilib_version: "2021.16.0"
gompi-2024a:
  compiler: "GNU"
  compiler_version: "13.3.0"
  mpilib: "OpenMPI"
  mpilib_version: "5.0.3"
openmpi/5.0.8:
  mpilib: "OpenMPI"
  mpilib_version: "5.0.8"
aocc/5.0.0:
  compiler: "aocc"
  compiler_version: "5.0.0"
```

The file does not need to exist. If provided, the tool attempts to update
the `compiler`, `compiler_version`, `mpilib`, and `mpilib_version` columns
of table `runs` by inspecting the list of loaded modules in the `environ`
section of the LMX summary file.

The file has to be provided (and maintained) by the user. The user has to take
care of unique spelling (including use of capital letters) of compiler and MPI
library names.
