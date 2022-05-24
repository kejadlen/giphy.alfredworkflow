#![recursion_limit = "1024"]

extern crate alphred;
#[macro_use]
extern crate error_chain;
extern crate rayon;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate url;

mod errors;
mod giphy;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::errors::*;
use alphred::Item;
use rayon::prelude::*;
use url::Url;

quick_main!(run);

fn run() -> Result<()> {
    let query = env::args().skip(1).collect::<Vec<_>>().join(" ");
    let limit = env::var("LIMIT")
        .ok()
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or(8);
    let browser = env::var("BROWSER").ok().unwrap_or_else(|| "true".into());
    let resp = search_giphy(&query, limit)?;
    let gifs = resp.gifs;
    let dir = temp_dir()?;

    let icons: Vec<_> = gifs
        .par_iter()
        .map(|gif| {
            // Some images don't have fixed_width_small_still, so
            // just download the original image if that's the case
            let url = gif
                .thumbnail()
                .map(|t| &t.url)
                .unwrap_or_else(|| &gif.images.original.url);

            let path = dir.join(format!("{}.gif", gif.id));
            download(url, &path)?;
            Ok(path)
        })
        .collect::<Result<_>>()?;

    let mut items: Vec<_> = gifs
        .iter()
        .zip(icons.iter())
        .map(|(gif, icon)| {
            let subtitle = format!("{} ({})", gif.id, gif.download_size());
            Item::new(gif.slug.clone())
                .subtitle(&subtitle)
                .arg(gif.download_url().as_str())
                .icon(icon.as_path())
                .variables(action("gif"))
        })
        .collect();

    if browser == "true" {
        items.push(
            Item::new(format!("Search for \"{}\" in browser", query))
                .arg(&format!("https://giphy.com/search/{}", query))
                .variables(action("browser")),
        );
    }

    let json = json!({ "items": items });
    println!("{}", json);

    Ok(())
}

fn search_giphy(query: &str, limit: usize) -> Result<giphy::SearchResponse> {
    let mut url = reqwest::Url::parse("https://api.giphy.com/v1/gifs/search")?;
    for &(k, v) in &[
        ("q", query),
        ("limit", &limit.to_string()),
        ("api_key", "mHT38alQ1MfE5gM6WL4OUfhox33NbXti"),
    ] {
        url.query_pairs_mut().append_pair(k, v);
    }
    reqwest::blocking::get(url)?.json().map_err(Error::from)
}

fn temp_dir() -> Result<PathBuf> {
    let dir = env::var("alfred_workflow_cache")
        .map(|x| PathBuf::from(x.trim()))
        .unwrap_or_else(|_| env::temp_dir());
    if !dir.exists() {
        fs::create_dir(&dir)?;
    }
    Ok(dir)
}

fn download(url: &Url, to: &Path) -> Result<()> {
    let mut res = reqwest::blocking::get(url.clone())?;
    let mut file = fs::File::create(to)?;
    std::io::copy(&mut res, &mut file)?;
    Ok(())
}

fn action(value: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("action".into(), value.into());
    map
}
