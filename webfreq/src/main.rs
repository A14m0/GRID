#![feature(format_args_nl)]



use std::collections::HashMap;
use std::future::Future;
use std::hash::Hash;
use std::io::{Write, BufRead, read_to_string};
use std::path::Path;

use regex::Regex;


#[macro_use]
mod log;


/// Checks if a path `path` exists
pub fn path_exists(
	path: impl AsRef<Path>
) -> bool {
    match std::fs::metadata(path) {
		Ok(_) => return true,
		Err(_) => return false
	}
    
}


async fn fetch_tags(url: &str) -> Result<HashMap<String, usize>, String> {

    // ignore everything that already has a file
    if path_exists(format!("./corpus/{}_body.html", &url[7..])) {
        return Err(format!("file exists in corpus"))
    }


    let mut tags: HashMap<String, usize> = HashMap::new();



    info!("Fetching {}", url);

    // todo: add timeout
    let client = reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(10))
            .build().unwrap();

    let req = reqwest::Request::new(
        reqwest::Method::GET, 
        url.try_into().unwrap()
    );

    let r = match client.execute(req).await {
        Ok(a) => {
            a.text().await.unwrap()
        },
        Err(e) => return Err(format!("{}", e))
    };

    // save the body for use later
    let mut f = match std::fs::File::create(format!("./corpus/{}_body.html", &url[7..])) {
        Ok(a) => a,
        Err(e) => return Err(format!("Failed to create file './corpus/{}_body.html': {}", &url[7..], e))
    };
    writeln!(f, "{}", r).unwrap();


    let re = Regex::new(r"<[a-z,A-Z,0-9]+([ ]|[>])").unwrap();

    for cap in re.captures_iter(&r[..]) {
        let text1 = cap.get(0).map_or("", |m| m.as_str());
        let output = String::from(&text1[1..text1.len()-1]);
        *tags.entry(output).or_insert_with(|| 0) += 1;
    }

    Ok(tags)
}

fn read_tags_from_file(path: impl AsRef<Path>) -> Result<HashMap<String, usize>, String> {
    let f = std::fs::File::open(path).unwrap();
    let r = read_to_string(f).unwrap();

    let mut tags: HashMap<String, usize> = HashMap::new();

    let re = Regex::new(r"<[a-z,A-Z,0-9]+([ ]|[>])").unwrap();

    for cap in re.captures_iter(&r[..]) {
        let text1 = cap.get(0).map_or("", |m| m.as_str());
        let output = String::from(&text1[1..text1.len()-1]);
        *tags.entry(output).or_insert_with(|| 0) += 1;
    }

    Ok(tags)
}


#[tokio::main]
async fn main() {

    let mut global_tags: HashMap<String, usize> = HashMap::new();

    // build the corpus of URLs
    // we can do that through misc google searches

    // first see if the corpus exists
    if !path_exists("./corpus") {
        info!("No corpus detected, generating...");
        std::fs::create_dir("./corpus").unwrap();
    } else {
        info!("Detected corpus");
    }


    // create corpus
    let f = std::fs::File::open("url_list.txt").unwrap();
    let reader = std::io::BufReader::new(f);

    info!("Beginning URL fetching...");
    
    // fetch each line's url
    for url in reader.lines() {
        match url {
            Ok(b) => {
                match fetch_tags(&b).await {
                    // save the tags
                    Ok(mut a) => global_tags.extend(a.drain()),
                    Err(e) => warn!("Failed to fetch {}: {}", b, e)
                }
            },
            Err(e) => warn!("Failed to read url: {}", e)
        }
    }
    
    
    info!("Reading saved data...");
    // save frequencies to the list
    for f in std::fs::read_dir("./corpus").unwrap() {
        match f {
            Ok(fpath) => {
                match read_tags_from_file(fpath.path()) {
                    Ok(mut a) => global_tags.extend(a.drain()),
                    Err(e) => warn!("Failed to handle file: {e}")
                }
            }, 
            Err(e) => warn!("Failed to parse path: {e}")
        }
            
    }
    

    let mut f = std::fs::File::create("tags_freq.txt").unwrap();
    writeln!(f, "Key,Key Frequency").unwrap();
    for key in global_tags.keys() {
        writeln!(f, "{},{}", key, global_tags[key]).unwrap();
    }
        


    info!("Collection complete. Stored in `tags_freq.txt`");
}
