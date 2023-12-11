use url::Url;
#[derive(Debug)]
struct Download {
    url: Url,
    extension: String,
    directory_for_save: String,
}
#[derive(Debug)]
enum Directory{
    Default(String),
    Custom(String)
}
fn main() {
    println!("Hello, world!");
    
}
