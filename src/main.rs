use std::env;

use reqwest;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        show_usage();
    } else {
        monitor_stream(&args[1]);
    }
}

fn show_usage() {
    println!("Usage:");
    println!(" vsm <master-playlist-url>")
}

fn monitor_stream(playlist_url: &str) {
    let mut res = reqwest::get(playlist_url).unwrap();

    // copy the response body directly to stdout
    let _ = std::io::copy(&mut res, &mut std::io::stdout());
}