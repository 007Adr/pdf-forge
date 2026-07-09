use clap::{Parser, Subcommand};
use std::path::PathBuf;
use lopdf::Document; // <-- On importe la librairie PDF

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
    Compress {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long, default_value_t = 5)]
        level: u8,
    },
    Sanitize {
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
            println!("(Fonction de compression en cours de développement...)");
        }
        Commands::Sanitize { input } => {
            println!("Analyse de sécurité et nettoyage du fichier : {:?}", input);
            // On appelle notre nouvelle fonction ici
            match sanitize_pdf(input) {
                Ok(_) => println!("✅ Nettoyage terminé avec succès !"),
                Err(e) => eprintln!("❌ Erreur lors du nettoyage : {}", e),
            }
        }
    }
}

/// Fonction principale du moteur de nettoyage (Core)
fn sanitize_pdf(input_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("-> Ouverture du PDF en mémoire binaire...");
    // 1. On charge l'arbre d'objets du PDF en mémoire
    let mut doc = Document::load(input_path)?;

    // On récupère le nombre de pages pour prouver qu'on a bien lu le fichier
    let pages = doc.get_pages();
    println!("-> PDF chargé. Il contient {} pages.", pages.len());

    // 2. Nettoyage : On attaque le dictionnaire "Info" du PDF
    // Ce dictionnaire contient souvent l'auteur, le logiciel créateur, la date de création...
    let mut metadata_found = false;
    
    // On cherche la clé "Info" dans la structure principale (trailer) du PDF
    if doc.trailer.has(b"Info") {
        metadata_found = true;
        println!("-> Métadonnées détectées. Purge en cours...");
        // On remplace le dictionnaire d'informations par un objet Null (vide)
        doc.trailer.remove(b"Info");
    }

    if !metadata_found {
        println!("-> Aucune métadonnée classique détectée.");
    }

    // 3. Sauvegarde du nouveau fichier
    let mut out_path = input_path.clone();
    // On ajoute "_sanitized" au nom du fichier
    out_path.set_file_name(format!("{}_sanitized.pdf", out_path.file_stem().unwrap().to_string_lossy()));
    
    println!("-> Reconstruction et sauvegarde vers : {:?}", out_path);
    // doc.save() va recalculer toute la table xref (les positions en octets) automatiquement !
    doc.save(out_path)?;

    Ok(())
}