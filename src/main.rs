use std::env;
mod commands;
mod generate;
mod server;

fn main() {
    let args: Vec<String> = env::args().collect();

    if !generate::check_is_root(String::from("./")) {
        println!("Error: the current directory is not a Jet project.");
        return;
    }

    if args.len() == 2 {
        if args[1] == "build" {
            commands::build_site();
        } else if args[1] == "serve" {
            commands::serve();
        } else {
            println!("Error: command error: unknown command \"{}\" for \"jet\"", args[1]);
        }
    } else if args.len() == 3 {
        if args[1] != "create" {
            println!("Error: command error: unknown command \"{}\" for \"jet\"", args[1]);
        } else {
            let article_slug = args[2].clone();
            let _ = commands::create_article(article_slug);
        }
    } else {
        println!("Wrong number of arguments");
    }
}

