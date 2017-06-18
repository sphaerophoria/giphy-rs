mod search_results {
    use hyper::Chunk;
    use serde_json;

    #[derive(Deserialize, Debug)]
    struct Giphy {
        data: Results,
    }

    #[derive(Deserialize, Debug)]
    pub struct Result {
        pub url: String,
        pub images: Images,
    }

    #[derive(Deserialize, Debug)]
    pub struct Images {
        pub fixed_height: Image,
        pub fixed_width: Image,
    }

    #[derive(Deserialize, Debug)]
    pub struct Image {
        pub url: String,
        pub width: String,
        pub height: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Results(Vec<Result>);
    NewtypeDeref! { () struct Results(Vec<Result>); }

    impl From<Chunk> for Results {
        fn from(chunk: Chunk) -> Self {
            let res: Giphy = serde_json::from_slice(&chunk).unwrap();
            res.data
        }
    }
}

pub use self::search_results::Results as SearchResults;
