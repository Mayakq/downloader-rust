pub mod lib {

    use core::panic;
    use std::{fmt::Write, fs::File};

    use futures::{Stream, StreamExt};
    use tokio::fs;
    use url::Url;
    #[derive(Debug)]
    pub struct Download {
        pub url: Url,
        pub extension: String,
        pub directory_for_save: Directory,
    }
    #[derive(Debug)]
    pub enum Directory {
        /// Default is current directory ./
        Default,
        Custom(String),
    }

    impl Download {
        pub fn new(url: Url, extension: String, directory: Directory) -> Option<Self> {
            Some(Self {
                url,
                extension,
                directory_for_save: directory,
            })
        }
        pub async fn download(&self) {
            let client = reqwest::Client::new();
            let response = client.get(self.url.as_str()).send().await.unwrap();
            let length = response.content_length().unwrap(); // todo(PROGRESS BAR)
            let mut stream = response.bytes_stream();
            let mut file = File::create("./data.jpg").unwrap();
            while let Some(chunk) = stream.next().await {
                let item = chunk.unwrap();
                std::io::Write::write_all(&mut file, &item).unwrap();
            }
        }
    }
    pub async fn read_file(path: String) -> Option<Vec<u8>> {
        let lines = tokio::fs::read(path).await;
        match lines {
            Ok(result) => {
                return Some(result);
            }
            Err(err) => {
                panic!("{}", err)
            }
        }
    }
}
