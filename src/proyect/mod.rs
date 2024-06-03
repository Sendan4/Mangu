use std::process::{Command, Stdio};
use std::path::PathBuf;

mod cmake;
use crate::proyect::cmake::generar_cmake_file;

mod directories;
use crate::proyect::directories::move_to_path_create_dir;

mod source_file;
use crate::proyect::source_file::generar_source_file;

// Crea un nuevo projecto de patata en el directorio espeficicado usando la plantilla adecuada
pub fn new_project(project_name: &String, path: &mut PathBuf, template: &String) {
    println!("Generating a new project");

    // Intenta crear y moverse al directorio del projecto, si falla -> error
    path.push(project_name);
    if let Err(err) = move_to_path_create_dir(&path) {
        eprintln!("Failed to move to directory: {}", err);
        return;
    }

    // Trae patata
    println!("Downloading the Patata Engine code");

    let download_patata = Command::new("git")
        .arg("clone")
        .arg("--recurse-submodules")
        .arg("https://gitlab.com/patata-engine/patata-engine.git")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to retrieve Patata Engine from gitlab.");

    // Si falla al intentar traer patata
    if !download_patata.status.success() {
        eprintln!("Error: {:?}", download_patata);
        return;
    }


    // La logica de la platilla debe ir aqui
    generar_cmake_file(&project_name);

    generar_source_file(&template);
    // fin de la plantilla

    // Log
    println!("Project: {project_name}");
    println!("Template: {template}");
    println!("Path: {}.", path.display().to_string());
    println!("The project has been successfully created");
}
