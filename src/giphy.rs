use serde::Deserialize;
use url::Url;
use url_serde;

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "data")] pub gifs: Vec<Gif>,
}

#[derive(Debug, Deserialize)]
pub struct Gif {
    pub id: String,
    pub slug: String,
    #[serde(with = "url_serde")] pub url: Url,
    images: Images,
}

impl Gif {
    pub fn download_size(&self) -> usize {
        self.images.original.size
    }

    pub fn download_url(&self) -> &Url {
        &self.images.original.url
    }

    pub fn thumbnail_url(&self) -> &Url {
        &self.images.thumbnail.url
    }
}

#[derive(Debug, Deserialize)]
pub struct Images {
    pub original: OriginalImage,
    #[serde(rename = "fixed_width_small_still")] pub thumbnail: ThumbnailImage,
}

#[derive(Debug, Deserialize)]
pub struct OriginalImage {
    #[serde(deserialize_with = "size_from_string")] pub size: usize,
    #[serde(with = "url_serde")] pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct ThumbnailImage {
    #[serde(with = "url_serde")] pub url: Url,
}

fn size_from_string<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<usize>().map_err(::serde::de::Error::custom)
}
