/// A simple benchmarker to dump a result of testrun(s)
/// Requirement: GNU parallel installed in your PATH
/// Usage: sat-benchmark [OPTIONS] [solvers]
/// # Examples:
///   - sat-bench -3 250 minisat             # 3-SAT from 150 (default) to 250 vars
///   - sat-bench -3 250 -s mios             # 3-SATs and your set of structured problems
///   - sat-bench -o "-cla-decay 0.9" -s glucose     # options to solver
///   - sat-bench -t ./g2-ACG-15-10p1.cnf -s glucose # -t for a CNF file
///   - sat-bench -t '../test / *.cnf' -s glucose      # -t for CNF files
use regex::Regex;
use std::fs;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::Command;
use std::time::SystemTime;
use structopt::StructOpt;

const VERSION: &'static str = "sat-benchmark 0.90.0";
const SAT_PROBLEMS: [(usize, &'static str); 7] = [
    (100, "3-SAT/UF100"),
    (125, "3-SAT/UF125"),
    (150, "3-SAT/UF150"),
    (175, "3-SAT/UF175"),
    (200, "3-SAT/UF200"),
    (225, "3-SAT/UF225"),
    (250, "3-SAT/UF250"),
];
const UNSAT_PROBLEMS: [(usize, &'static str); 1] = [(250, "3-SAT/UUF250")];
const STRUCTURED_PROBLEMS: [(&'static str, &'static str); 4] = [
    ("itox", "SR2015/itox_vc1130.cnf"),
    ("m283", "SR2015/manthey_DimacsSorter_28_3.cnf"),
    ("38b", "SR2015/38bits_10.dimacs.cnf"),
    ("44b", "SR2015/44bits_11.dimacs.cnf"),
];

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "sat-bench", about = "A SAT benchmarking utility")]
#[structopt(name = "sat-bench", about = "Run simple benchmark")]
struct Config {
    solvers: Vec<String>,
    #[structopt(long = "targets", short = "t", default_value = "")]
    targets: String,
    #[structopt(long = "from", short = "L", default_value = "200")]
    range_from: usize,
    #[structopt(long = "upto", short = "U", default_value = "250")]
    range_to: usize,
    #[structopt(long = "3SAT", short = "3")]
    three_sat_set: bool,
    #[structopt(long = "structured", short = "s")]
    structured_set: bool,
    #[structopt(long = "timeout", short = "T", default_value = "510")]
    timeout: usize,
    #[structopt(long = "terminate-hook", default_value = "finished")]
    terminate_hook: String,
    #[structopt(long = "options", default_value = "")]
    solver_options: String,
    #[structopt(long = "header", short = "H", default_value = "")]
    header: String,
    #[structopt(long = "message", short = "M", default_value = "")]
    message: String,
    #[structopt(long = "aux-key", short = "K", default_value = "")]
    aux_key: String,
}

fn main() {
    println!("{}", VERSION);
    let config = Config::from_args();
    let base = env!("PWD");
    let single_solver = match config.solvers.len() {
        0 => panic!("no solver"),
        1 => true,
        _ => false,
    };
    let extra_message = if config.message == "" {
        "".to_string()
    } else {
        format!(", {}", config.message)
    };
    let date = Command::new("date")
        .arg("-Iseconds")
        .output()
        .expect("failed to execute process")
        .stdout;
    let d = String::from_utf8_lossy(&date[..date.len() - 1]);
    let host = Command::new("hostname")
        .arg("-s")
        .output()
        .expect("failed to execute process")
        .stdout;
    let h = String::from_utf8_lossy(&host[..host.len() - 1]);
    // io::stdout().write_all(&output.stdout).unwrap();
    println!(
        "# {}, t={}, p='{}' on {} @ {}{}",
        VERSION, config.timeout, config.solver_options, h, d, extra_message
    );
    if single_solver {
        print_solver(&config.solvers.get(0).unwrap());
    }
    match config.header.as_ref() {
        "" => println!(
            "{:<14}{:>3},{:>16}{:>8}",
            "solver,", "num", "target,", "time"
        ),
        _ => println!("{}", config.header),
    }
    for solver in &config.solvers {
        if !single_solver {
            print_solver(solver);
        }
        let mut num: usize = 1;
        if config.targets.is_empty() {
            if config.three_sat_set {
                for (n, s) in &SAT_PROBLEMS {
                    if config.range_from <= *n && *n <= config.range_to {
                        let dir = format!("{}/{}", base, s);
                        execute_3sats(&config, solver, num, *n, &dir);
                        num += 1;
                    }
                }
                for (n, s) in &UNSAT_PROBLEMS {
                    if config.range_from <= *n && *n <= config.range_to {
                        let dir = format!("{}/{}", base, s);
                        execute_3sats(&config, solver, num, *n, &dir);
                        num += 1;
                    }
                }
            }
            if config.structured_set {
                for (k, s) in &STRUCTURED_PROBLEMS {
                    let cnf = format!("{}/{}", base, s);
                    execute(&config, solver, num, k, &cnf);
                    num += 1;
                }
            }
        } else {
            for t in config.targets.split_whitespace() {
                execute(&config, solver, num, t, t);
                num += 1;
            }
        }
    }
    if !config.terminate_hook.is_empty() {
        let _ = Command::new(config.terminate_hook).output();
    }
}

fn print_solver(solver: &str) -> Option<String> {
    let mut which = match Command::new("which").arg(&solver).output() {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        _ => return None,
    };
    which.pop();
    // printf 更新時刻とフルパス、バージョンのみ表示
    let version = match Command::new(solver).arg("--version").output() {
        Ok(o) => String::from_utf8_lossy(&o.stdout[..o.stdout.len() - 1]).to_string(),
        _ => String::from("???"),
    };
    let at = match Command::new("date").arg("--iso-8601=seconds").output() {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        _ => String::from("???"),
    };
    println!("# {} ({}) @ {}", which, version, &at[..at.len() - 1]);
    Some(which.to_string())
}

/// show the average or total result of SAT problems
#[allow(unused_variables)]
fn execute_3sats(config: &Config, solver: &str, num: usize, n: usize, dir: &str) {
    let solver_name = format!("{}{}", solver, config.aux_key);
    let mut count: usize = 0;
    let start = SystemTime::now();
    for e in fs::read_dir(dir).unwrap() {
        if let Ok(f) = e {
            print!(
                "\x1B[1G\x1B[032mRunning on {}...\x1B[000m",
                f.path().file_name().unwrap().to_str().unwrap()
            );
            stdout().flush().unwrap();
            let mut run = Command::new("timeout");
            let mut command = run.arg(format!("{}", config.timeout)).set_solver(solver);
            for opt in config.solver_options.split_whitespace() {
                command = command.arg(&opt[opt.starts_with('\\') as usize..]);
            }
            match command.arg(f.path()).output() {
                Ok(out) => {
                    if !out.status.success() {
                        let end = match start.elapsed() {
                            Ok(e) => e.as_secs() as f64 + f64::from(e.subsec_millis()) / 1000.0f64,
                            Err(_) => config.timeout as f64,
                        };
                        if config.timeout as f64 <= end {
                            println!(
                                "\x1B[1G\x1B[0K{:<14}{:>3},{:>16}{}",
                                &format!("\"{}\",", solver_name),
                                num,
                                &format!("\"UF{}\",", n),
                                &format!(
                                    "TIMEOUT at {} ({:.3})",
                                    f.file_name().to_str().unwrap(),
                                    end
                                ),
                            );
                        }
                        return;
                    }
                    count += 1;
                }
                Err(_) => panic!("timeout"),
            };
        }
    }
    let end: f64 = match start.elapsed() {
        Ok(e) => e.as_secs() as f64 + f64::from(e.subsec_millis()) / 1000.0f64,
        Err(_) => 0.0f64,
    };
    println!(
        "\x1B[1G\x1B[0K{:<14}{:>3},{:>16}{:>8.3}",
        &format!("\"{}\",", solver_name),
        num,
        &format!(
            "\"{}({})\",",
            PathBuf::from(dir).file_name().unwrap().to_string_lossy(),
            count
        ),
        end,
    );
}

#[allow(unused_variables)]
fn execute(config: &Config, solver: &str, num: usize, name: &str, target: &str) {
    let solver_name = format!("{}{}", solver, config.aux_key);
    for e in target.split_whitespace() {
        let f = PathBuf::from(e);
        if f.is_file() {
            print!(
                "\x1B[1G\x1B[032mRunning on {}...\x1B[000m",
                f.file_name().unwrap().to_str().unwrap()
            );
            stdout().flush().unwrap();
            let start = SystemTime::now();
            let mut run = Command::new("timeout");
            let mut command = run.arg(format!("{}", config.timeout)).set_solver(solver);
            for opt in config.solver_options.split_whitespace() {
                command = command.arg(&opt[opt.starts_with('\\') as usize..]);
            }
            match command.arg(f.as_os_str()).output() {
                Ok(out) => {
                    if !out.status.success() {
                        let end = match start.elapsed() {
                            Ok(e) => e.as_secs() as f64 + f64::from(e.subsec_millis()) / 1000.0f64,
                            Err(_) => config.timeout as f64,
                        };
                        if config.timeout as f64 <= end {
                            println!(
                                "\x1B[1G\x1B[0K{:<14}{:>3},{:>16}{:>8}",
                                &format!("\"{}\",", solver_name),
                                num,
                                &format!("\"{}\",", name),
                                "TIMEOUT",
                            );
                        }
                        continue;
                    }
                }
                Err(_) => println!("timeout"),
            };
            let end = match start.elapsed() {
                Ok(e) => e.as_secs() as f64 + f64::from(e.subsec_millis()) / 1000.0f64,
                Err(_) => 0.0f64,
            };
            println!(
                "\x1B[1G\x1B[0K{:<14}{:>3},{:>16}{:>8.3}",
                &format!("\"{}\",", solver_name),
                num,
                &format!("\"{}\",", name),
                end,
            );
        }
    }
}

trait SolverHandling {
    fn set_solver(&mut self, solver: &str) -> &mut Self;
}

impl SolverHandling for Command {
    fn set_solver(&mut self, solver: &str) -> &mut Command {
        // FIXME: use lazy-static to reuse compiled regexs
        let glucose = Regex::new(r"\bglucose").expect("wrong regex");
        let lingeling = Regex::new(r"\blingeling").expect("wrong regex");
        let minisat = Regex::new(r"\bminisat").expect("wrong regex");
        let mios = Regex::new(r"\bmios").expect("wrong regex");
        let splr = Regex::new(r"\bsplr").expect("wrong regex");
        if splr.is_match(solver) {
            self.args(&[solver, "-r", "-"])
        } else if glucose.is_match(solver) {
            self.args(&[solver, "-verb=0"])
        } else if lingeling.is_match(solver) {
            self.arg(solver)
        } else if mios.is_match(solver) {
            self.arg(solver)
        } else if minisat.is_match(solver) {
            self.arg(solver)
        } else {
            self.arg(solver)
        }
    }
}
