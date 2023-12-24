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
            let mut counter = 0;
            let mut threads = Vec::with_capacity(self.url.len());
            for url in self.url.clone() {
                let path = format!("./{}.{}", counter, "png");
                let file = File::create(path).unwrap();
                let thread = tokio::spawn(async move {
                    write_bytes(url, file).await;
                });
                counter +=1;
                threads.push(thread);
            }
            for thread in threads {
                thread.await.unwrap();
            }
        }
    }

    pub async fn read_file(path: String) -> Option<String> {
        let lines = tokio::fs::read(path).await;
        match lines {
            Ok(result) => {
                return Some(String::from_utf8(result).unwrap());
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

    pub async fn write_bytes(url: Url,  mut file: File) {
        let client = reqwest::Client::new();
        let response = client.get(url.to_string()).send().await.unwrap();
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let item = chunk.unwrap();
            std::io::Write::write_all(&mut file, &item).unwrap();
        }
    }
}
