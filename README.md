# lmx2db

Convert LMX summary files (produced by the LMX_trace profiling/tracing tool)
into SQL statements and database entries.

## Features

- Parse LMX summary YAML files.
- Import data into MySQL or write SQL files for later ingestion.
- Optional enrichment using project, settings, and modules YAML files.

## Installation

Install Rust via [rustup](https://rust-lang.org/tools/install/).

Build and install from the repo:

```bash
cargo install --path .
```

## Usage

Run against one or more directories that contain LMX summary files:

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
        compiler_version: 5.0.0
    openmpi/4.1.6:
        mpilib: OpenMPI
        mpilib_version: 4.1.6
Other_cluster:
    GCC/14.1.0:
        compiler: GNU
        compiler_version: 14.1.0
```

The file does not need to exist. If provided, the tool attempts to update
the `compiler`, `compiler_version`, `mpilib`, and `mpilib_version` columns
of table `runs` by inspecting the list of loaded modules in the `environ`
section of the LMX summary file.

The file has to be provided (and maintained) by the user. The user has to take
care of unique spelling (including use of capital letters) of compiler and MPI
library names.
