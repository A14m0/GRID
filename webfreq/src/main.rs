#![feature(format_args_nl)]



use std::collections::HashMap;
use std::io::BufRead;
use regex::Regex;




#[macro_use]
mod log;


async fn fetch_tags(url: &str) -> Result<HashMap<String, usize>, String> {
    let mut tags: HashMap<String, usize> = HashMap::new();

    let r = match reqwest::get(url).await{
        Ok(a) => {
            a.text()
            .await.unwrap()
        },
        Err(e) => return Err(format!("{}", e))
    };



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

    
    println!("{:?}", global_tags);


    //println!("{}", r);
}
