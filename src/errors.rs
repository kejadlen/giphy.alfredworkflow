#![allow(deprecated)]
error_chain! {
    foreign_links {
        Http(::reqwest::Error);
        Url(::url::ParseError);
        Io(::std::io::Error);
    }
}
