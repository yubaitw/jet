use std::env;
mod commands;
mod generate;
mod server;
mod helper;
mod articles;

fn main() {
    let current_directory = String::from("./");

    if !helper::check_is_root(current_directory) {
        println!("Error: the current directory is not a Jet project.");
        return;
    } else {
        let args: Vec<String> = env::args().collect();
        let argc = args.len();

        if argc >= 2 {
            let command = &args[1].clone();

            commands::execute_commnads(command.as_str(), argc, args.clone());
        } else {
            println!("Wrong number of arguments");
        }
    }
}
