// Copyright 2026 lmx2db C. Pospiech
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(test)]
mod tests {
    use crate::cmdline::CliArgs;
    use clap::Parser;

    #[test]
    fn test_parse_with_defaults() {
        // Use temp_env to ensure LMX2DB_DATABASE_URL is unset for this test,
        // preventing interference from parallel tests that set it.
        temp_env::with_var_unset("LMX2DB_DATABASE_URL", || {
            let args = CliArgs::try_parse_from(["lmx2db"]).unwrap();

            assert!(!args.verbose);
            assert!(!args.dry_run);
            assert!(!args.create_sqltypes);
            assert!(!args.do_import);
            assert_eq!(args.db_url, "output_to_files_only");
            assert_eq!(args.sqltypes_file, "sqltypes.yml");
            assert_eq!(args.sql_file, "import.sql");
            assert_eq!(args.module_file, "modules.yml");
            assert_eq!(args.settings_file, "settings.yml");
            assert_eq!(args.project_file, "project.yml");
            assert!(args.directories.is_empty());
        });
    }

    #[test]
    fn test_parse_with_all_short_flags() {
        let args = CliArgs::try_parse_from([
            "lmx2db",
            "-v",
            "-D",
            "-c",
            "-i",
            "-u",
            "mysql://user:pass@host/db",
            "-t",
            "types.yml",
            "-f",
            "import_test.sql",
            "-m",
            "mods.yml",
            "-s",
            "sets.yml",
            "-p",
            "proj.yml",
            "dir1",
            "dir2",
        ])
        .unwrap();

        assert!(args.verbose);
        assert!(args.dry_run);
        assert!(args.create_sqltypes);
        assert!(args.do_import);
        assert_eq!(args.db_url, "mysql://user:pass@host/db");
        assert_eq!(args.sqltypes_file, "types.yml");
        assert_eq!(args.sql_file, "import_test.sql");
        assert_eq!(args.module_file, "mods.yml");
        assert_eq!(args.settings_file, "sets.yml");
        assert_eq!(args.project_file, "proj.yml");
        assert_eq!(args.directories, vec!["dir1", "dir2"]);
    }

    #[test]
    fn test_parse_with_long_flags() {
        let args = CliArgs::try_parse_from([
            "lmx2db",
            "--verbose",
            "--dry-run",
            "--create-sqltypes",
            "--do-import",
            "--db-url",
            "mysql://localhost/test",
            "dir1",
        ])
        .unwrap();

        assert!(args.verbose);
        assert!(args.dry_run);
        assert!(args.create_sqltypes);
        assert!(args.do_import);
        assert_eq!(args.db_url, "mysql://localhost/test");
    }

    #[test]
    fn test_parse_multiple_directories() {
        let args = CliArgs::try_parse_from(["lmx2db", "dir1", "dir2", "dir3"]).unwrap();

        assert_eq!(args.directories, vec!["dir1", "dir2", "dir3"]);
    }

    #[test]
    fn test_parse_db_url_from_env() {
        let env_url = "mysql://envuser:envpass@envhost/envdb";
        temp_env::with_var("LMX2DB_DATABASE_URL", Some(env_url), || {
            let args = CliArgs::try_parse_from(["lmx2db"]).unwrap();
            assert_eq!(args.db_url, env_url);
        });
    }

    #[test]
    fn test_parse_db_url_flag_overrides_env() {
        let env_url = "mysql://envuser:envpass@envhost/envdb";
        let flag_url = "mysql://flaguser:flagpass@flaghost/flagdb";
        temp_env::with_var("LMX2DB_DATABASE_URL", Some(env_url), || {
            let args = CliArgs::try_parse_from(["lmx2db", "--db-url", flag_url]).unwrap();
            assert_eq!(args.db_url, flag_url);
        });
    }
}
