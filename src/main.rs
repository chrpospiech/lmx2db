pub(crate) mod cmdline;

fn main() {
    let args = cmdline::parse_args();
    if args.verbose {
        println!("Verbose mode is on");
    }
    if args.dry_run {
        println!("Performing a dry run");
    }
}
