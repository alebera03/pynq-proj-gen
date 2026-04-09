use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use fs_extra::dir::{copy, CopyOptions};
use git2::Repository;
use std::{
    fs::{OpenOptions, create_dir_all, remove_dir_all}, io::{BufWriter, Write, stdin, stdout}, path::PathBuf
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
            let scripts_subdir = local.join(".scripts");
            if local.exists() {
                loop {
                    let s = read_tty_input("Do you want to overwrite [y/n]: ")?;
                    if s == "y" {
                        // remove current local/.scripts file
                        if scripts_subdir.exists() {
                            remove_dir_all(&scripts_subdir)?;
                        }
                        break;
                    }
                    else if s == "n" {
                        return Err(anyhow!("Directory {:?} already exists!", local));
                    }
                };
            }

            // Create the base local directory and the '.scripts' subdir
            create_dir_all(&scripts_subdir).context("Failed to create .scripts folder")?;

            // Copy options
            let mut options = CopyOptions::new();
            options.overwrite = true;
            options.content_only = true;

            // fs_extra::dir::copy needs the destination parent to exist
            copy(SCRIPTS_DIR, &scripts_subdir, &options)
                .map_err(|e| anyhow!("Failed to copy scripts: {}", e))?;


            let source_env = std::path::Path::new(SCRIPTS_DIR).join(".env");
            let dest_env = scripts_subdir.join(".env");
            if !source_env.exists() {
                return Err(anyhow!("Source .env not found in {}, re-run build.sh", SCRIPTS_DIR));
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
            writeln!(writer, ".scripts")?;


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