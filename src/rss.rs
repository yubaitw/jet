use rss::{ChannelBuilder, Item, ItemBuilder};
use chrono::{Utc, TimeZone};
use crate::blog::Blog;
use crate::articles::{Article, Articles};
use std::fs;

pub fn create_rss_xml(blog: &Blog, output_dir: String) {
    let rss_content = create_rss_content(blog);

    fs::write(output_dir + "/rss.xml", rss_content).unwrap();
}

fn create_rss_content(blog: &Blog) -> String {
    let channel = ChannelBuilder::default()
        .title(&blog.config.title)
        .link(format!("{}rss.xml", &blog.config.base_url))
        .description(&blog.config.description)
        .items(create_article_items(blog.config.base_url.to_string(), &blog.articles))
        .build();

    return channel.to_string();
}

fn create_article_items(base_url: String, articles: &Articles) -> Vec<Item> {
    let mut article_items: Vec<Item> = vec![];

    for article in articles {
        if !article.draft {
            article_items.push(make_article_item(base_url.clone(), article));
        }
    }

    return article_items;
}

fn make_article_item(base_url: String, article: &Article) -> Item {
    let pub_date = Utc.from_utc_datetime(&article.date.and_hms_opt(12, 0, 0).unwrap()).to_rfc2822();

    return ItemBuilder::default()
        .title(article.title.clone())
        .link(base_url + "/posts/" + &article.slug)
        .description(article.description.clone())
        .content(article.content.clone())
        .pub_date(pub_date)
        .build();
}
