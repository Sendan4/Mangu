use std::io::Write;
use std::fs::File;

pub fn generar_cmake_file(project_name : &String) {
    println!("Generating the file CMakeLists.txt");

    let mut _cmake_file = File::create("CMakeLists.txt");

    println!("Writing in CMakeLists.txt");

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
