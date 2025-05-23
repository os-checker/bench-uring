use bench_uring::{
    Result,
    utils::{Config, EnvConfig},
};
use eyre::ContextCompat;
use serde::Serialize;
use std::process::Command;

#[derive(Debug)]
pub struct Examples {
    /// All example names.
    pub all: Vec<String>,
    /// Server example names.
    pub servers: Vec<String>,
    /// Client example names.
    pub clients: Vec<String>,
}

impl Examples {
    pub fn new() -> Result<Self> {
        let mut all = get_examples()?;
        all.sort_unstable();

        let clients = all
            .iter()
            .filter(|name| name.starts_with("client_"))
            .cloned()
            .collect();
        let servers = all
            .iter()
            .filter(|name| name.starts_with("server_"))
            .cloned()
            .collect();

        Ok(Examples {
            all,
            clients,
            servers,
        })
    }

    pub fn build(&self) -> Result<()> {
        for example in &self.all {
            run(
                "cargo",
                &["build", "--example", example],
                |_| (),
                |_| Ok(()),
            )?;
        }
        Ok(())
    }

    fn combinations(&self) -> Vec<[&str; 2]> {
        let mut v = Vec::with_capacity(self.servers.len() * self.clients.len());
        for server in &self.servers {
            for client in &self.clients {
                v.push([server.as_str(), client]);
            }
        }
        v
    }

    pub fn bench(&self) -> Result<Vec<Throughput>> {
        let v = self.combinations();
        let env_fns = gen_env_fns();
        let combinations = v.len();
        let benches_len = combinations * env_fns.len();
        info!(combinations, benches_len);

        let mut benches = Vec::with_capacity(benches_len);
        for [server, client] in &v {
            for &env_fn in &env_fns {
                benches.push(Bench {
                    server,
                    client,
                    env_fn,
                });
            }
        }

        // Repeat each bench.
        let repeat = 1;
        let mut repeated_benches = Vec::with_capacity(benches.len() * repeat);
        for _ in 0..repeat {
            repeated_benches.extend(benches.iter().copied());
        }
        drop(benches);

        // shuffle
        use rand::seq::SliceRandom;
        repeated_benches.shuffle(&mut rand::rng());

        let len = repeated_benches.len();
        let mut throughputs = Vec::with_capacity(len);
        for (idx, bench) in repeated_benches.iter().enumerate() {
            bench.run(&mut throughputs, idx, len)?;
        }

        // Descending sort.
        throughputs.sort_unstable_by(|a, b| b.cmp(a));
        Ok(throughputs)
    }
}

#[derive(Clone, Copy)]
struct Bench<'a> {
    server: &'a str,
    client: &'a str,
    env_fn: EnvFn,
}

impl Bench<'_> {
    fn run(self, throughputs: &mut Vec<Throughput>, idx: usize, len: usize) -> Result {
        let Self {
            server,
            client,
            env_fn,
        } = self;
        let _span = info_span!("bench", idx, len, server, client).entered();
        let (stdout, config) = run_pair(server, client, env_fn)?;
        let throughput = parse_output(&stdout, server, client, config)
            .with_context(|| format!("No throughput in:\n{stdout:?}"))?;
        info!(conn = throughput.conn, secs = throughput.secs);
        throughputs.push(throughput);
        Ok(())
    }
}

fn get_examples() -> Result<Vec<String>> {
    let mut v = Vec::new();
    for entry in std::fs::read_dir("examples")? {
        let entry = entry?;
        let path = entry.path();
        if entry.metadata()?.is_file() && path.extension().map(|ext| ext == "rs").unwrap_or(false) {
            if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                v.push(file_stem.to_owned());
            }
        }
    }
    Ok(v)
}

pub fn run<T>(
    exe: &str,
    args: &[&str],
    env: impl for<'a> FnOnce(&mut EnvConfig<'a>),
    f: impl FnOnce(String) -> Result<T>,
) -> Result<(T, Config)> {
    let cmd = || {
        std::iter::once(exe)
            .chain(args.iter().copied())
            .collect::<Vec<_>>()
            .join(" ")
    };
    let _span = error_span!("run", cmd = cmd()).entered();

    let mut cmd = Command::new(exe);
    cmd.args(args);
    let mut env_config = EnvConfig::new(&mut cmd);
    env(&mut env_config);
    let config = env_config.finish();
    let output = cmd.output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    if !output.status.success() {
        error!(stdout, stderr, "Failed to run.");
    }

    let ret = f(stdout)?;
    Ok((ret, config))
}

type EnvFn = fn(&mut EnvConfig);

fn gen_env_fns() -> Vec<fn(&mut EnvConfig)> {
    [
        |_: &mut EnvConfig| (),
        |env: &mut EnvConfig| _ = env.set_duration(4),
        |env: &mut EnvConfig| _ = env.set_socket_len(200),
        |env: &mut EnvConfig| _ = env.set_size(1024 * 1024), // 1MB
    ]
    .into_iter()
    .collect()
}

fn run_pair(server: &str, client: &str, env: EnvFn) -> Result<(String, Config)> {
    std::thread::scope(|scope| {
        let task_server = scope.spawn(|| run("cargo", &["run", "--example", server], env, Ok));
        let task_client =
            scope.spawn(|| run("cargo", &["run", "--example", client], env, |_| Ok(())));
        task_client.join().unwrap()?;
        task_server.join().unwrap()
    })
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Throughput {
    /// MB per seconds
    pub mbps: usize,
    /// Connections per second.
    pub conn: u32,
    /// Duration in seconds.
    pub secs: u32,
    /// Server example name.
    pub server: String,
    /// Client example name.
    pub client: String,
    // If single-threaded? False means multi-threaded.
    pub server_st: bool,
    pub client_st: bool,
    // Config for this bench.
    pub size: usize,
    pub socket_len: usize,
    pub duration: u32,
    pub interval: u32,
}

// Avg: 53807 (538073 / 10s)
fn parse_output(s: &str, server: &str, client: &str, config: Config) -> Option<Throughput> {
    const PAT: &str = "Avg: ";
    let last = &s[s.rfind(PAT)?..];
    let (_, conn, secs) = lazy_regex::regex_captures!(r#"Avg: (\d+) \(\d+ / (\d+)s\)"#, last)?;
    let conn = conn.parse::<usize>().ok()?;
    Some(Throughput {
        mbps: conn * (config.size / 1024) / 1024,
        conn: conn as u32,
        secs: secs.parse().ok()?,
        server: server.to_owned(),
        client: client.to_owned(),
        server_st: server.ends_with("st"),
        client_st: client.ends_with("st"),
        size: config.size,
        socket_len: config.socket_len,
        duration: config.duration.as_secs() as u32,
        interval: config.interval.as_secs() as u32,
    })
}

pub fn write_csv<S: Serialize>(path: &str, data: &[S]) -> Result {
    let _span = error_span!("write_csv", path).entered();
    info!(data.len = data.len());

    let mut writer = csv::Writer::from_path(path)?;
    for row in data {
        writer.serialize(row)?;
    }

    Ok(())
}
