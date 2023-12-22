use dwmanager::lib::{Directory, Download, read_file};
use url::Url;
#[tokio::main]
async fn main() {
    start().await;

}
async fn start(){
    let url = Url::parse(
        "https://yastat.net/s3/rasp/s/morda-front/_/images/hotelBanner/bannerDesktop.jpg",
    );
    let extension = ".png".to_string();
    let directory = Directory::Default;
    let urls = read_file("./urls.txt".to_string()).await;
    
    match url {
        Err(err) => {
            panic!("{}", err.to_string())
        }
        Ok(url) => {
            let manager = Download::new(url, extension, directory).unwrap();

        }
    }
}