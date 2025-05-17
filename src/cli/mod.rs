use bench_uring::Result;
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
        let mut all = examples()?;
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
            run("cargo", &["build", "--example", example], |_| Ok(()))?;
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
        dbg!(&self, v.len(), &v);

        let mut throughputs = Vec::with_capacity(v.len());
        for [server, client] in &v {
            println!("{server} - {client} : start");
            let stdout = run_pair(server, client)?;
            let throughput = parse_output(&stdout, server, client)
                .ok_or_else(|| format!("No throughput in:\n{stdout:?}"))?;
            println!("{server} - {client} : {throughput:?}");
            throughputs.push(throughput);
        }

        throughputs.sort_unstable();
        Ok(throughputs)
    }
}

fn examples() -> Result<Vec<String>> {
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

pub fn run<T>(exe: &str, args: &[&str], f: impl FnOnce(String) -> Result<T>) -> Result<T> {
    println!("{exe:?} {args:?}");

    let output = Command::new(exe).args(args).output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    if !output.status.success() {
        eprintln!("Failed to run {exe:?} {args:?}:\nstdout={stdout}\nstderr={stderr}");
    }

    f(stdout)
}

fn run_pair(server: &str, client: &str) -> Result<String> {
    std::thread::scope(|scope| {
        let task_server = scope.spawn(|| run("cargo", &["run", "--example", server], Ok));
        let task_client = scope.spawn(|| run("cargo", &["run", "--example", client], |_| Ok(())));
        task_client.join().unwrap()?;
        task_server.join().unwrap()
    })
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Throughput {
    /// Connections per second.
    pub conn: u32,
    /// Duration in seconds.
    pub secs: u32,
    /// Server example name.
    pub server: String,
    /// Client example name.
    pub client: String,
}

// Avg: 53807 (538073 / 10s)
fn parse_output(s: &str, server: &str, client: &str) -> Option<Throughput> {
    const PAT: &str = "Avg: ";
    let last = &s[s.rfind(PAT)?..];
    let (_, conn, secs) = lazy_regex::regex_captures!(r#"Avg: (\d+) \(\d+ / (\d+)s\)"#, last)?;
    Some(Throughput {
        conn: conn.parse().ok()?,
        secs: secs.parse().ok()?,
        server: server.to_owned(),
        client: client.to_owned(),
    })
}
