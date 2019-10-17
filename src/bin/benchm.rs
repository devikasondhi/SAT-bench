/// A simple SAT benchmarker
use lazy_static::lazy_static;
use regex::Regex;
use sat_bench::matrix;
use sat_bench::utils::current_date_time;
use sat_bench::{bench18::SCB, utils::parse_result};
use std::collections::HashMap;
use std::fs;
use std::io::{stdout, BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;
use std::str;
use std::sync::RwLock;
use std::{env, time};
use structopt::StructOpt;

const VERSION: &str = "benchbot 0.6.6";

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
    pub static ref DIFF: RwLock<String> = RwLock::new(String::new());
    pub static ref PQ: RwLock<Vec<(usize, String)>> = RwLock::new(Vec::new());
    pub static ref PROCESSED: RwLock<usize> = RwLock::new(0);
    pub static ref ANSWERED: RwLock<usize> = RwLock::new(0);
    pub static ref M: RwLock<String> = RwLock::new(String::new());
    pub static ref RUN: RwLock<String> = RwLock::new(String::new());
    pub static ref N: RwLock<usize> = RwLock::new(0);
}

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "sat-bench", about = "Run the SAT Competition benchmark")]
pub struct Config {
    /// solver names
    #[structopt(long = "solver", short = "s", default_value = "")]
    pub solver: String,
    /// start of the range of target problems
    #[structopt(long = "from", default_value = "0")]
    pub target_from: usize,
    /// end of the range of target problems
    #[structopt(long = "to", default_value = "400")]
    pub target_to: usize,
    /// step of choosen target problems
    #[structopt(long = "step", default_value = "1")]
    pub target_step: usize,
    /// time out in seconds
    #[structopt(long = "timeout", short = "T", default_value = "1000")]
    pub timeout: usize,
    /// number of workers
    #[structopt(long = "jobs", short = "j", default_value = "4")]
    pub num_jobs: usize,
    /// arguments passed to solvers
    #[structopt(long = "options", default_value = "")]
    pub solver_options: String,
    /// data directory
    #[structopt(long = "data", default_value = "~/Library/SAT/SC18main")]
    pub data_dir: PathBuf,
    /// solver repository
    #[structopt(long = "repo", default_value = "~/Repositories/splr")]
    pub repo_dir: PathBuf,
    /// cloud sharing directory
    #[structopt(long = "sync", default_value = "~/Desktop/splr-exp")]
    pub sync_dir: PathBuf,
    /// cloud sync command
    #[structopt(long = "sync-cmd", default_value = "")]
    pub sync_cmd: String,
    /// Don't assign
    #[structopt(long = "dump", default_value = "")]
    pub dump_dir: PathBuf,
    /// Don't assign
    #[structopt(long = "run", default_value = "")]
    pub run_name: String,
    /// user id to post to Matrix
    #[structopt(long = "matrix-id", default_value = "")]
    pub matrix_id: String,
    /// user password to post to Matrix
    #[structopt(long = "matrix-password", default_value = "")]
    pub matrix_password: String,
    /// The Matrix room
    #[structopt(long = "matrix-room", default_value = "")]
    pub matrix_room: String,
    #[structopt(skip)]
    pub matrix_token: Option<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            solver: String::from("splr"),
            target_from: 0,
            target_to: 400,
            target_step: 1,
            timeout: 5000,
            num_jobs: 3,
            solver_options: String::new(),
            data_dir: PathBuf::new(),
            repo_dir: PathBuf::new(),
            sync_dir: PathBuf::new(),
            sync_cmd: String::new(),
            dump_dir: PathBuf::new(),
            run_name: String::new(),
            matrix_id: String::new(),
            matrix_password: String::new(),
            matrix_room: String::new(),
            matrix_token: None,
       }
    }
}

fn main() {
    let mut config = Config::from_args();
    let tilde = Regex::new("~").expect("wrong reex");
    let home = env::var("HOME").expect("No home");
    config.data_dir = PathBuf::from(
        tilde
            .replace(&config.data_dir.to_string_lossy(), &home[..])
            .to_string(),
    );
    config.sync_dir = PathBuf::from(
        tilde
            .replace(&config.sync_dir.to_string_lossy(), &home[..])
            .to_string(),
    );
    config.repo_dir = PathBuf::from(
        tilde
            .replace(&config.repo_dir.to_string_lossy(), &home[..])
            .to_string(),
    );
    // Matrix
    if !config.matrix_id.is_empty() {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("user", &config.matrix_id);
        map.insert("password", &config.matrix_password);
        config.matrix_token = matrix::get_token(&map);
    }
    start_benchmark();
}

fn start_benchmark() {
    let mut config = if let Ok(conf) = CONFIG.read() {
        conf.clone()
    } else {
        Config::from_args()
    };
    if config.solver.is_empty() {
        config.solver = "splr".to_string();
        for e in config.repo_dir.join("src/bin").read_dir().expect("no repo") {
            if let Ok(f) = e {
                let splr = f.path().file_stem().unwrap().to_string_lossy().to_string();
                if splr.contains("splr") {
                    print!("\x1B[032mCompiling {}...\x1B[000m", config.solver);
                    stdout().flush().unwrap();
                    Command::new("cargo")
                        .current_dir(&config.repo_dir)
                        .args(&["install", "--path", ".", "--force"])
                        .output()
                        .expect("fail to compile");
                    config.solver = splr;
                    println!("\x1B[032mdone.\x1B[000m");
                }
            }
        }
    }
    let host = {
        let h = Command::new("hostname")
            .arg("-s")
            .output()
            .expect("failed to execute process")
            .stdout;
        String::from_utf8_lossy(&h[..h.len() - 1]).to_string()
    };
    config.run_name = {
        let commit_id_u8 = Command::new("git")
            .current_dir(&config.repo_dir)
            .args(&["log", "-1", "--format=format:%h"])
            .output()
            .expect("fail to git")
            .stdout;
        let commit_id = unsafe { String::from_utf8_unchecked(commit_id_u8) };
        let timestamp = current_date_time().format("%F").to_string();
        format!("{}-{}-{}-{}", config.solver, commit_id, timestamp, host)
    };
    let diff = {
        let diff8 = Command::new("git")
            .current_dir(&config.repo_dir)
            .args(&["diff"])
            .output()
            .expect("fail to git diff")
            .stdout;
        unsafe { String::from_utf8_unchecked(diff8) }
    };
    if let Ok(mut d) = DIFF.write() {
        *d = diff.clone();
    }
    if let Ok(mut run) = RUN.write() {
        *run = config.run_name.clone();
    }
    config.dump_dir = PathBuf::from(&config.run_name);
    if config.dump_dir.exists() {
        println!("WARNING: {} exists.", config.dump_dir.to_string_lossy());
    } else {
        fs::create_dir(&config.dump_dir).expect("fail to mkdir");
    }
    if let Ok(mut queue) = PQ.write() {
        for s in SCB.iter().take(config.target_to).skip(config.target_from) {
            if s.0 % config.target_step == 0 {
                queue.push((s.0, s.1.to_string()));
            }
        }
        queue.reverse();
    }
    if let Ok(mut processed) = PROCESSED.write() {
        *processed = config.target_from;
    }
    if let Ok(mut answered) = ANSWERED.write() {
        let (s, u) = report(&config).unwrap_or((0, 0));
        *answered = s + u;
    }
    matrix::post(&config.matrix_token,
                 &format!(
                     "A new {} parallel benchmark starts.",
                     config.num_jobs,
                 )
    );
    if !diff.is_empty() {
        matrix::post(&config.matrix_token,
                     &format!(
                         "**WARNING: unregistered modifications**\n```diff\n{}```\n",
                         diff
                     )
        );
    }
    crossbeam::scope(|s| {
        for i in 0..config.num_jobs {
            let c = config.clone();
            s.spawn(move |_| {
                std::thread::sleep(time::Duration::from_millis((2 + 2 * i as u64) * 1000));
                worker(c);
            });
    }
        }).unwrap();

    if let Ok(mut conf) = CONFIG.write() {
        *conf = config.clone();
    }
}

fn worker(config: Config) {
    loop {
        let mut new_solution = false;
        let pro = PROCESSED.read().and_then(|v| Ok(*v)).unwrap_or(0);
        if pro % config.num_jobs == 0 {
            let (s, u) = report(&config).unwrap_or((0, 0));
            if let Ok(mut answered) = ANSWERED.write() {
                let sum = s + u;
                if *answered < sum {
                    new_solution = true;
                    *answered = sum;
                }
            }
        }
        let ans = ANSWERED.read().and_then(|v| Ok(*v)).unwrap_or(0);
        if config.timeout == 1000 && 4 <= config.num_jobs {
            if (pro == 20 && 3 < ans)
                || (pro == 40 && 4 < ans)
                || (pro == 60 && 20 < ans)
                || (pro == 80 && 26 < ans)
                || (pro == 100 && 42 < ans)
                || (pro == 120 && 54 < ans)
                || (pro == 140 && 56 < ans)
                || (pro == 160 && 58 < ans)
                || (pro == 180 && 63 < ans)
                || (pro == 200 && 70 < ans)
                || (pro == 220 && 77 < ans)
                || (pro == 240 && 88 < ans)
                || (pro == 260 && 90 < ans)
                || (pro == 280 && 97 < ans)
                || (pro == 300 && 107 < ans)
                || (pro == 320 && 117 < ans)
                || (pro == 340 && 129 < ans)
                || (pro == 360 && 141 < ans)
                || (pro == 380 && 144 < ans)
                || (pro == 400 && 145 < ans)
            {
                print!("*{:>3}-th problem,{:>3} solutions,", pro, ans);
                matrix::post(&config.matrix_token,
                             &format!("*{:>3}-th problem,{:>3} solutions,", pro, ans)
                );
            } else {
                print!(" {:>3}-th problem,{:>3} solutions,", pro, ans);
                stdout().flush().unwrap();
                if new_solution {
                    matrix::post(&config.matrix_token,
                                 &format!(" {:>3}-th problem,{:>3} solutions,", pro, ans)
                    );
                }
            }
        }
        let p: PathBuf;
        let remains: usize;
        if let Ok(mut q) = PQ.write() {
            if let Some((index, top)) = q.pop() {
                p = config.data_dir.join(top);
                remains = q.len();
                if let Ok(mut processed) = PROCESSED.write() {
                    *processed = index;
                }
            } else {
                break;
            }
        } else {
            break;
        }
        execute(&config, &p);
        if remains == 0 {
            // I'm the last one.
            let (s, u) = report(&config).unwrap_or((0, 0));
            if let Ok(mut answered) = ANSWERED.write() {
                let sum = s + u;
                *answered = sum;
            }
            let tarfile = config.sync_dir.join(&format!("{}.tar.xz", config.run_name));
            Command::new("tar")
                .args(&[
                    "cvf",
                    &tarfile.to_string_lossy(),
                    &config.dump_dir.to_string_lossy(),
                ])
                .output()
                .expect("fail to tar");
            if !config.sync_cmd.is_empty() {
                Command::new(&config.sync_cmd)
                    .output()
                    .expect("fail to run sync command");
            }
            println!("Benchmark {} has been done.", config.run_name);
        }
    }
}

fn execute(config: &Config, cnf: &PathBuf) {
    let f = PathBuf::from(cnf);
    if f.is_file() {
        let target: String = f.file_name().unwrap().to_str().unwrap().to_string();
        println!("\x1B[032m{}\x1B[000m", target);
        let mut command: Command = solver_command(config);
        for opt in config.solver_options.split_whitespace() {
            command.arg(&opt[opt.starts_with('\\') as usize..]);
        }
        let result = command.arg(f.as_os_str()).output();
        match &result {
            Err(_) => println!("Something happened to {}.", &target),
            Ok(r)
                if String::from_utf8(r.stderr.clone())
                .unwrap()
                .contains("thread 'main' panicked") =>
            {
                panic!("**Panic at {}.**", &target);
            }
            _ => (),
        }
    }
}

fn solver_command(config: &Config) -> Command {
    lazy_static! {
        static ref GLUCOSE: Regex = Regex::new(r"\bglucose").expect("wrong regex");
        // static ref lingeling: Regex = Regex::new(r"\blingeling").expect("wrong regex");
        // static ref minisat: Regex = Regex::new(r"\bminisat").expect("wrong regex");
        // static ref mios: Regex = Regex::new(r"\bmios").expect("wrong regex");
        static ref SPLR: Regex = Regex::new(r"\bsplr").expect("wrong regex");
    }
    if SPLR.is_match(&config.solver) {
        let mut command = Command::new(&config.solver);
        command.args(&[
            "--to",
            &format!("{}", config.timeout),
            "-o",
            &config.dump_dir.to_string_lossy(),
        ]);
        command
    } else if GLUCOSE.is_match(&config.solver) {
        let mut command = Command::new(&config.solver);
        command.args(&["-verb=0", &format!("-cpu-lim={}", config.timeout)]);
        command
    } else {
        Command::new(&config.solver)
    }
}

fn report(config: &Config) -> std::io::Result<(usize, usize)> {
    let outname = config.sync_dir.join(config.run_name.to_string() + ".csv");
    let mut nsat = 0;
    let mut nunsat = 0;
    {
        let outfile = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&outname)?;
        let mut outbuf = BufWriter::new(outfile);
        let mut hash: HashMap<&str, (f64, String, bool)> = HashMap::new();
        let timeout = config.timeout as f64;
        let processed = if let Ok(p) = PROCESSED.read() { *p } else { 0 };
        for e in config.dump_dir.read_dir()? {
            let f = e?;
            if !f.file_type()?.is_file() {
                continue;
            }
            let fname = f.file_name().to_string_lossy().to_string();
            if fname.starts_with(".ans_") {
                let cnf = &fname[5..];
                for (_n, key) in SCB.iter() {
                    if *key == cnf {
                        if None != hash.get(key) {
                            panic!("duplicated {}", cnf);
                        }
                        if let Some((t, s, m)) = parse_result(f.path()) {
                            if s {
                                nsat += 1;
                            } else {
                                nunsat += 1;
                            }
                            hash.insert(key, (timeout.min(t), m, s));
                            break;
                        }
                    }
                }
            }
        }
        writeln!(
            outbuf,
            "#{} by {}: from {} to {}\n# process: {}, timeout: {}\n# Procesed: {}, total answers: {} (SAT: {}, UNSAT: {}) so far",
            config.run_name,
            VERSION,
            config.target_from,
            config.target_to,
            config.num_jobs,
            config.timeout,
            processed,
            nsat + nunsat,
            nsat,
            nunsat,
        )?;
        if let Ok(diff) = DIFF.write() {
            for l in diff.lines() {
                writeln!(outbuf, "# {}", l)?;
            }
        }
        writeln!(
            outbuf,
            "solver, num, target, nsolved, time, strategy, satisfiability"
        )?;
        let mut nsolved = 0;
        for (i, key) in SCB.iter() {
            if let Some(v) = hash.get(key) {
                nsolved += 1;
                writeln!(
                    outbuf,
                    "\"{}\",{},\"SC18main/{}\",{:>3},{:>8.2},{},{}",
                    config.dump_dir.to_string_lossy(),
                    i,
                    key,
                    nsolved,
                    v.0,
                    v.1,
                    v.2,
                )?;
            } else {
                writeln!(
                    outbuf,
                    "\"{}\",{},\"SC18main/{}\",{:>3},{:>5},,",
                    config.dump_dir.to_string_lossy(),
                    i,
                    key,
                    nsolved,
                    config.timeout,
                )?;
            }
        }
    }
    Command::new("make")
        .current_dir(&config.sync_dir)
        .output()?;
    if !config.sync_cmd.is_empty() {
        Command::new(&config.sync_cmd).output()?;
    }
    Ok((nsat, nunsat))
}
