#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate rayon;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod alphred;
mod errors;
mod giphy;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use rayon::prelude::*;
use url::Url;
use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    let query = env::args().skip(1).collect::<Vec<_>>().join(" ");
    let resp = search_giphy(&query)?;
    let gifs = resp.gifs;
    let dir = temp_dir()?;

    let icons: Vec<_> = gifs.par_iter()
        .map(|gif| {
                 let path = dir.join(format!("{}.gif", gif.id));
                 download(gif.thumbnail_url(), &path)?;
                 Ok(path)
             })
        .collect::<Result<_>>()?;

    let items: Vec<_> = gifs.iter()
        .zip(icons.iter())
        .map(|(gif, icon)| {
                 alphred::Item {
                     title: gif.slug.clone(),
                     subtitle: gif.id.clone(),
                     arg: gif.download_url().as_str().into(),
                     icon: alphred::Icon { path: icon.clone() },
                 }
             })
        .collect();

    let json = json!({
                         "items": items
                     });
    println!("{}", json);

    Ok(())
}

fn search_giphy(query: &str) -> Result<giphy::SearchResponse> {
    let mut url = reqwest::Url::parse("http://api.giphy.com/v1/gifs/search")?;
    url.query_pairs_mut().append_pair("q", &query);
    url.query_pairs_mut().append_pair("limit", "9");
    url.query_pairs_mut()
        .append_pair("api_key", "dc6zaTOxFJmzC");
    reqwest::get(url)?.json().map_err(Error::from)
}

fn temp_dir() -> Result<PathBuf> {
    let dir = env::var("alfred_workflow_cache")
        .map(|x| PathBuf::from(x.trim()))
        .unwrap_or(env::temp_dir());
    if !dir.exists() {
        fs::create_dir(&dir)?;
    }
    Ok(dir)
}

fn download(url: &Url, to: &Path) -> Result<()> {
    let mut res = reqwest::get(url.clone())?;
    let mut file = fs::File::create(to)?;
    std::io::copy(&mut res, &mut file)?;
    Ok(())
}
