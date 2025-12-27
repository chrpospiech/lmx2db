use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "lmx2db", author = "C. Pospiech", version = "0.1", about = "Convert LMX files to database entries", long_about = None)]
pub struct CliArgs {
    /// Perform a dry run without making actual changes
    #[arg(short = 'D', long, default_value_t = false)]
    pub dry_run: bool,

    /// Verbose output
    #[arg(short = 'v', long, default_value_t = false)]
    pub verbose: bool,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}
