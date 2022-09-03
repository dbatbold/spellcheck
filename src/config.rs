use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub datafile: Vec<Datafile>,
}

#[derive(Deserialize, Debug)]
pub struct Datafile {
    pub lang: String,
    pub url: String,
    pub file: Option<String>,
    pub kind: String,
    pub enable: bool,
}

impl Config {
    pub fn load(path: &Path) -> Result<Config, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}
