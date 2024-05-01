use reqwest::blocking::ClientBuilder;

pub mod auth;
pub mod families;
pub mod users;

pub static APP_HOST: &'static str = "http://127.0.0.1:8080/api/v1";

pub fn setup_unauthorized_client() -> reqwest::blocking::Client {
    use reqwest::header;

    let mut headers = header::HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str("application/json").unwrap(),
    );

    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    return client;
}

pub fn setup_authorized_client(token: &String) -> reqwest::blocking::Client {
    use reqwest::header;

    let mut headers = header::HeaderMap::new();

    eprintln!("{:?}", token);

    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str("application/json").unwrap(),
    );

    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );

    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    return client;
}
