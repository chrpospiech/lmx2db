use clap::Parser;

#[derive(Parser, Debug)]
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
        default_value = "mysql://lmxtest:lmxtest@localhost/lmxtest"
    )]
    pub db_url: String,

    /// name of sqlkeys file
    #[arg(short = 'k', long, default_value = "sqlkeys.yml")]
    pub sqlkeys_file: String,

    /// create sqlkeys file from database
    #[arg(short = 'c', long, default_value_t = false)]
    pub create_sqlkeys: bool,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}

pub fn echo_args(args: &CliArgs) {
    if args.verbose {
        println!("Verbose: {}", args.verbose);
        println!("Dry run: {}", args.dry_run);
        println!("SQLKeys file: {}", args.sqlkeys_file);
        println!("Create SQLKeys: {}", args.create_sqlkeys);
        println!("Database URL: {}", args.db_url);
    } else if args.dry_run {
        println!("Performing a dry run");
    }
}
