use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "data")]
    pub gifs: Vec<Gif>,
}

#[derive(Debug, Deserialize)]
pub struct Gif {
    pub id: String,
    pub slug: String,
    // pub url: Url,
    pub images: Images,
}

impl Gif {
    pub fn download_size(&self) -> String {
        let (size, suffix) = match self.images.original.size as f64 {
            size if (0.0..=1_000.0).contains(&size) => (size, ""),
            size if (0.0..=1_000.0).contains(&size) => (size, "KB"),
            size => (size / 1_000_000.0, "MB"),
        };
        format!("{:.1}{}", size, suffix)
    }

    pub fn download_url(&self) -> &Url {
        &self.images.original.url
    }

    pub fn thumbnail(&self) -> Option<&ThumbnailImage> {
        match &self.images.thumbnail {
            Image::Empty {} => None,
            Image::Image(thumbnail) => Some(thumbnail),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Images {
    pub original: OriginalImage,
    #[serde(rename = "fixed_width_small_still")]
    pub thumbnail: Image<ThumbnailImage>,
}

#[derive(Debug, Deserialize)]
pub struct OriginalImage {
    #[serde(deserialize_with = "size_from_string")]
    pub size: usize,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct ThumbnailImage {
    pub url: Url,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Image<T> {
    Image(T),
    Empty {},
}

fn size_from_string<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<usize>().map_err(::serde::de::Error::custom)
}
