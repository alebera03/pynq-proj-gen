use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use fs_extra::dir::{copy, CopyOptions};
use std::{
    fs::{File, create_dir_all}, io::{BufWriter, stdin, stdout, Write}, path::PathBuf
};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
/// Tool useful to help linux PC to develop over ssh on pynq board creating new project with scripts that help to
/// sync changes with board and remotely launch there run command (python)
struct Args {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create new project with scripts inside
    New {
        #[arg(short, long)]
        local: PathBuf,
        #[arg(short, long)]
        remote: PathBuf,
    },
}

const SCRIPTS_DIR: &str = "/opt/pz2/.scripts";

fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::New { local, remote }) => {
            // validation: If local already exists, bail out early.
            if local.exists() {
                loop {
                    let s = read_tty_input("Do you want to overwrite [y/n]: ")?;
                    if s == "y" {
                        break;
                    }
                    else if s == "n" {
                        return Err(anyhow!("Directory {:?} already exists!", local));
                    };
                };
            }

            // Create the base local directory and the 'files' subdir
            let scripts_subdir = local.join(".scripts");
            create_dir_all(&scripts_subdir).context("Failed to create .scripts folder")?;

            // Copy scripts from /opt/pz2/.scripts to local/.scripts
            let mut options = CopyOptions::new();
            options.overwrite = true;
            options.content_only = true;

            // fs_extra::dir::copy needs the destination parent to exist
            copy(SCRIPTS_DIR, &scripts_subdir, &options)
                .map_err(|e| anyhow!("Failed to copy scripts: {}", e))?;

            // Create .env file inside the 'files' folder
            let env_path = scripts_subdir.join(".env");
            let file = File::create(&env_path).context("Failed to create .env file")?;
            let mut writer = BufWriter::new(&file);
            
            // Writing paths to .env
            writeln!(writer, "LOCAL_PROJECT_PATH={:?}", local)?;
            writeln!(writer, "REMOTE_PROJECT_PATH={:?}", remote)?;

            println!("Successfully initialized project at {:?}", local);
            Ok(())
        },
        None => {Ok(())}
    }
}


fn read_tty_input(msg: &str) -> Result<String> {
    let mut s=String::new();
    print!("{}", msg);
    let _=stdout().flush();
    stdin().read_line(&mut s)?;
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    };
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    };
    Ok(s)
}