use std::path::{Path, PathBuf};
use std::{env, fs};
use std::process::Command;
use clap::{Parser, Subcommand, Args};

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
        Cmds::New(new) => {
            new_project(&new.project_name, &new.path, &new.template);

            // Aqui se pueden crear cualquier archivo necesario para un projecto
        }
    }
}

// Crea un nuevo projecto de patata en el directorio espeficicado usando la plantilla adecuada
fn new_project(project_name: &String, path: &PathBuf, template: &String) {

    // Intenta crear y moverse al directorio del projecto, si falla -> error
    if let Err(err) = move_to_path_create_dir(&path) {
        eprintln!("Failed to move to directory: {}", err);
        return;
    }

    // Trae patata
    let output = Command::new("git")
        .arg("pull")
        .arg("https://gitlab.com/patata-engine/patata-engine.git")
        .output()
        .expect("Failed to retrieve Patata Engine from gitlab.");

    // Si falla al intentar traer patata
    if !output.status.success() {
        eprintln!("Error: {:?}", output);
        return;
    }
    
    // Trae la plantilla -- TODO: Esto no funciona :(  UnU
    //
    // let output = Command::new("git")
    //    .output()
    //    .expect(&format!("Failed to retrieve template {} from patata-engine/Mangu", template));

    // Si falla al intentar la plantilla
    // if !output.status.success() {
    //    eprintln!("Error: {:?}", output);
    //   return;
    // }

    // Log
    println!("Projecto: {project_name}/{template} creado con exito en {}.", path.display().to_string());
}

// Crea y se mueve a un directorio
fn move_to_path_create_dir(path: &Path) -> Result<(), std::io::Error> {
    // Create directory if it doesn't exist
    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    // Move to the specified directory
    env::set_current_dir(path)?;

    Ok(())
}
