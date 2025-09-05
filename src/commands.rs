use crate::generate;
use crate::server;
use chrono;
use std::fs;
use std::io;

pub fn build_site() {
    let article_filepaths: Vec<generate::Path> = match generate::get_article_filepaths("./articles") {
        Ok(paths) => paths,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let articles = generate::get_all_articles(article_filepaths.clone());
    let _ = generate::create_homepage_html_file(articles, "public/".to_string(), true);
    let articles = generate::get_all_articles(article_filepaths);
    for article in articles {
        if !article.draft {
            match generate::create_article_html_file(
                &article,
                "templates/article.html".to_string(),
                "public/posts/".to_string(),
            ) {
                Ok(_ok) => {}
                Err(e) => {
                    println!("{}", e);
                }
            };
        }
    }

    generate::copy_assets_to_output_dir("assets/", "public/");

    println!("Site was generated successfully.");
}

pub fn create_article(article_slug: String) -> io::Result<()> {
    let article_template = format!(
        "---\n \
         title: \"\"\n \
         date: \"{}\"\n \
         slug: \"{}\"\n \
         draft: true\n \
         description: \"\"\n \
         ---",
        chrono::Local::now().format("%Y-%m-%d"),
        article_slug
    );
    fs::write(format!("articles/{}.md", article_slug), article_template)?;
    println!("Create article: articles/{}.md", article_slug.clone());
    Ok(())
}

pub fn serve() {
    println!("Web Server is available at http://localhost:3000/ (bind address 127.0.0.1) ");
    println!("Press Ctrl+C to stop");
    server::start_server();
}
