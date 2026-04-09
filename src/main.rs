use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use fs_extra::dir::{copy, CopyOptions};
use git2::Repository;
use std::{
    env, fs::{OpenOptions, create_dir_all, remove_dir_all}, io::{BufWriter, Write, stdin, stdout}, path::PathBuf
};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
/// Tool useful to help linux PC to develop over ssh on pynq board creating new project with utils that help to
/// sync changes with board and remotely launch there run command (python)
struct Args {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create new project with utils inside
    New {
        #[arg(short, long)]
        local: PathBuf,
        #[arg(short, long)]
        remote: PathBuf,
    },
    Sync,
    Open
}

const UTILS_DIR: &str = "/opt/pz2/.utils";

fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::New { local, remote }) => {
            // validation: If local already exists, bail out early.
            let pz2_dot_subdir = local.join(".pz2");
            if local.exists() {
                loop {
                    let s = read_tty_input("Do you want to overwrite [y/n]: ")?;
                    if s == "y" {
                        // remove current local/.utils file
                        if pz2_dot_subdir.exists() {
                            remove_dir_all(&pz2_dot_subdir)?;
                        }
                        break;
                    }
                    else if s == "n" {
                        return Err(anyhow!("Directory {:?} already exists!", local));
                    }
                };
            }

            // Create the base local directory and the '.pz2' subdir
            create_dir_all(&pz2_dot_subdir).context("Failed to create .pz2 folder")?;

            // Copy options
            let mut options = CopyOptions::new();
            options.overwrite = true;
            options.content_only = true;

            // fs_extra::dir::copy needs the destination parent to exist
            copy(UTILS_DIR, &pz2_dot_subdir, &options)
                .map_err(|e| anyhow!("Failed to copy utils: {}", e))?;


            let source_env = std::path::Path::new(UTILS_DIR).join(".env");
            let dest_env = pz2_dot_subdir.join(".env");
            if !source_env.exists() {
                return Err(anyhow!("Source .env not found in {}, re-run 'build.sh'", UTILS_DIR));
            }
            std::fs::copy(&source_env, &dest_env).context("Failed to copy .env template")?;
            let env = OpenOptions::new()
                .append(true)
                .open(&dest_env)
                .context("Failed to open local .env for appending")?;
            let mut writer = BufWriter::new(env);
            // Writing paths to .env
            writeln!(writer, "LOCAL_PROJECT_PATH={:?}", local)?;
            writeln!(writer, "REMOTE_PROJECT_PATH={:?}", remote)?;

            // add git init e .gitignore
            if let Ok(_) = Repository::open(local) {
                println!("repository is already initialized, remember to add '.pz2' within '.gitignore' file");
            } else {
                match Repository::init(&local) {
                    Ok(_) => {
                        println!("repository has correctly created");
                    },
                    Err(e) => {
                        return Err(anyhow!(e));
                    }
                };
                let gitignore = OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(local.join(".gitignore"))?;
                writer = BufWriter::new(gitignore);
                writeln!(writer, ".pz2")?;
            }


            println!("Successfully initialized project at {:?}", local);
            Ok(())
        },

        Some(Commands::Sync) => {

            match env::current_dir() {
                Ok(current_dir) => {
                    let pz2_dir = current_dir.join(".pz2");
                    let env_path = pz2_dir.join(".env");
                    let sync_path = pz2_dir.join("sync.sh");
                    if pz2_dir.exists() {
                        if !env_path.exists() || !sync_path.exists() {
                            return Err(anyhow!(".pz2 folder is broken, re-init project with 'pz2 new ...'"));
                        }
                        Command::new("bash")
                            .args([sync_path, env_path])
                            .status()?;
                    }
                    else {
                        return Err(anyhow!("this folder is not a pz2 project"));
                    }
                },
                Err(e) => {
                    return Err(anyhow!(e));
                }
            }

            Ok(())
        },
        Some(Commands::Open) => {

            match env::current_dir() {
                Ok(current_dir) => {
                    let pz2_dir = current_dir.join(".pz2");
                    let env_path = pz2_dir.join(".env");
                    let open_path = pz2_dir.join("open.sh");
                    if pz2_dir.exists() {
                        if !env_path.exists() || !open_path.exists() {
                            return Err(anyhow!(".pz2 folder is broken, re-init project with 'pz2 new ...'"));
                        }
                        Command::new("bash")
                            .args([open_path, env_path])
                            .status()?;
                    }
                    else {
                        return Err(anyhow!("this folder is not a pz2 project"));
                    }
                },
                Err(e) => {
                    return Err(anyhow!(e));
                }
            }

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