use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// PDF-Forge : Outil d'optimisation et de nettoyage de fichiers PDF
#[derive(Parser)]
#[command(name = "pdf-forge")]
#[command(about = "Compresse et nettoie des PDF localement et de manière sécurisée", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compresse un fichier PDF pour réduire sa taille
    Compress {
        /// Le chemin vers le fichier PDF source
        #[arg(short, long)]
        input: PathBuf,

        /// Le chemin vers le fichier de destination (optionnel)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Niveau de compression (ex: 1 pour rapide, 9 pour maximum)
        #[arg(short, long, default_value_t = 5)]
        level: u8,
    },
    /// Nettoie un PDF en supprimant ses métadonnées et scripts cachés
    Sanitize {
        /// Le chemin vers le fichier PDF à nettoyer
        #[arg(short, long)]
        input: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Compress { input, output, level } => {
            println!("Début de la compression...");
            println!("Fichier source : {:?}", input);
            println!("Niveau de compression demandé : {}", level);
            
            let out_path = output.clone().unwrap_or_else(|| {
                let mut path = input.clone();
                path.set_file_name(format!("{}_compressed.pdf", path.file_stem().unwrap().to_string_lossy()));
                path
            });
            println!("Fichier de destination : {:?}", out_path);
        }
        Commands::Sanitize { input } => {
            println!("Analyse de sécurité et nettoyage du fichier : {:?}", input);
        }
    }
}