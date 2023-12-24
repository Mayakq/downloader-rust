use dwmanager::lib::{read_file, Directory, Download};

#[tokio::main]
async fn main() {
    start().await;

}
async fn start() {
    let extension = "png".to_string();
    let directory = Directory::Default;
    let urls = read_file("./urls.txt".to_string()).await;
    let url = dwmanager::lib::get_url(urls.unwrap()).await;
    let manager = Download::new(url, extension, directory).unwrap();
    manager.download().await;
}

