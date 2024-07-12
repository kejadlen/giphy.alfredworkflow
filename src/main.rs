#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

use alphred::Item;
use error_chain::quick_main;
use errors::*;
use rayon::prelude::*;
use reqwest::IntoUrl;
use serde_json::json;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod errors;
mod giphy;

quick_main!(run);

fn run() -> Result<()> {
    let query = env::args().skip(1).collect::<Vec<_>>().join(" ");
    let limit = env::var("LIMIT")
        .ok()
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or(9);
    let resp = search_giphy(&query, limit)?;
    let gifs = resp.gifs;
    let dir = temp_dir()?;

    let icons: Vec<_> = gifs
        .par_iter()
        .map(|gif| {
            let path = dir.join(format!("{}.gif", gif.id));
            download(gif.thumbnail_url().to_string(), &path)?;
            Ok(path)
        })
        .collect::<Result<_>>()?;

    let items: Vec<_> = gifs
        .iter()
        .zip(icons.iter())
        .map(|(gif, icon)| {
            let subtitle = format!("{} ({})", gif.id, gif.download_size());
            Item::new(gif.slug.clone())
                .subtitle(&subtitle)
                .arg(gif.download_url().as_str())
                .icon(icon.as_path())
        })
        .collect();

    let json = json!({ "items": items });
    println!("{}", json);

    Ok(())
}

fn search_giphy(query: &str, limit: usize) -> Result<giphy::SearchResponse> {
    let url = reqwest::Url::parse_with_params(
        "https://api.giphy.com/v1/gifs/search",
        &[
            ("q", query),
            ("limit", &limit.to_string()),
            ("api_key", "mHT38alQ1MfE5gM6WL4OUfhox33NbXti"),
        ],
    )
    .chain_err(|| "couldn't construct url")?;
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

fn download<T: IntoUrl>(url: T, to: &Path) -> Result<()> {
    let mut res = reqwest::blocking::get(url)?;
    let mut file = fs::File::create(to)?;
    std::io::copy(&mut res, &mut file)?;
    Ok(())
}
