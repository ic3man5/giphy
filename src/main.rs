use getopts::Options;
use std::env;
use giphy::v1::gifs::SearchRequest;
use giphy::v1::sync::*;
use reqwest;
use clipboard_win::{formats, set_clipboard};
use rand::thread_rng;
use rand::seq::SliceRandom;

// Gets a "help" String to print to the console.
pub fn print_usage_string(program_name: &str, opts: &Options) {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let brief = format!("Usage: {} FILE [options]", program_name);
    println!("Version: {}\n{}", VERSION, opts.usage(&brief));
}

fn main() {
    let api_key = "WPWGNRtoMq37sWokCR2GGIiHIWXQlPRG";
    // Lets parse the arguments
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    // Help argument
    opts.optflag("h", "help", "Displays this help menu");
    // Version argument
    opts.optflag("v", "version", "Displays the version");
    // Make sure we have some arguments
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f),
    };
    // Parse the arguments
    if matches.opt_present("v") {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("Version: {}", VERSION);
        return;
    }
    if matches.opt_present("h") {
        print_usage_string(&args[0], &opts);
        return;
    }
    if matches.free.is_empty() {
        panic!("search argument needed to grab a giphy!");
    }
    // Finally lets get to the real stuff
    let search_string = &matches.free[0];
    let client = reqwest::blocking::Client::new();
    let api = SyncApi::new(api_key.to_string(), client);
    let response = SearchRequest::new(search_string)
        //.with_limit(20)
        .send_to(&api)
        .unwrap_or_else(|e|
            panic!("Search request failed: {:?}", e)
        
        );
    println!("Found {} results...", response.data.len());        
    let mut rng = thread_rng();
    let result = response.data.choose(&mut rng)
        .unwrap_or_else(||
            panic!("No results returned from Giphy!")
        );
    // Copy the result to the clipboard
    let value = format!("`{}`\n{}", search_string, result.embed_url);
    set_clipboard(formats::Unicode, value).unwrap();
    println!("Copied {} to the clipboard", result.embed_url);
}
