use std::env;
use std::process;
// crate import
use minigrep::Config;

// in main.rs
fn main() {
    // let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args);

    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // if let match err
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
