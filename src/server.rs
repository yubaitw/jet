use crate::generate::{self, convert_article_to_html, read_file_content};
use axum::{routing::get, Router, extract::Path, response::Html};
use tower_http::services::ServeDir;

#[tokio::main]
pub async fn start_server() {
    let assets_service = ServeDir::new("assets");
    let articles_routes: Router<> = Router::new().route("/posts/{article_slug}", get(get_article));
    let app = Router::new().route("/", get(get_homepage)).merge(articles_routes).fallback_service(assets_service);

    // Bind to an address and serve the app
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_homepage() -> Html<String> {
    let article_filepaths: Vec<generate::Path> = match generate::get_article_filepaths("./articles") {
        Ok(paths) => paths,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let articles = generate::get_all_articles(article_filepaths.clone());
    let homepage_template = generate::read_file_content("templates/homepage.html".to_string());
    let is_production = false;
    let homepage_html = generate::create_homepage_html(articles, homepage_template, is_production);

    Html(homepage_html)
}

async fn get_article(Path(article_slug): Path<String>) -> Html<String> {
     // Build the router with a single route
    let article_filepaths: Vec<generate::Path> = match generate::get_article_filepaths("./articles") {
        Ok(paths) => paths,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let articles = generate::get_all_articles(article_filepaths.clone());

    if let Some(article) = articles.iter().find(|a| a.slug == article_slug) {
        Html(convert_article_to_html(&read_file_content("./templates/article.html".to_string()), article))
    } else {
        Html("Not Found".to_string())
    }
}
