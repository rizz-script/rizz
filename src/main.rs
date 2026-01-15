use std::path::PathBuf;

use clap::{Parser, Subcommand};

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
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let cli = Cli::parse();
            match cli.cmd {
                Command::Run { file, args } => {
                    let src = tokio::fs::read_to_string(&file).await?;
                    let program =
                        rizzscript::parser::parse_program(&src, file.to_string_lossy().as_ref())?;
                    let mut rt =
                        rizzscript::runtime::Runtime::new(file.to_string_lossy().as_ref(), args);
                    rt.exec_program(&program).await?;
                }
            }
            Ok::<(), anyhow::Error>(())
        })
        .await?;

    Ok(())
}

