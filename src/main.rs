use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use fs_extra::dir::{copy, CopyOptions};
use std::{
    fs::{File, create_dir_all}, io::{BufWriter, Write}, option, path::PathBuf
};

#[derive(Parser, Debug)]
#[command(about = "Command to handle PC interaction with remote pynq board", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands, // Removed Option since you error out anyway if empty
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create new project with scripts inside
    New {
        #[arg(long)]
        local: PathBuf,
        #[arg(long)]
        remote: PathBuf,
    },
}

const SCRIPTS_DIR: &str = "/opt/pz2/.scripts";

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::New { local, remote } => {
            // 1. Validation: If local already exists, bail out early.
            if local.exists() {
                return Err(anyhow!("Directory {:?} already exists!", local));
            }

            // 2. Create the base local directory and the 'files' subdir
            let scripts_subdir = local.join(".scripts");
            create_dir_all(&scripts_subdir).context("Failed to create .scripts folder")?;

            // 3. Copy scripts from /opt/pz2/.scripts to local/.scripts
            let mut options = CopyOptions::new();
            options.overwrite = true;
            options.content_only = true;
        
            // fs_extra::dir::copy needs the destination parent to exist
            copy(SCRIPTS_DIR, &scripts_subdir, &options)
                .map_err(|e| anyhow!("Failed to copy scripts: {}", e))?;

            // 4. Create .env file inside the 'files' folder
            let env_path = scripts_subdir.join(".env");
            let file = File::create(&env_path).context("Failed to create .env file")?;
            let mut writer = BufWriter::new(&file);
            
            // Writing paths to .env
            writeln!(writer, "LOCAL_PROJECT_PATH={:?}", local)?;
            writeln!(writer, "REMOTE_PROJECT_PATH={:?}", remote)?;

            println!("Successfully initialized project at {:?}", local);
            Ok(())
        }
    }
}