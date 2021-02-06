use std::path::PathBuf;

// With the "paw" feature enabled in structopt
#[derive(structopt::StructOpt)]
/// Compare line and branch coverage in two XML coverage files
///
/// Compare a Cobertura XML coverage file against a baseline XML file,
/// and return an error if the coverage has dropped. The error is returned
/// as a status code of 1. A zero is returned if coverage is the same
/// or improves.
///
///     EXAMPLE:
///
///     $ covcompare master-coverage.xml this-pr-coverage.xml --tolerance=0.001
///
/// If the two files are present, they will be compared and if the line or
/// branch coverage is lower in the second file, an exit code of 1 will
/// be returned.
struct Args {
    /// The fractional allowance for under-coverage, past which a
    /// status code of 1 will be returned. For example, in a 5000 line
    /// program, 10 lines would be 0.2%. If you want to fail the build
    /// if the drop in coverage is more than 0.2%, this value should be
    /// set to 0.002.
    #[structopt(short = "t", long = "tolerance", default_value = "0.002")]
    tolerance: f64,
    /// The baseline Cobertura xml coverage file, typically for the
    /// master/main branch.
    #[structopt(parse(from_os_str))]
    baseline: PathBuf,
    /// The Cobertura xml coverage file to compare to the baseline. This
    /// would typically be the coverage file generated from the CI
    /// test run.
    #[structopt(parse(from_os_str))]
    change: PathBuf,
}

#[paw::main]
fn main(args: Args) -> Result<(), std::io::Error> {
    let (exit_code, msg) = covcompare::compare(args.baseline, args.change, args.tolerance);
    eprintln!("{}", msg);
    std::process::exit(exit_code as i32);
    // Ok(())
}
