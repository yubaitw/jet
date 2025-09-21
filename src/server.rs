use crate::generate;
use crate::articles::{get_articles, convert_article_to_html};
use crate::helper;
use axum::{routing::get, Router, extract::Path, response::Html};
use tower_http::services::ServeDir;

#[tokio::main]
pub async fn start_server() {
    let assets_service = ServeDir::new("assets");
    let articles_routes: Router<> = Router::new().route("/posts/{article_slug}", get(get_article));
    let app = Router::new().route("/", get(get_homepage))
        .merge(articles_routes)
        .fallback_service(assets_service);

    // Bind to an address and serve the app
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_homepage() -> Html<String> {
    let articles_directory = "./articles".to_string();
    let articles = get_articles(articles_directory);
    let homepage_template = helper::read_file_content("templates/homepage.html".to_string());
    let is_production = false;
    let homepage_html = generate::create_homepage_html(articles, homepage_template, is_production);

    Html(homepage_html)
} 

async fn get_article(Path(article_slug): Path<String>) -> Html<String> {
    let articles_directory = "./articles".to_string();
    let articles = get_articles(articles_directory);

    if let Some(article) = articles.iter().find(|a| a.slug == article_slug) {
        Html(convert_article_to_html(&helper::read_file_content("./templates/article.html".to_string()), article))
    } else {
        Html("Not Found".to_string())
    }
}
