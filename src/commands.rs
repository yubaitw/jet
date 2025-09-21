use crate::generate;
use crate::server;
use crate::helper;
use chrono;
use std::fs;
use std::io;
use std::path;

const DEFAULT_ARTICLE_TEMPLATE: &str = "---\n\
     title: \"\"\n\
     date: \"{date}\"\n\
     slug: \"{slug}\"\n\
     draft: true\n\
     description: \"\"\n\
     ---";

pub fn execute_commnads(command: &str, argc: usize, args: Vec<String>) {
    match command {
        "build" => {
            if argc == 2 {
                build_site("public/".to_string());
            } else if argc == 4 {
                let option = args[2].clone();

                if option == "--output-dir" || option == "-o" {
                    let output_dir = args[3].clone();
                    build_site(output_dir);
                } else {
                    println!("Unknown option: {}", &option);
                }
            } else {
                println!("Wrong number of arguments");
            }
        }
        "serve" => {
            if argc == 2 {
                serve();
            } else {
                println!("Wrong number of arguments");
            }
        }
        "create" => {
            if argc == 3 {
                let article_slug = args[2].clone();
                let _ = create_article(article_slug);
            } else {
                println!("Wrong number of arguments");
            }
        }
        _ => {
            println!(
                "Error: command error: unknown command \"{}\" for \"jet\"",
                &command
            );
        }
    }
}

pub fn build_site(output_dir: generate::Path) {
    let articles_directory = "./articles".to_string();
    let articles = generate::get_articles(articles_directory.clone());
    let _ = generate::create_homepage_html_file(articles, output_dir.clone(), true);
    let articles = generate::get_articles(articles_directory.clone());

    for article in articles {
        if !article.draft {
            let mut output_dir_path = path::PathBuf::from(&output_dir);
            output_dir_path.push("posts/");

            match generate::create_article_html_file(
                &article,
                "templates/article.html".to_string(),
                output_dir_path.to_str().unwrap().to_string(),
            ) {
                Ok(_ok) => {}
                Err(e) => {
                    println!("{}", e);
                }
            };
        }
    }

    helper::copy_assets_to_output_dir("assets/", output_dir.as_str());

    println!("Site was generated successfully.");
}

pub fn create_article(article_slug: String) -> io::Result<()> {
    let article_content = DEFAULT_ARTICLE_TEMPLATE
        .replace("{date}", chrono::Local::now().format("%Y-%m-%d").to_string().as_str())
        .replace("{slug}", article_slug.as_str());

    fs::write(format!("articles/{}.md", article_slug), article_content)?;
    println!("Create article: articles/{}.md", article_slug.clone());
    return Ok(());
}

pub fn serve() {
    println!("Web Server is available at http://localhost:3000/ (bind address 127.0.0.1) ");
    println!("Press Ctrl+C to stop");
    server::start_server();
}
