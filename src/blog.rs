use serde;
use toml;
use crate::helper;
use crate::generate::{Path};
use crate::articles::Articles;

pub struct Blog {
    pub config: Config,
    pub articles: Articles
}

#[derive(serde::Deserialize)]
pub struct Config {
    pub title: String,
    pub base_url: String,
    pub description: String,
}

pub fn read_blog_config(config_filepath: Path) -> Config {
    let toml_content = helper::read_file_content(config_filepath);
    let blog: Config = match toml::from_str(toml_content.as_str()) {
        Ok(blog) => {
            blog
        }
        Err(_e) => {
            panic!("jet.toml are incomplete!")
        }
    };

    return blog;
}
