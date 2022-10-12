use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use hi_push::service;

#[tokio::main]
async fn main() {
    let mongo_uri = std::env::var("MONGO_URI").expect("no MONGO_URI env");
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = mongodb::Client::with_uri_str(mongo_uri).await.expect("da error").database("kuafu_next");

    let app = service::App::new(db).await;

    hi_push::http::start(app).await.expect("http start error")
}