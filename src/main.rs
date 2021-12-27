use std::{env,process};
use std::error::Error;
use command_line_tool_minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments : {}",err);
        process::exit(1);
    });
    eprintln!("Searching for {}",config.query);
    eprintln!("In file {}",config.filename);
    if let Err(e) = command_line_tool_minigrep::run(config){
        eprintln!("Application Error: {}",e);
        process::exit(1);
    }
}
