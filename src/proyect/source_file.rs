use std::fs::File;
use std::io::Write;

pub fn generar_source_file(template: &String) {
    println!("Generating the file Main.cpp");

    match template.as_str() {
        "blank" => {
            println!("Writing in Main.cpp");

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
        },
        &_ => {
            println!("This template does not exist. Instead a Main.cpp with the blank template will be created.");

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
    }
}
