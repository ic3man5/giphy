use core::panic;
use getopts::Options;
use std::env;
//use giphy::v1::gifs::SearchRequest;
//use giphy::v1::sync::*;
//use reqwest;
use clipboard_win::{formats, set_clipboard};
use rand::seq::SliceRandom;
use rand::thread_rng;

use giphyc::utility::print_usage_string;
use giphyc::{Giphy, GiphySearchError, GiphyURLType};

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
    let urls = giphy
        .search_url(&search_text, GiphyURLType::Original, Some(300))
        .unwrap();

    println!("Found {} results...", urls.len());
    let mut rng = thread_rng();
    let result = urls.choose(&mut rng).unwrap();

    let value = format!("`{}`\n{}", search_text, result);
    set_clipboard(formats::Unicode, value).unwrap();

    println!("Copied {} to the clipboard", result);
}
