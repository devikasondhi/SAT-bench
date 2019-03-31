/// A simple SAT benchmarker
use chrono::offset::TimeZone;
use chrono::{DateTime, Local};
// use http::{header, Request, Response, StatusCode};
#[macro_use]
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::StructOpt;
use serenity::builder::GetMessages;
use serenity::client::{Client, Context, EventHandler};
use serenity::command;
use serenity::framework::standard::StandardFramework;
use serenity::model::channel::Channel;
use serenity::model::gateway::Game;
use serenity::model::id::{ChannelId, GuildId};
use serenity::model::prelude::Ready;
use serenity::model::prelude::{Message, PresenceUpdateEvent};
// use serenity::framework::standard::Args;
use std::sync::RwLock;
use std::{env, thread, time::Duration};

lazy_static! {
    pub static ref M: &'static str = "a";
    pub static ref N: RwLock<usize> = RwLock::new(0);
}

fn main() {
    run_bot()
}

struct Handler;
impl EventHandler for Handler {
    fn ready(&self, ctx: Context, _: Ready) {
        println!("ready");
        let name = "Let's give a try!";
        ctx.set_game(Game::playing(&name));
    }
    fn webhook_update(&self, ctx: Context, _guild_id: GuildId, _c: ChannelId) {
        println!("webhook");
        let name = "Something happen?";
        ctx.set_game(Game::playing(&name));
    }
    fn presence_update(&self, ctx: Context, _: PresenceUpdateEvent) {
        println!("update presence");
        let name = "Someone comes back.";
        ctx.set_game(Game::playing(&name));
    }
    fn message(&self, ctx: Context, m: Message) {
        if let Ok(n) = N.read() {
            let name = format!("Something {} happen.", n);
            ctx.set_game(Game::playing(&name));
        }
        if m.content == "#clear" {
            run_cleaner();
        }
    }
}

fn run_bot() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    thread::spawn(move || {
        let millis = Duration::from_millis(20 * 1000);
        loop {
            // call solver here
            if let Ok(mut n) = N.write() {
                *n += 1;
                println!("ooo {}", *n);
                if let Ok(channel) = ChannelId(
                    env::var("DISCORD_CHANNEL")
                        .expect("no channel")
                        .parse::<u64>()
                        .unwrap(),
                )
                    .to_channel()
                {
                    if let Channel::Guild(gchannel) = &channel {
                        let ch = gchannel.read();
                        if *n % 10 == 0 {
                            ch.broadcast_typing().expect("oo");
                            // ch.say("#clear").expect("fail");
                        //} else {
                          //  ch.say("not yet").expect("failed");
                        }
                    }
                }
                let _name = format!("Let's give a try {}!", n);
                // ctx.set_game(Game::playing(&name));
            }
            thread::sleep(millis);
        }
    });
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("#"))
            .cmd("ping", ping)
            .cmd("clear", clean),
    );
    let shard_manager = client.shard_manager.clone();
    thread::spawn(move || {
        loop {
            println!("Shard count instantiated: {}",
                     shard_manager.lock().shards_instantiated().len());
    
            thread::sleep(Duration::from_millis(5000));
        }
    });
    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

fn run_cleaner() {
    if let Ok(channel) = ChannelId(
        env::var("DISCORD_CHANNEL")
            .expect("no channel")
            .parse::<u64>()
            .unwrap(),
    )
    .to_channel()
    {
        if let Channel::Guild(gchannel) = &channel {
            let ch = gchannel.read();
            match ch.say("Hi, I'm going to clean your channel up...") {
                Ok(m) => cleaner(&channel, &m).expect("failed"),
                _ => panic!("fail!"),
            }
        }
    }
}

fn cleaner(channel: &Channel, message: &Message) -> serenity::Result<()> {
    let test = env::var("BOT_TEST")
        .unwrap_or_else(|_| "1".to_string())
        .parse::<usize>()
        .unwrap_or(1)
        == 1;
    if let Channel::Guild(gchannel) = channel {
        let ch = gchannel.read();
        let retriever = GetMessages::default().before(message.id).limit(40);
        match ch.messages(|_| retriever) {
            Ok(ref v) if !test => {
                let len = v.len();
                let mut n = 0;
                for (i, m) in v[len / 2..].iter().enumerate() {
                    match m.delete() {
                        Ok(_) => n += 1,
                        Err(why) => println!("{} -> {}", i, why),
                    }
                }
                ch.say(&format!(
                    "I've deleted {} messages out of {}. Good bye.",
                    n, len
                ))?;
            }
            Ok(_) => {
                ch.say("Yay, succeeded to test APIs. Let's switch to real mode.")?;
            }
            Err(e) => {
                ch.say(&format!("Error {}", e))?;
            }
        }
    }
    Ok(())
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});

command!(game(ctx, _msg, _args) {
    let name = "Let's give a try!";
    ctx.set_game(Game::playing(&name));
});

command!(clean(context, message) {
    context.set_game(Game::playing("Hmm"));
    let test = env::var("BOT_TEST")
        .unwrap_or_else(|_| "1".to_string())
        .parse::<usize>()
        .unwrap_or(1)
        == 1;
    let ch = message.channel_id;
    let retriever = GetMessages::default().before(message.id).limit(100);
    match ch.messages(|_| retriever) {
        Ok(ref v) if !test => {
            let len = v.len();
            let mut _n = 0;
            for (i, m) in v[len / 2..].iter().enumerate() {
                match m.delete() {
                    Ok(_) => _n += 1,
                    Err(why) => println!("{} -> {}", i, why),
                }
            }
            message.delete().unwrap();
        }
        Ok(_) => {
            ch.say("Yay, succeeded to test APIs. Let's switch to real mode.")?;
        }
        Err(e) => {
            ch.say(&format!("Error {}", e))?;
        }
    }
});
/////////////////////////

const VERSION: &str = "benchbot 0.0.0";
const STRUCTURED_PROBLEMS: [(&str, &str); 4] = [
    ("SR2015/itox", "SR2015/itox_vc1130.cnf"),
    ("SR2015/m283", "SR2015/manthey_DimacsSorter_28_3.cnf"),
    ("SR2015/38b", "SR2015/38bits_10.dimacs.cnf"),
    ("SR2015/44b", "SR2015/44bits_11.dimacs.cnf"),
];
const CLEAR: &str = "\x1B[1G\x1B[0K";

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "sat-bench", about = "Run simple SAT benchmarks")]
struct Config {
    /// solvers names
    solvers: Vec<String>,
    /// Structured instances
    #[structopt(long = "structured", short = "s")]
    structured_set: bool,
    /// time out in seconds
    #[structopt(long = "timeout", short = "T", default_value = "510")]
    timeout: usize,
    /// command to be executed after end of run
    #[structopt(long = "terminate-hook", default_value = "finished")]
    terminate_hook: String,
    /// arguments passed to solvers
    #[structopt(long = "options", default_value = "")]
    solver_options: String,
    ///  additinal string used in header
    #[structopt(long = "message", short = "M", default_value = "")]
    message: String,
    /// additional string following solver name
    #[structopt(long = "aux-key", short = "K", default_value = "")]
    aux_key: String,
    /// data directory
    #[structopt(long = "lib", default_value = "")]
    lib_dir: String,
}

pub fn main_exp() {
    let config = Config::from_args();
    let base = if config.lib_dir.is_empty() {
        match option_env!("SATBENCHLIB") {
            Some(dir) => dir,
            None => env!("PWD"),
        }
    } else {
        &config.lib_dir
    };
    let single_solver = match config.solvers.len() {
        0 => {
            println!("Abort: no solver");
            return;
        }
        1 => true,
        _ => false,
    };
    let extra_message = if config.message == "" {
        "".to_string()
    } else {
        format!(", {}", config.message)
    };
    let host = Command::new("hostname")
        .arg("-s")
        .output()
        .expect("failed to execute process")
        .stdout;
    let h = String::from_utf8_lossy(&host[..host.len() - 1]);
    if config.solver_options.is_empty() {
        println!(
            "# {}, timeout:{} on {} @ {}{}",
            VERSION,
            config.timeout,
            h,
            system_time_to_date_time(SystemTime::now())
                .format("%F-%m-%dT%H:%M:%S")
                .to_string(),
            extra_message
        );
    } else {
        println!(
            "# {}, timeout:{}, options:'{}' on {} @ {}{}",
            VERSION,
            config.timeout,
            config.solver_options,
            h,
            system_time_to_date_time(SystemTime::now())
                .format("%F-%m-%dT%H:%M:%S")
                .to_string(),
            extra_message
        );
    }
    if single_solver {
        print_solver(&config.solvers[0]);
    }
    println!(
        "{:<14}{:>3},{:>20}{:>8}",
        "solver,", "num", "target,", "time"
    );
    for solver in &config.solvers {
        if !single_solver {
            print_solver(solver);
        }
        let mut num: usize = 1;
        if config.structured_set {
            for (k, s) in &STRUCTURED_PROBLEMS {
                let cnf = format!("{}/{}", base, s);
                execute(&config, solver, num, k, &cnf);
                num += 1;
            }
        }
    }
    if !config.terminate_hook.is_empty() {
        let _ = Command::new(config.terminate_hook).output();
    }
}


#[allow(unused_variables)]
fn execute(config: &Config, solver: &str, num: usize, name: &str, target: &str) {
    let solver_name = format!("{}{}", solver, config.aux_key);
    for e in target.split_whitespace() {
        let f = PathBuf::from(e);
        if f.is_file() {
            print!(
                "{}\x1B[032mRunning on {}...\x1B[000m",
                CLEAR,
                f.file_name().unwrap().to_str().unwrap()
            );
            stdout().flush().unwrap();
            let start = SystemTime::now();
            let mut run = Command::new("timeout");
            let mut command = run.arg(format!("{}", config.timeout)).set_solver(solver);
            for opt in config.solver_options.split_whitespace() {
                command = command.arg(&opt[opt.starts_with('\\') as usize..]);
            }
            match command
                .arg(f.as_os_str())
                .check_result(solver, &start, config.timeout as f64)
            {
                Some(end) => {
                    println!(
                        "{}{:<14}{:>3},{:>20}{:>8.3}",
                        CLEAR,
                        &format!("\"{}\",", solver_name),
                        num,
                        &format!("\"{}\",", name),
                        end,
                    );
                }
                None => {
                    println!(
                        "{}{:<14}{:>3},{:>20}{:>8}",
                        CLEAR,
                        &format!("\"{}\",", solver_name),
                        num,
                        &format!("\"{}\",", name),
                        "TIMEOUT",
                    );
                }
            };
        }
    }
}

trait SolverHandling {
    fn set_solver(&mut self, solver: &str) -> &mut Self;
    fn check_result(&mut self, solver: &str, start: &SystemTime, timeout: f64) -> Option<f64>;
}

impl SolverHandling for Command {
    fn set_solver(&mut self, solver: &str) -> &mut Command {
        lazy_static! {
            static ref GLUCOSE: Regex = Regex::new(r"\bglucose").expect("wrong regex");
            // static ref lingeling: Regex = Regex::new(r"\blingeling").expect("wrong regex");
            // static ref minisat: Regex = Regex::new(r"\bminisat").expect("wrong regex");
            // static ref mios: Regex = Regex::new(r"\bmios").expect("wrong regex");
            static ref SPLR: Regex = Regex::new(r"\bsplr").expect("wrong regex");
        }
        if SPLR.is_match(solver) {
            self.args(&[solver, "-r", "-"])
        } else if GLUCOSE.is_match(solver) {
            self.args(&[solver, "-verb=0"])
        } else {
            self.arg(solver)
        }
    }
    fn check_result(&mut self, solver: &str, start: &SystemTime, timeout: f64) -> Option<f64> {
        lazy_static! {
            static ref MINISAT_LIKE: Regex =
                Regex::new(r"\b(glucose|minisat)").expect("wrong regex");
        }
        let result = self.output();
        match result {
            Ok(ref done) => {
                match done.status.code() {
                    Some(10) | Some(20) if MINISAT_LIKE.is_match(solver) => (),
                    Some(0) => (),
                    _ => return None,
                }
                match start.elapsed() {
                    Ok(e) => {
                        let end = e.as_secs() as f64 + f64::from(e.subsec_millis()) / 1000.0f64;
                        if end < timeout {
                            Some(end)
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }
}

fn print_solver(solver: &str) -> Option<String> {
    let mut which = match Command::new("which").arg(&solver).output() {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        _ if PathBuf::from(solver).exists() => PathBuf::from(solver)
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .into_owned(),
        _ => return None,
    };
    which = which.trim_end_matches('\n').to_string();
    let at = fs::metadata(&which);
    if let Ok(home) = env::var("HOME") {
        home.trim_end_matches('/');
        let hr = Regex::new(&home).expect("wrong regex");
        which = hr.replace(&which, "~").to_string();
    }
    // printf 更新時刻とフルパス、バージョンのみ表示
    let version = match Command::new(solver).arg("--version").output() {
        Ok(o) => String::from_utf8_lossy(&o.stdout[..o.stdout.len() - 1]).to_string(),
        _ => String::from("???"),
    };
    print!("# {} ({})", which, version);
    if let Ok(meta) = at {
        if let Ok(time) = meta.modified() {
            println!(
                " @ {}",
                system_time_to_date_time(time)
                    .format("%F-%m-%dT%H:%M:%S")
                    .to_string()
            );
        }
    } else {
        println!();
    }
    Some(which.to_string())
}

// See https://users.rust-lang.org/t/convert-std-time-systemtime-to-chrono-datetime-datetime
fn system_time_to_date_time(t: SystemTime) -> DateTime<Local> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => {
            // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        }
    };
    Local.timestamp(sec, nsec)
}

