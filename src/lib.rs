pub mod lib {

    use core::panic;
    use std::fs::File;

    use futures::StreamExt;
    use url::Url;
    #[derive(Debug)]
    pub struct Download {
        pub url: Vec<Url>,
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
        pub fn new(url: Vec<Url>, extension: String, directory: Directory) -> Option<Self> {
            Some(Self {
                url,
                extension,
                directory_for_save: directory,
            })
        }
        pub async fn download(&self) {
            let client = reqwest::Client::new();
            for url in &self.url {
                let response = client.get(url.to_string()).send().await.unwrap();
                let length = response.content_length().unwrap(); // todo(PROGRESS BAR)
                let mut stream = response.bytes_stream();
                let mut file = File::create("./data.jpg").unwrap();
                while let Some(chunk) = stream.next().await {
                    let item = chunk.unwrap();
                    std::io::Write::write_all(&mut file, &item).unwrap();
                }
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
    pub async fn get_url(array_str: String) -> Vec<Url> {
        let mut vec = Vec::with_capacity(10);
        for elem in array_str.split("\n") {
            match Url::parse(elem) {
                Ok(url) => {
                    vec.push(url);
                }
                Err(error) => println!("Get error at parse urls - {}", error.to_string()),
            }
        }
        if vec.len() == 0 {
            panic!("Error parse url from file. Count parsed url = 0");
        }
        vec
    }
}
