error_chain! {
    foreign_links {
        Http(::reqwest::Error);
        Url(::reqwest::UrlError);
        Io(::std::io::Error);
    }
}
