//! `mwemu-x86-test` — instruction-semantics test harness for libmwemu.
//!
//! It reads the pooled x86Tester corpus (the same hardware-oracle test vectors
//! that `remill-tester` consumes), replays each row through libmwemu one
//! instruction at a time, and diffs the resulting CPU state against the oracle.
//! Rows that reference state libmwemu does not model yet (x87/MMX/AVX-512, etc.)
//! are reported as skips, not failures.

mod corpus;
mod driver;
mod keys;

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

use clap::{App, Arg};

use driver::{Options, Verdict};

#[derive(Default)]
struct Tally {
    cases: usize,
    rows: usize,
    pass: usize,
    fail: usize,
    unsupported: usize,
    not_executed: usize,
}

fn collect_inputs(files: Vec<PathBuf>, dir: Option<&str>) -> Vec<PathBuf> {
    let mut inputs = files;
    if let Some(dir) = dir {
        if let Ok(entries) = fs::read_dir(dir) {
            let mut found: Vec<PathBuf> = entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().map(|x| x == "txt").unwrap_or(false))
                .collect();
            found.sort();
            inputs.extend(found);
        } else {
            eprintln!("warning: cannot read --input-dir {dir}");
        }
    }
    inputs
}

fn run_file(
    path: &Path,
    mnemonics: &Option<HashSet<String>>,
    limit: Option<usize>,
    opts: &Options,
    stop_on_first_fail: bool,
    verbose: bool,
    tally: &mut Tally,
) -> bool {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error: cannot read {}: {e}", path.display());
            return true;
        }
    };
    let cases = match corpus::parse(&text) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {}: {e}", path.display());
            return true;
        }
    };

    for case in &cases {
        if let Some(set) = mnemonics {
            if !set.contains(&case.mnemonic) {
                continue;
            }
        }
        tally.cases += 1;
        for (i, row) in case.rows.iter().enumerate() {
            if let Some(n) = limit {
                if tally.rows >= n {
                    return true;
                }
            }
            tally.rows += 1;
            match driver::run_row(case, row, opts) {
                Verdict::Pass => tally.pass += 1,
                Verdict::Unsupported(reason) => {
                    tally.unsupported += 1;
                    if verbose {
                        println!(
                            "SKIP {} @0x{:x} [{}] row {}: {reason}",
                            case.mnemonic, case.address, case.asm, i
                        );
                    }
                }
                Verdict::NotExecuted => {
                    tally.not_executed += 1;
                    if verbose {
                        println!(
                            "NOEXEC {} @0x{:x} [{}] row {}",
                            case.mnemonic, case.address, case.asm, i
                        );
                    }
                }
                Verdict::Fail(diffs) => {
                    tally.fail += 1;
                    println!(
                        "FAIL {} @0x{:x} [{}] row {}",
                        case.mnemonic, case.address, case.asm, i
                    );
                    for d in diffs {
                        println!("     {d}");
                    }
                    if stop_on_first_fail {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn main() {
    // Keep libmwemu quiet unless the user opts into its logging.
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("off"))
        .try_init();

    let matches = App::new("mwemu-x86-test")
        .about("Test libmwemu instruction semantics against hardware-oracle x86-64 test vectors")
        .arg(
            Arg::with_name("input")
                .multiple(true)
                .help("Corpus .txt files"),
        )
        .arg(
            Arg::with_name("input-dir")
                .long("input-dir")
                .takes_value(true)
                .help("Run every .txt file in this directory"),
        )
        .arg(
            Arg::with_name("mnemonic")
                .long("mnemonic")
                .takes_value(true)
                .help("Only run these mnemonics (comma-separated)"),
        )
        .arg(
            Arg::with_name("limit")
                .long("limit")
                .takes_value(true)
                .help("Stop after this many rows"),
        )
        .arg(
            Arg::with_name("stop-on-first-fail")
                .long("stop-on-first-fail")
                .help("Exit at the first mismatch"),
        )
        .arg(
            Arg::with_name("no-flags")
                .long("no-flags")
                .help("Do not compare RFLAGS (until per-instruction undefined-flag masking lands)"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("Print each skipped (unsupported / not-executed) row"),
        )
        .get_matches();

    let files: Vec<PathBuf> = matches
        .values_of("input")
        .map(|v| v.map(PathBuf::from).collect())
        .unwrap_or_default();
    let inputs = collect_inputs(files, matches.value_of("input-dir"));
    if inputs.is_empty() {
        eprintln!("no input: pass corpus .txt files or --input-dir <dir>");
        exit(2);
    }

    let mnemonics = matches.value_of("mnemonic").map(|s| {
        s.split(',')
            .map(str::trim)
            .filter(|x| !x.is_empty())
            .map(|x| x.to_ascii_lowercase())
            .collect::<HashSet<_>>()
    });
    let limit = matches
        .value_of("limit")
        .and_then(|s| s.parse::<usize>().ok());
    let opts = Options {
        compare_flags: !matches.is_present("no-flags"),
    };
    let stop_on_first_fail = matches.is_present("stop-on-first-fail");
    let verbose = matches.is_present("verbose");

    let mut tally = Tally::default();
    for path in &inputs {
        let keep_going = run_file(
            path,
            &mnemonics,
            limit,
            &opts,
            stop_on_first_fail,
            verbose,
            &mut tally,
        );
        if !keep_going {
            break;
        }
    }

    println!(
        "\n{} cases, {} rows: {} pass, {} fail, {} unsupported, {} not-executed",
        tally.cases, tally.rows, tally.pass, tally.fail, tally.unsupported, tally.not_executed
    );

    exit(if tally.fail > 0 { 1 } else { 0 });
}
