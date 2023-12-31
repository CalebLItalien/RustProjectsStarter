use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}"); // problem with err
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
