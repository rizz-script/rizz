use std::path::PathBuf;
use std::time::Duration;

use clap::{Parser, Subcommand};
use globset::{Glob, GlobSet, GlobSetBuilder};
use notify::{RecursiveMode, Watcher};

#[derive(Parser)]
#[command(name = "rizz", version, about = "RizzScript interpreter (v0.1)")]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run a .rizz file
    Run {
        file: PathBuf,
        /// Arguments passed to the script (available as global ARGS)
        args: Vec<String>,
        /// Auto-restart on file changes (default: the script itself). Optionally provide a glob.
        #[arg(long, num_args = 0..=1, default_missing_value = "__SELF__")]
        watch: Option<String>,
    },
    /// Create a starter `main.rizz` in current directory
    Init,
    /// Format (and basic-lint) .rizz files
    Format {
        /// File or directory
        path: PathBuf,
        /// Check only (non-zero exit if changes needed)
        #[arg(long)]
        check: bool,
    },
    /// Print version
    Version,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let cli = Cli::parse();
            match cli.cmd {
                Command::Run { file, args, watch } => {
                    if let Some(watch) = cli_watch_spec(&file, watch)? {
                        watch_and_restart(&file, &args, watch).await?;
                    } else {
                        let src = tokio::fs::read_to_string(&file).await?;
                        let program =
                            rizz_core::parser::parse_program(&src, file.to_string_lossy().as_ref())?;
                        rizz_core::typecheck::typecheck(&program)?;
                        let mut rt =
                            rizz_core::runtime::Runtime::new(file.to_string_lossy().as_ref(), args);
                        rt.exec_program(&program).await?;
                    }
                }
                Command::Init => {
                    let path = PathBuf::from("main.rizz");
                    if path.exists() {
                        return Err(anyhow::anyhow!("main.rizz already exists"));
                    }
                    let starter = r#"const MESSAGE = "What's Up!"

function main() {
  Rizz(MESSAGE)
}

Vibe main()
"#;
                    tokio::fs::write(&path, starter).await?;
                    println!("Created {}", path.display());
                }
                Command::Format { path, check } => {
                    format_paths(&path, check)?;
                }
                Command::Version => {
                    println!("{}", env!("CARGO_PKG_VERSION"));
                }
            }
            Ok::<(), anyhow::Error>(())
        })
        .await?;

    Ok(())
}

fn cli_watch_spec(file: &PathBuf, watch: Option<String>) -> anyhow::Result<Option<Vec<String>>> {
    let Some(w) = watch else { return Ok(None) };
    if w == "__SELF__" {
        return Ok(Some(vec![file.to_string_lossy().to_string()]));
    }
    Ok(Some(vec![w]))
}

async fn watch_and_restart(file: &PathBuf, args: &[String], patterns: Vec<String>) -> anyhow::Result<()> {
    let bin = std::env::current_exe()?;
    let cwd = std::env::current_dir()?;

    let matcher = build_globset(&patterns)?;

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();
    std::thread::spawn({
        let tx = tx.clone();
        let cwd2 = cwd.clone();
        move || {
            let mut watcher = notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
                if let Ok(ev) = res {
                    for p in ev.paths {
                        let rel = p.strip_prefix(&cwd2).unwrap_or(&p);
                        if matcher.is_match(rel) {
                            let _ = tx.send(());
                            break;
                        }
                    }
                }
            }).expect("watcher");
            watcher.watch(&cwd, RecursiveMode::Recursive).expect("watch");
            loop {
                std::thread::sleep(Duration::from_secs(3600));
            }
        }
    });

    let mut child: Option<std::process::Child> = None;

    loop {
        if child.is_none() {
            let mut cmd = std::process::Command::new(&bin);
            cmd.arg("run").arg(file);
            for a in args {
                cmd.arg(a);
            }
            cmd.stdin(std::process::Stdio::inherit())
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit());
            child = Some(cmd.spawn()?);
        }

        tokio::select! {
            _ = rx.recv() => {
                if let Some(mut c) = child.take() {
                    let _ = c.kill();
                    let _ = c.wait();
                }
                // debounce a bit
                tokio::time::sleep(Duration::from_millis(150)).await;
            }
        }
    }
}

fn build_globset(patterns: &[String]) -> anyhow::Result<GlobSet> {
    let mut b = GlobSetBuilder::new();
    for p in patterns {
        b.add(Glob::new(p)?);
    }
    Ok(b.build()?)
}

fn format_paths(path: &PathBuf, check: bool) -> anyhow::Result<()> {
    let mut files: Vec<PathBuf> = Vec::new();
    if path.is_dir() {
        for e in walkdir::WalkDir::new(path) {
            let e = e?;
            if !e.file_type().is_file() {
                continue;
            }
            let p = e.path();
            if p.extension().and_then(|s| s.to_str()) == Some("rizz") {
                files.push(p.to_path_buf());
            }
        }
    } else {
        files.push(path.clone());
    }

    let mut changed = false;
    for p in files {
        let src = std::fs::read_to_string(&p)?;
        // lint: must parse
        let _ = rizz_core::parser::parse_program(&src, p.to_string_lossy().as_ref())?;

        let formatted = rizz_core::formatter::format_source(&src);
        if formatted != src {
            changed = true;
            if !check {
                std::fs::write(&p, formatted)?;
            }
        }
    }

    if check && changed {
        return Err(anyhow::anyhow!("formatting changes needed"));
    }
    Ok(())
}

