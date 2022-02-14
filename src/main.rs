use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Url(String);

impl Url {
    fn new(value: String) -> Self {
        Self(format!("http://localhost:3000{}", value))
    }
}
#[derive(Serialize)]
struct UrlDescription {
    url: Url,
    method: String,
    description: String,
    payload: Option<String>,
}

async fn index() -> Json<Vec<UrlDescription>> {
    let data: Vec<UrlDescription> = vec![
        UrlDescription {
            url: Url::new("/".to_string()),
            method: "GET".to_string(),
            description: "See Documentation".to_string(),
            payload: None,
        },
        UrlDescription {
            url: Url::new("/status".to_string()),
            method: "GET".to_string(),
            description: "See the Status of the Blockchain".to_string(),
            payload: None,
        },
        UrlDescription {
            url: Url::new("/blocks".to_string()),
            method: "GET".to_string(),
            description: "See All Blocks".to_string(),
            payload: None,
        },
        UrlDescription {
            url: Url::new("/blocks".to_string()),
            method: "POST".to_string(),
            description: "Add A Block".to_string(),
            payload: Some("data:string".to_string()),
        },
        UrlDescription {
            url: Url::new("/blocks/{hash}".to_string()),
            method: "GET".to_string(),
            description: "See A Block".to_string(),
            payload: None,
        },
        UrlDescription {
            url: Url::new("/balance/{address}".to_string()),
            method: "GET".to_string(),
            description: "Get TxOuts for an Address".to_string(),
            payload: None,
        },
        UrlDescription {
            url: Url::new("/ws".to_string()),
            method: "GET".to_string(),
            description: "Upgrade to WebSockets".to_string(),
            payload: None,
        },
    ];
    Json(data)
}

async fn status<'a>() -> &'a str {
    "This is status"
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .route("/status", get(status));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
