use chrono;
use chrono::Datelike;
use markdown;
use markdown::CompileOptions;
use markdown_frontmatter;
use minijinja::{context, Environment};
use serde;
use std::collections::HashMap;
use std::fs;
use std::fs::create_dir;
use std::io;
use std::path;

#[derive(serde::Serialize)]
pub struct Article {
    pub title: String,
    pub date: chrono::NaiveDate,
    pub content: String,
    pub slug: String,
    pub draft: bool,
    pub description: String,
}

pub type Path = String;

pub type Articles = Vec<Article>;

#[derive(serde::Serialize)]
struct YearArchive {
    articles: Articles,
}

type YearArchives = HashMap<i32, YearArchive>;

#[derive(serde::Deserialize)]
struct Frontmatter {
    title: String,
    date: String,
    slug: String,
    draft: bool,
    description: String,
}

pub fn create_homepage_html_file(articles: Articles, output_dir_path: Path, is_production: bool) -> io::Result<()> {
    if !path::Path::new(&output_dir_path).is_dir() {
        fs::create_dir(&output_dir_path)?;
    }

    fs::write(
        (output_dir_path + "index.html").to_string(),
        create_homepage_html(
            articles,
            read_file_content("templates/homepage.html".to_string()),
            is_production
        ),
    )?;

    Ok(())
}

pub fn copy_assets_to_output_dir(assets_path: &str, output_dir_path: &str) {
    if path::Path::new(&assets_path).is_dir() {
       let _ = copy_files_in_dir_to_dst(assets_path, output_dir_path);
    }
}

fn copy_files_in_dir_to_dst(src_dir: &str, dst_dir: &str) -> io::Result<()> {
    fs::create_dir_all(dst_dir)?;

    for entry in fs::read_dir(&src_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_name = entry.file_name();
            let dest_path = path::Path::new(&dst_dir).join(file_name);
            fs::copy(&path, &dest_path)?;
        } else if path.is_dir() {
            println!("{}", path.to_str().unwrap());
            let _dst_dir = path::Path::new(&dst_dir).join(entry.file_name());
            fs::create_dir_all(&_dst_dir)?;
            let _ = copy_files_in_dir_to_dst(&path.to_str().unwrap(), _dst_dir.to_str().unwrap());
        }
    }

    Ok(())
}

pub fn create_homepage_html(articles: Articles, homepage_template: String, is_production: bool) -> String {
    let year_archives = create_year_archives(articles, is_production);
    let mut env = Environment::new();
    let mut years: Vec<_> = year_archives.keys().collect();
    years.sort();
    years.reverse();

    let _ = env.add_template("homepage", &homepage_template);

    let tmpl = env.get_template("homepage").unwrap();

    return tmpl
        .render(context! { years => years, year_archives => year_archives })
        .unwrap();
}

fn create_year_archives(articles: Articles, is_production: bool) -> YearArchives {
    let mut year_archives: YearArchives = HashMap::new();

    for article in articles {
        if !article.draft || !is_production {
            let year = article.date.year();
            if let Some(year_archive) = year_archives.get_mut(&year) {
                year_archive.articles.push(article);
            } else {
                year_archives.insert(
                    year,
                    YearArchive {
                        articles: vec![article],
                    },
                );
            }
        }
    }

    for year_archive in year_archives.values_mut() {
        year_archive.articles.sort_by(|a, b| b.date.cmp(&a.date));
    }

    return year_archives;
}

pub fn create_article_html_file(
    article: &Article,
    article_template_path: Path,
    output_dir_path: Path,
) -> io::Result<()> {
    if !path::Path::new(&output_dir_path).is_dir() {
        create_dir(&output_dir_path)?;
    }
    fs::write(
        String::from(output_dir_path + &(article.slug.clone() + ".html")),
        convert_article_to_html(&read_file_content(article_template_path), article),
    )?;
    Ok(())
}

pub fn convert_article_to_html(article_html_template: &str, article: &Article) -> String {
    let mut env = Environment::new();
    let _ = env.add_template("article", article_html_template);

    let tmpl = env.get_template("article").unwrap();

    return tmpl
        .render(context! { title => article.title, content => article.content, description => article.description })
        .unwrap();
}

pub fn get_article_filepaths(article_directory: &str) -> io::Result<Vec<Path>> {
    let mut article_filepaths: Vec<Path> = vec![];

    for entry in fs::read_dir(article_directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let sub_files = get_article_filepaths(path.to_string_lossy().as_ref())?;
            article_filepaths.extend(sub_files);
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    article_filepaths.push(path.to_str().unwrap().to_string());
                }
            }
        }
    }

    return Ok(article_filepaths);
}

pub fn get_all_articles(article_filepaths: Vec<Path>) -> Vec<Article> {
    let mut articles: Vec<Article> = vec![];

    for article_filepath in article_filepaths {
        articles.push(read_article_from_file(article_filepath));
    }

    return articles;
}

fn read_article_from_file(article_filepath: Path) -> Article {
    let file_contents = read_file_content(article_filepath);
    let (frontmatter, body) = markdown_frontmatter::parse::<Frontmatter>(&file_contents).unwrap();
    let compile_options = markdown::CompileOptions {
        allow_dangerous_html: true,
        ..CompileOptions::default()
    };
    let options = markdown::Options {
        compile: compile_options,
        ..markdown::Options::default()
    };
    let article = Article {
        title: frontmatter.title,
        date: chrono::NaiveDate::parse_from_str(&frontmatter.date, "%Y-%m-%d")
            .expect("The format of date is incorrect."),
        content: markdown::to_html_with_options(body, &options).unwrap(),
        slug: frontmatter.slug,
        draft: frontmatter.draft,
        description: frontmatter.description,
    };
    return article;
}

pub fn read_file_content(filepath: Path) -> String {
    let contents = match fs::read_to_string(filepath) {
        Ok(contents) => {
            contents
        }
        Err(_e) => {
           panic!("File doesn't exist.") 
        }
    };

    return contents;
}

pub fn check_is_root(directory_path: Path) -> bool {
    return path::Path::new(&(directory_path + "jet.toml")).is_file();
}
