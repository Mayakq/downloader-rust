use std::io::stdin;

use dwmanager::lib::{read_file, Directory, Download};

#[tokio::main]
async fn main() {
    start().await;
}
async fn start() {
    let mut extension = String::with_capacity(3);
    println!("Write extension for save files. For example - png");
    stdin().read_line(&mut extension).expect("Error read from console");
    let directory = Directory::Default;
    let urls = read_file("./urls.txt".to_string()).await;
    let url = dwmanager::lib::get_url(urls.unwrap()).await;
    let manager = Download::new(url, extension, directory).unwrap();
    manager.download().await;
}
