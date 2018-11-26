use std::env;

use url::Url;
use reqwest;
use m3u8_rs::playlist::{MasterPlaylist, VariantStream};
use m3u8_rs::parse_master_playlist_res;


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
    let base_url = base_path_from_url(&playlist_url) + "/";
    let master = load_master_playlist(&playlist_url).unwrap();
    let highest_bandwith_stream = select_highest_bandwidth(&master).unwrap();

    println!("Base url: {:?}", base_url);
    println!("Master Playlist:\n{:?}", &master);
    println!("Highest bitrate:{:?}", &highest_bandwith_stream);
}


fn base_path_from_url(url: &str) -> String {
    let mut parsed = Url::parse(url).unwrap();
    parsed.path_segments_mut().unwrap().pop();
    parsed.to_string()
}


fn load_master_playlist(url: &str) -> Option<MasterPlaylist> {
    let mut buf: Vec<u8> = vec![];
    let mut resp= reqwest::get(url).unwrap();
    resp.copy_to(&mut buf).unwrap();

    let parse_result = parse_master_playlist_res(&buf);
    parse_result.ok()
}


fn select_highest_bandwidth(master: &MasterPlaylist) -> Option<&VariantStream> {
    master.variants.iter(). get(0)
}