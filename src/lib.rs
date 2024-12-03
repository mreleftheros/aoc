use dotenvy::dotenv;
use reqwest::{blocking, cookie::Jar, Url};
use std::sync::Arc;

pub fn get_from_url(url: &str) -> String {
    dotenv().unwrap();
    let url = url.parse::<Url>().unwrap();
    let _ga_var = std::env::var("_GA").unwrap();
    let _ga_var = format!("_ga={_ga_var}");
    let _ga_mhsnpjkwc7_var = std::env::var("_GA_MHSNPJKWC7").unwrap();
    let _ga_mhsnpjkwc7_var = format!("_ga_MHSNPJKWC7={_ga_mhsnpjkwc7_var}");
    let _gid_var = std::env::var("_GID").unwrap();
    let _gid_var = format!("_gid={_gid_var}");
    let session_var = std::env::var("SESSION").unwrap();
    let session_var = format!("session={session_var}");

    let jar = Arc::new(Jar::default());
    jar.add_cookie_str(_ga_var.as_str(), &url);
    jar.add_cookie_str(_ga_mhsnpjkwc7_var.as_str(), &url);
    jar.add_cookie_str(_gid_var.as_str(), &url);
    jar.add_cookie_str(session_var.as_str(), &url);

    let client = blocking::Client::builder()
        .cookie_provider(jar)
        .build()
        .unwrap();

    let res = client.get(url).send().unwrap().text().unwrap();
    res
}
