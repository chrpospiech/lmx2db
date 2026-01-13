use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(name = "lmx2db", author = "C. Pospiech", version = "0.1", about = "Convert LMX files to database entries", long_about = None)]
pub struct CliArgs {
    /// Verbose output
    #[arg(short = 'v', long, default_value_t = false)]
    pub verbose: bool,

    /// Perform a dry run without making actual changes
    #[arg(short = 'D', long, default_value_t = false)]
    pub dry_run: bool,

    /// Database URL
    #[arg(
        short = 'u',
        long,
        default_value = "mysql://lmxdb:lmxdb@localhost/lmxdb"
    )]
    pub db_url: String,

    /// name of sqltypes file
    #[arg(short = 't', long, default_value = "sqltypes.yml")]
    pub sqltypes_file: String,

    /// create sqltypes file from database
    #[arg(short = 'c', long, default_value_t = false)]
    pub create_sqltypes: bool,

    /// Name of the SQL file with import statements
    #[arg(short = 'f', long, default_value = "import.sql")]
    pub sql_file: String,

    /// import unknown foreign keys rather then raising errors
    #[arg(short = 'i', long, default_value_t = false)]
    pub do_import: bool,

    /// Filename of the optional YAML file for parsing compiler and MPI versions
    #[arg(short = 'm', long, default_value = "modules.yml")]
    pub module_file: String,

    /// Filename of optional YAML file with additional data for the settings table
    #[arg(short = 's', long, default_value = "settings.yml")]
    pub settings_file: String,

    /// filename of compulsory YAML file with project data
    #[arg(short = 'p', long, default_value = "project.yml")]
    pub project_file: String,

    /// Input files to process
    pub files: Vec<String>,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}

pub fn echo_args(args: &CliArgs) {
    if args.dry_run {
        println!("Performing a dry run");
    }
    if args.verbose || args.dry_run {
        println!("Verbose: {}", args.verbose);
        println!("Dry run: {}", args.dry_run);
        println!("SQLTypes file: {}", args.sqltypes_file);
        println!("Create SQLTypes: {}", args.create_sqltypes);
        println!("Database URL: {}", args.db_url);
        println!("SQL file: {}", args.sql_file);
        println!("Import unknown foreign keys: {}", args.do_import);
        println!("Module file: {}", args.module_file);
        println!("Settings file: {}", args.settings_file);
        println!("Project file: {}", args.project_file);
        println!("Input files: {:?}", args.files);
    };
}
