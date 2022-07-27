use getopts::Options;
use core::panic;
use std::env;
//use giphy::v1::gifs::SearchRequest;
//use giphy::v1::sync::*;
//use reqwest;
use clipboard_win::{formats, set_clipboard};
use rand::thread_rng;
use rand::seq::SliceRandom;

use giphyc::utility::print_usage_string;
use giphyc::{Giphy, GiphyURLType, GiphySearchError};


fn main() {
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

    let search_text = &matches.free.join(" ");
    let giphy = Giphy::new("WPWGNRtoMq37sWokCR2GGIiHIWXQlPRG".to_string());
    let urls = giphy.search_url(
        &search_text,
        GiphyURLType::Original,
        Some(300)).unwrap();
    for url in &urls {
        println!("url: {}", url);
    }


    /*
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
    let url_str = match &result.images.original.url {
        Some(s) => s,
        _ => panic!("Failed to get real gif URL for {}", result.url)
    };
    let value = format!("`{}`\n{}", search_string, url_str);
    set_clipboard(formats::Unicode, value).unwrap();

    println!("Copied {} to the clipboard", url_str);
    */
}
