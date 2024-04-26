use std::path::PathBuf;
use clap::{Parser, Subcommand};


/**
 * OUTLINE
 * mangu new <project_name> --path <path | default="./"> --template <template | default = "blank"> 
 */

#[derive(Parser)]
#[command(
    version=1.0, 
    about="Patata Engine CLI", 
    long_about = "Create and manage Patata Engine projects"
)]
struct Args {
    #[subcommand]
    cmd: Command
}

#[derive(Parser)]
enum Command { // Enumeración de comandos: mangu <cmd>
    New(New) // mangu new ...: Crea un nuevo proyecto
}

#[derive(Parser)]
struct New {
    #[clap(short, long, default_value = "./")]
    path: PathBuf,

    #[clap(short, long, default_value = "blank")]
    template: String,

    project_name: String
}


fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Command::New(new) => {
            println!("Creating new project at {:?}", new.path);
            println!("Project name: {}", new.project_name);
            println!("Template: {}", new.template);
        }
    }
}
