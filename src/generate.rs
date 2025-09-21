use chrono::{Datelike};
use minijinja::{context, Environment};
use serde;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path;
use crate::helper;
use crate::articles::Articles;

pub type Path = String;

#[derive(serde::Serialize)]
struct YearArchive {
    articles: Articles,
}

type YearArchives = HashMap<i32, YearArchive>;

pub fn create_homepage_html_file(articles: Articles, output_dir_path: &Path, is_production: bool) -> io::Result<()> {
    if !path::Path::new(&output_dir_path).is_dir() {
        fs::create_dir(&output_dir_path)?;
    }

    let mut homepage_html_filename = path::PathBuf::from(output_dir_path);
    homepage_html_filename.push("index.html");

    fs::write(
        homepage_html_filename.to_str().unwrap(),
        create_homepage_html(
            articles,
            helper::read_file_content("templates/homepage.html".to_string()),
            is_production
        ),
    )?;

    return Ok(());
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
