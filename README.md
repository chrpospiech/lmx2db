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
  This file is normally searched for in any super directory of the
  `LMX_summary.*.yml` file and can therefore be shared among several
  runs.
- Attributes runs to the user that created these results. This can be
  achieved by adding the following information to the `project.yml` file.

  ```yaml
  cluster: name_of_cluster
  person: Christoph Pospiech
  ```

  If cluster `name_of_cluster` is already in table `clusters` of the
  receiving database and the tables `people` and `userids` contain the
  necessary information, the person can be also determined from the value
  of the environment variable `$USER` during the run.
- Check all data types against the database schema before creating
  the SQL queries.
- Optionally provide additional settings for each run through a file
  `settings.yml` in the same directory as the `LMX_summary.*.yml`
  file.
- Optionally determine compiler and MPI versions from the environment
  modules loaded during run time of the job, provided a translation
  table `modules.yml` is provided as detailed below. This file is normally
  searched for in any super directory of the `LMX_summary.*.yml` file
  and can therefore be shared among several runs.

## Installation

The tool is written in [Rust](https://rust-lang.org/),
which is also required for installing the tool itself.
As of now, there are no binaries provided.

Two out of several choices to install `Rust` are outlined
below.

### Installation of Rust via rustup

The recommended way to install `Rust` is by using
[rustup](https://rust-lang.org/tools/install/). The referred web page
provides the following command, that can be copied and pasted into a
bash prompt.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

No root needed, this installs into `$HOME/.cargo` by default.
The installation also adds `$HOME/.cargo/bin` to the `$PATH`
variable --- unless opted out by choosing a custom installation.
The change of `$PATH` is done by appending the line `. $HOME/.cargo/env`
to the following files.

- `$HOME/.bashrc`
- `$HOME/.bash_profile`
- `$HOME/.profile`
- `$HOME/.zshrc`

The extra lines can be easily removed from there if not desired.

### Installation of Rust via EasyBuild

A fairly recent version of `Rust` is required and can be installed by
the following command.

```bash
eb --skip --robot Rust-1.91.1-GCCcore-14.2.0.eb
```

### Installation of lmx2db

Once `Rust` is installed, `lmx2db` can be installed
with the following command into `<install_prefix>/bin`.

```bash
cargo install --path [<project_dir>|.] [--root <install_prefix>]
```

- `cargo` needs to be in the `$PATH`, which is either arranged by `rustup`
  or by loading the `EasyBuild` module `Rust/1.91.1`.
- The `<project_dir>` means the directory where **this** `README.md` resides.
  If this already is the current directory, the `.` can be used in this command.
- If not already there (e.g. created by `rustup`), the above installation command
  creates and populates a directory `$HOME/.cargo`.
- If the parameter `--root` is not specified, the installation goes to `$HOME/.cargo/bin`
- A single file `lmx2db` is installed in `[<install_prefix>|$HOME/.cargo]/bin`.

## Usage

Run against one or more directories that contain `LMX_trace` output files:

```bash
lmx2db -u mysql://lmx_user:lmx_pass@database_ip/lmxdb /path/to/runs /path/to/other/runs
```

The specified directories `/path/to/runs /path/to/other/runs` are searched recursively
for `LMX_trace` output files.

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

`lmx2db` inserts the data directly into the database, if the following conditions are met:

- Project YAML files of the following form.

  ```YAML
  ---
  project: 4paper_2025
  code: GROMACS
  code_version: 2025.3
  test_case: benchMEM
  cluster: name_of_cluster
  person: Christoph Pospiech
  ```

  The last two items are optional. The file location
  [follows the rules below](#handling-of-options--m-and--p).
- A `mariadb` database server is listening on `database_ip:3306`
- The database `lmxdb` must exist on this `mariadb` server.
- The database must conform to the schema as discussed below.
- The database user `lmx_user` with password `lmx_pass`
   - must exist and must be allowed access `lmxdb`
   - must be able to execute SQL INSERT, UPDATE, SELECT and
     execution of stored functions for database `lmxdb`.

If the database URL is invalid (which **may** take a TCP/IP timeout
to find out), `lmx2db` will write the SQL queries to a file
(see option `-f, --sql-file`).

However, `lmx2db` checks all data types against the database schema
before creating the SQL queries. If the database cannot be queried
for the correct schema and data types for each table column, this
information has to be provided in a file (see option `-t, --sqltypes-file`).

There is a sample file `sample_sqltypes.sql` provided, based on the
database schema in subdirectory `schema`. This file can be used
if the database schema of the database pointed to by the URL fits
the one given in directory schema.

A correct sqltypes file can be created on a different computer with
access to the correct database by a separate call to `lmx2db` with
option `-c` (and -u pointing to the desired database).
Then this file needs to be transferred to the computer where `lmx2db`
is called to process `/path/to/runs /path/to/other/runs`.

With these extra type checks, the SQL queries are not created if
the input data cannot be cast to the correct types or the database
schema was changed in a way that is not backward compatible.

Any such database schema and type mismatches would also create
SQL import errors, but these are sometimes very cryptic and
do not list the table or column that are in error. This strategy
also minimizes the chance of creating a file with invalid SQL
queries, which would not create any error until the file is
attempted to import into the database after being
transferred to a system with database access.

## Database Schema

The subdirectory `schema` contains the required database schema.
After creating an empty database `lmxdb` and issuing a `USE lmxdb;`
query, the following files should be imported in the stated order.

- `tables.sql`: Creates the database tables.
- `functions.sql`: Creates (Defines) the stored functions.
- `view.sql`: (Optional) creates a single view as an example.
- `mpi_names.sql`: (Optional) imports the names of the MPI calls.
- `minimal_data.sql`: (Optional) imports person, cluster and userid sample data.

These database operations can be conveniently executed with `phpMyAdmin`.

## Modules File

The database moduledefs.db (in a predecessor of this tool) has been discontinued
in favor of a YAML file with the following proposed structure.

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
  compiler: "AOCC"
  compiler_version: "5.0.0"
```

The file does not need to exist. If provided, the tool attempts to update
the columns `compiler`, `compiler_version`, `mpilib`, and `mpilib_version`
of table `runs` by inspecting the list of loaded modules in the `environ`
section of the LMX summary file.

The file location [follows the rules below](#handling-of-options--m-and--p).

The file has to be provided (and maintained) by the user. The user has to take
care of unique spelling (including use of capital letters) of compiler and MPI
library names.

## Handling of options -m and -p

Both options search for the requested file in any super directory of the
`LMX_summary.*.yml` file and can therefore be shared among several
`LMX_summary.*.yml` files. Three cases are distinguished.

- The option is not set: The file is searched for as described above.
- The option is set to a string not containing a path separator:
  The string is treated as a file name and is searched for as described above.
- The option is set to a string containing a path separator: The
  string is treated as an absolute or relative path and the file is expected
  at this location.

## Testing

Tests use `#[sqlx::test]` and require `DATABASE_URL` to be set. The test
runner loads `.env` from the repository root if it exists. Use
`.env.example` as a template for local configuration.

Example `.env`:

```text
DATABASE_URL=mysql://test_user:test_pass@database_ip:3306/lmxtest
```

The `DATABASE_URL` needs to meet the following conditions.

- A `mariadb` server needs to be up and running and listening to requests
  sent to `database_ip:3306`.
- The database `lmxtest` must exist on this `mariadb` server.
- It is sufficient and even desired that this database is empty.
- The database user `test_user` with password `test_pass`
   - must exist and must be allowed access `lmxtest`
   - must be allowed to create and drop databases and tables on the server ad libitum.
- It may be considered to run the database in a (docker)
  [container](https://hub.docker.com/_/mariadb) with the database `root` as `test_user`.
- Many of the unit tests set up their private database for testing which may lead to
  transient unit test failures caused by SQL
  [error 1615](https://mariadb.com/docs/server/reference/error-codes/mariadb-error-codes-1600-to-1699/e1615).
  The test databases have quite a few stored functions. These are compiled (SQL slang: prepared)
  and the result stored in a stored-program-cache. With lots of concurrently running unit tests,
  this cache overflows (sometimes) which causes the SQL error 1615.
  The cure is to increase the value of the system variable stored-program-cache
  in the correct /etc/mysql/*.cnf file to the double of the default value and
  then restart mariadb with systemctl.

When all these conditions are met, the unit tests can be started with the following command:

```bash
cargo test
```

## License

See LICENSE for details.

## Hints for Developers

The repo contains linter configuration files `rustfmt.toml`,
`.codespell.dictionary`, and `.pre-commit-config.yaml`.
The developer is strongly encouraged to use `pre-commit`
with these configuration files to maintain code quality.

To enable `pre-commit` using `uv`:

- `uv init`
- `rm .python-version main.py`
- `uv add pre-commit`
- `source .venv/bin/activate`
- `pre-commit run --all`
- `pre-commit install`
