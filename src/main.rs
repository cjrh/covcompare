use covcompare;
use std::path::PathBuf;

// With the "paw" feature enabled in structopt
#[derive(structopt::StructOpt)]
struct Args {
    #[structopt(parse(from_os_str))]
    baseline: PathBuf,
    #[structopt(parse(from_os_str))]
    change: PathBuf,
}

#[paw::main]
fn main(args: Args) -> Result<(), std::io::Error> {
    let (exit_code, msg) = covcompare::compare(args.baseline, args.change);
    eprintln!("{}", msg);
    std::process::exit(exit_code as i32);
    // Ok(())
}
