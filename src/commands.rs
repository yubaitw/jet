use crate::generate;
use crate::server;
use chrono;
use std::fs;
use std::io;
use std::path;

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
    let article_filepaths: Vec<generate::Path> = match generate::get_article_filepaths("./articles")
    {
        Ok(paths) => paths,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let articles = generate::get_all_articles(article_filepaths.clone());
    let _ = generate::create_homepage_html_file(articles, output_dir.clone(), true);
    let articles = generate::get_all_articles(article_filepaths);
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

    generate::copy_assets_to_output_dir("assets/", output_dir.as_str());

    println!("Site was generated successfully.");
}

pub fn create_article(article_slug: String) -> io::Result<()> {
    let article_template = format!(
        "---\n\
         title: \"\"\n\
         date: \"{}\"\n\
         slug: \"{}\"\n\
         draft: true\n\
         description: \"\"\n\
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
