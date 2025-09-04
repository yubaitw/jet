use std::env;
mod commands;
mod generate;
mod server;

fn main() {
    let args: Vec<String> = env::args().collect();

    let current_directory = String::from("./");

    if !generate::check_is_root(current_directory) {
        println!("Error: the current directory is not a Jet project.");
        return;
    } else {
        let argc = args.len();

        if argc >= 2 {
            let command = args[1].as_str();

            match command {
                "build" => {
                    if argc == 2 {
                        commands::build_site();
                    }
                }
                "serve" => {
                    if argc == 2 {
                        commands::serve();
                    } else {
                        println!("Wrong number of arguments");
                    }
                }
                "create" => {
                    if argc == 3 {
                        let article_slug = args[2].clone();
                        let _ = commands::create_article(article_slug);
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
        } else {
            println!("Wrong number of arguments");
        }
    }
}
