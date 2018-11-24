use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        show_usage();
    } else {
        monitor(&args[1]);
    }
}

fn show_usage() {
    println!("Usage:");
    println!(" vsm <master-playlist-url>")
}

fn monitor(playlist_url: &str) {
    println!("Looking for master playlist at:");
    println!("{}", playlist_url)
}