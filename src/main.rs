use std::path::PathBuf;
use clap::{Parser, Subcommand};

// Definición de la estructura CLI con atributos para la ayuda
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Nombre (opcional)
    #[arg(short, long)]
    name: Option<String>,

    /// Configuración personalizada
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Habilita la información de depuración
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

// Definición de subcomandos
#[derive(Subcommand)]
enum Commands {
    /// Realiza pruebas
    Test {
        /// Lista los valores de prueba
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // Verifica los argumentos proporcionados
    if let Some(name) = &cli.name {
        println!("Valor para nombre: {}", name);
    }

    if let Some(config_path) = &cli.config {
        println!("Valor para config: {}", config_path.display());
    }

    // Verifica el nivel de depuración
    match cli.debug {
        0 => println!("El modo de depuración está desactivado"),
        1 => println!("El modo de depuración está parcialmente activado"),
        2 => println!("El modo de depuración está completamente activado"),
        _ => println!("No te vuelvas loco"),
    }

    // Verifica si hay subcomandos
    if let Some(Commands::Test { list }) = &cli.command {
        if *list {
            println!("Imprimiendo listas de prueba...");
        } else {
            println!("No se están imprimiendo listas de prueba...");
        }
    }

}
