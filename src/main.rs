use std::env;

// Recherche un pattern dans un fichier et affiche les lignes correspondantes.
// Le pattern est la string que à chercher
// Le path est le chemin du fichier à chercher
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = env::args().nth(1).expect("No pattern given.");
    let path    = env::args().nth(2).expect("No path given");

    let args = Cli {
        pattern,
        path: std::path::PathBuf::from(path),
    };

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}

