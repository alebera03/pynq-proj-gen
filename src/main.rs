use clap::{Parser, Subcommand};
use anyhow::{Result, anyhow};
use std::{fs::{DirBuilder, File}, io::{BufWriter, Write}, path::PathBuf};
use fs_extra::dir::{copy, CopyOptions};

/// command to handle PC interaction with remote pynq board
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// create new project with scripts inside
    New {
        #[arg(long)]
        local: PathBuf,
        #[arg(long)]
        remote: PathBuf
    }
}

/*
/opt
    ...
    /pz2
        pz2     # executable
        files   # scripts to copy within new project 
    ...
*/


const FILES_DIR: &str = "/opt/pz2/files";

fn main() -> Result<()>{

    let args = Args::parse();

    match &args.command {

        Some(Commands::New { local, remote }) => {

            let mut dirbuilder = DirBuilder::new();
            let LOCAL_DIR: String;
            let REMOTE_DIR: String;
            dirbuilder.recursive(true);

            // check str
            match local.to_str() {
                Some(l) => {
                    LOCAL_DIR = String::from(l);
                },
                None => {
                    return Err(anyhow!("given string is not a valid path"));
                }
            }
            match remote.to_str() {
                Some(r) => {
                    REMOTE_DIR = String::from(r);
                },
                None => {
                    return Err(anyhow!("given string is not a valid path"));
                }
            }

            // if local already exists raise an error
            if local.is_dir(){
                return Err(anyhow!("choose another directory, {:?} already exists", local));
            };

            // create local folder path
            dirbuilder.create(local.as_path())?;

            // copy into local folder scripts and .env
            // configure options
            let mut options = CopyOptions::new();
            options.overwrite = true;   // overwrite if exists
            options.copy_inside = true; // copy all contentes (not DIR) to destinazione
            copy(FILES_DIR, REMOTE_DIR.clone() + "/.scripts", &options)?;

            // create .env file inside .../files
            let file = File::create_new(LOCAL_DIR.clone()+"/files/.env")?;
            let mut writer = BufWriter::new(&file);
            writeln!(writer, "LOCAL_PROJECT_PATH=\"{}\"", &LOCAL_DIR)?;
            writeln!(writer, "REMOTE_PROJECT_PATH=\"{}\"", &REMOTE_DIR)?;

            Ok(())

        },
        _ => {
            return Err(anyhow!("no subcommand inserted"));
        }
    }

}