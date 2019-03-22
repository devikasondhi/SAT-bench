use regex::Regex;
use sat_bench::bench18::SCB;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::*;
use std::path::PathBuf;
use structopt::StructOpt;

/// Configuration built from command line options
#[derive(Debug, StructOpt)]
#[structopt(
    name = "satbench2csv",
    about = "Convert SAT Competition Benchmark results to a CSV file"
)]
pub struct Config {
    /// directory to scan
    #[structopt(long = "from", default_value = ".")]
    pub from: String,
    /// solver name (use 'from' if this is empty).
    #[structopt(long = "solver", default_value = "")]
    pub solver: String,
    /// value for instances timed out
    #[structopt(long = "timeout", default_value = "3000")]
    pub timeout: usize,
    /// Name for the target set, ending with a delimitor
    #[structopt(long = "target", default_value = "SC18main/")]
    pub target: String,
}

fn main() -> std::io::Result<()> {
    let config = Config::from_args();
    let mut hash: HashMap<&str, (f64, bool, String)> = HashMap::new();
    let tag: &str = if config.solver.is_empty() {
        if config.from.ends_with('/') {
            &config.from[..config.from.len() - 1]
        } else {
            &config.from
        }
    } else {
        &config.solver
    };
    let timeout = config.timeout as f64;
    let mut nsat = 0;
    let mut nunsat = 0;
    for e in fs::read_dir(&config.from)? {
        let f = e?;
        if !f.file_type()?.is_file() {
            continue;
        }
        let fname = f.file_name().to_string_lossy().to_string();
        if fname.starts_with(".ans_") {
            let cnf = &fname[5..];
            for key in SCB.iter() {
                if *key == cnf {
                    if None != hash.get(key) {
                        panic!("duplicated {}", cnf);
                    }
                    if let Some((t, s, m)) = parse_result(f.path()) {
                        hash.insert(key, (timeout.min(t), s, m));
                        if s {
                            nsat += 1;
                        } else {
                            nunsat += 1;
                        }
                        break;
                    }
                }
            }
        }
    }
    println!(
        "# SAT: {}, UNSAT: {}, total: {} so far",
        nsat,
        nunsat,
        nsat + nunsat
    );
    println!("solver, num, target, time, satisfiability, strategy");
    for (i, key) in SCB.iter().enumerate() {
        if let Some(v) = hash.get(key) {
            println!(
                "\"{}\",{},\"{}{}\",{:>8.2},{},{}",
                tag,
                i + 1,
                config.target,
                key,
                v.0,
                v.1,
                v.2,
            );
        } else {
            println!(
                "\"{}\",{},\"{}{}\",{:>5},{},",
                tag,
                i + 1,
                config.target,
                key,
                config.timeout,
                "",
            );
        }
    }
    Ok(())
}

fn parse_result(fname: PathBuf) -> Option<(f64, bool, String)> {
    let f;
    match File::open(fname) {
        Ok(fin) => f = fin,
        Err(_) => return None,
    }
    let mut input = BufReader::new(f);
    let sat = Regex::new(r"\bSATISFIABLE\b").expect("wrong regex");
    let unsat = Regex::new(r"\bUNSATISFIABLE\b").expect("wrong regex");
    let splr = Regex::new(r"^c +Strategy\|mode: +([^,]+), time: +([.0-9]+)").expect("wrong regex");
    let glucose = Regex::new(r"^c CPU time +: ([.0-9]+)").expect("wrong regex");
    let mut buf = String::new();
    let mut time: Option<f64> = None;
    let mut found: Option<bool> = None;
    let mut strategy: String = "".to_string();
    while let Ok(k) = input.read_line(&mut buf) {
        if k == 0 {
            break;
        }
        if sat.is_match(&buf) {
            assert_eq!(found, None);
            found = Some(true);
        } else if unsat.is_match(&buf) {
            assert_eq!(found, None);
            found = Some(false);
        } else if let Some(c) = splr.captures(&buf) {
            strategy = c[1].to_string();
            if let Ok(v) = c[2].parse() {
                time = Some(v);
            }
        } else if let Some(c) = glucose.captures(&buf) {
            if let Ok(v) = c[1].parse() {
                time = Some(v);
            }
        }
        buf.clear();
    }
    match (time, found) {
        (Some(t), Some(f)) => Some((t, f, strategy)),
        _ => None,
    }
}
