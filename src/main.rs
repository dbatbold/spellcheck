mod config;
mod datafile;
mod spell;

use std::io::Read;
use std::path::Path;

use crate::config::Config;
use crate::spell::Tree;

fn main() {
    // Load configuration
    let config_path = Path::new("config.json");
    let config = Config::load(&config_path).unwrap();

    // Open data files
    let files = match datafile::fetch(&config) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("fetch: {}", err);
            std::process::exit(1);
        }
    };

    // Load data files into Tree
    let mut tree = Tree::new();
    for mut file in files {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        tree.parse(contents.chars().collect());
        println!("Loaded {} bytes", contents.len())
    }
    println!("Count 'байна': {}", tree.count("байна"));

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    println!("{}", line);
}
