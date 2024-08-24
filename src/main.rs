use std::path;
use std::fs;
use clap::Parser;

// Recherche un pattern dans un fichier et affiche les lignes correspondantes.
// Le pattern est la string que à chercher
// Le path est le chemin du fichier à chercher
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    let content = fs::read_to_string(&args.path).expect("Could not read file.");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
