use std::path::PathBuf;
use clap::{Parser, Subcommand, Args};

mod proyect;
use crate::proyect::new_project;

/**
 * Comandos
 * mangu new <project_name> --path <path | default="./"> --template <template | default = "blank">
 */

#[derive(Parser)]
#[command(
    version="1.0",
    about="Patata Engine CLI",
    long_about = "Create and manage Patata Engine projects"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmds
}

#[derive(Subcommand)]
enum Cmds { // Enumeración de comandos: mangu <cmd>
    #[command(about="Crea un nuevo projecto de patata.")]
    New(NewCommand) // mangu new ...: Crea un nuevo proyecto
}

#[derive(Args)]
struct NewCommand { // mangu new project_name -path <path | default = "./" --template <template_name | default =
             // "blank"
    #[arg(short, long, default_value = "./")]
    path: PathBuf,

    #[arg(short, long, default_value = "blank")]
    template: String,

    project_name: String
}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {

        // mangu new
        Cmds::New(mut new) => {
            new_project(&new.project_name, &mut new.path, &new.template);
        }
    }
}
