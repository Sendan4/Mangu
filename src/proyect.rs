use std::io::Write;
use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};
use std::{env, fs, io};
use std::fs::File;

// Crea un nuevo projecto de patata en el directorio espeficicado usando la plantilla adecuada
pub fn new_project(project_name: &String, path: &mut PathBuf, template: &String) {

    // Intenta crear y moverse al directorio del projecto, si falla -> error
    path.push(project_name);
    if let Err(err) = move_to_path_create_dir(&path) {
        eprintln!("Failed to move to directory: {}", err);
        return;
    }

    // Trae patata
    println!("Downloading the Patata Engine code :");

    let download_patata = Command::new("git")
        .arg("clone")
        .arg("--recurse-submodules")
        .arg("https://gitlab.com/patata-engine/patata-engine.git")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to retrieve Patata Engine from gitlab.");

    // Si falla al intentar traer patata
    if !download_patata.status.success() {
        eprintln!("Error: {:?}", download_patata);
        return;
    }
    else {
        io::stdout().write_all(&download_patata.stdout).unwrap();
    }

    generar_cmake_file(&project_name);

    generar_source_file();

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
    println!("Projecto: {project_name}, con la plantilla {template} creado con exito en {}.", path.display().to_string());
}

// Crea y se mueve a un directorio
fn move_to_path_create_dir(path: &Path) -> Result<(), std::io::Error> {
    // Create directory if it doesn't exist

    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    println!("Current Path : {:?}", path);

    // Move to the specified directory
    env::set_current_dir(path)?;

    if !path.exists() {
        fs::create_dir_all("src")?;
    }

    Ok(())
}


fn generar_cmake_file(project_name : &String) {
    println!("Generando el archivo CMakeLists.txt");

    let mut _cmake_file = File::create("CMakeLists.txt");

    println!("Escribiendo en CMakeLists.txt");

    _cmake_file.as_ref()
        .expect("No se pudo escribir en CMakeLists.txt")
        .write_all(b"cmake_minimum_required(VERSION 3.18)
project(\"").unwrap();

    _cmake_file.as_ref()
        .expect("No se pudo escribir en CMakeLists.txt")
        .write_all(project_name.as_bytes()).unwrap();

    _cmake_file.as_ref()
        .expect("No se pudo escribir en CMakeLists.txt")
        .write_all(b"\" LANGUAGES CXX)

add_subdirectory(patata-engine)

set(CMAKE_LIBRARY_OUTPUT_DIRECTORY \"${CMAKE_SOURCE_DIR}/bin\")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY \"${CMAKE_SOURCE_DIR}/bin\")

add_executable(${PROJECT_NAME} \"src/Main.cpp\")

target_include_directories(${PROJECT_NAME} PRIVATE
${CMAKE_CURRENT_SOURCE_DIR}/patata-engine/include)

target_link_libraries(${PROJECT_NAME} PRIVATE PatataEngine)").unwrap();
}

fn generar_source_file() {
    println!("Generando el archivo Main.cpp");

    let mut _cmake_file = File::create("src/Main.cpp");

    _cmake_file.as_ref()
        .expect("No se pudo escribir en Main.cpp")
        .write_all(b"#define SDL_MAIN_HANDLED
#include <SDL.h>
#include <PatataEngine/PatataEngine.hpp>

int main(int argv, char ** args) {
    Patata::Engine Patata(\"\", 1280, 720);

    while(true) {
        SDL_Event event;
        while(SDL_PollEvent(&event)) {
            // Patata Events
            Patata.HandleEvent(event);
            // Your Events
            if (event.type == SDL_QUIT) {
                goto FINISHLOOP;
            }
            }

            // Your Render Functions

            Patata.Render();
    }
    FINISHLOOP:

    return 0;
}").unwrap();
}
