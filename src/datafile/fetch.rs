use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use hyper::{body::HttpBody, Client, Uri};
use hyper_tls::HttpsConnector;

use crate::config::Config;

// Downloads datafiles or reads from 'cache' directory if the file exists.
pub fn fetch(config: &Config) -> Result<Vec<File>, String> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    // Make sure 'cache' directory exists
    match fs::metadata("cache") {
        Err(_) => {
            if let Err(err) = fs::create_dir("cache") {
                return Err(format!("Failed to create 'cache' directory: {}", err));
            }
        }
        Ok(m) => {
            if !m.is_dir() {
                return Err(String::from("'cache' must be a directory."));
            }
        }
    }

    let mut files = Vec::new();
    for df in &config.datafile {
        if !df.enable {
            continue;
        }
        let url = Uri::from_str(&df.url).unwrap();
        let path = Path::new(url.path());
        let filename = path.file_name().unwrap().to_str().unwrap();

        // Check if cache file exists
        let cache_file = format!("cache/{}", &filename);
        if let Ok(file) = OpenOptions::new().read(true).open(&cache_file) {
            println!("Reading cache from '{}'", cache_file);
            files.push(file);
            continue;
        }

        // Fetch file
        rt.block_on(async {
            match fetch_file(url, &cache_file).await {
                Ok(file) => files.push(file),
                Err(err) => eprintln!("{}", err),
            }
        });
    }

    Ok(files)
}

// Downloads a datafile and saved to cache directory.
async fn fetch_file(url: Uri, cache_file: &str) -> Result<File, String> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    println!("Fetching {}", &url);
    let mut res = match client.get(url).await {
        Err(err) => return Err(format!("Failed to fetch: {}", err)),
        Ok(r) => r,
    };
    if res.status() != 200 {
        return Err(format!("Request failed: {}", res.status()));
    }

    // Store datafile to 'cache' directory
    let mut file = match OpenOptions::new().create(true).write(true).open(cache_file) {
        Err(err) => return Err(format!("Failed to open '{}': {}", cache_file, err)),
        Ok(f) => f,
    };
    while let Some(next) = res.data().await {
        let chunk = next.unwrap();
        file.write_all(&chunk).unwrap();
    }
    file.sync_all().unwrap();

    Ok(file)
}

//"url": "https://gtsvn.uit.no/biggies/trunk/langs/mn/corp/mnwiki.txt",
