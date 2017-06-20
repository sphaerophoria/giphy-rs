extern crate hyper;
extern crate tokio_core;
#[macro_use]
extern crate lazy_static;
extern crate url;
extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate newtype_derive;

mod giphy_types;

pub use giphy_types::SearchResults;

use futures::Stream;
use hyper::client::HttpConnector;
use hyper::{Client, Uri};
use std::convert::From;
use std::str::FromStr;
use url::Url;
use futures::Future;


lazy_static! {
    static ref GIPHY_URI: Url = Url::parse("http://api.giphy.com/").expect("Failed to parse Giphy URL (should never happen)");
}

type ParameterList<'a> = &'a[(&'a str, &'a str)];

/// Giphy API using tokio and futures
pub struct Giphy {
    api_key: String,
    client: Client<HttpConnector>
}

impl Giphy {
    /// Creates a new Giphy api executor. Puts http requests on the event loop specified by
    /// handle.
    pub fn new(api_key: String, handle: tokio_core::reactor::Handle) -> Giphy {
        let client = Client::new(&handle);
        Giphy { api_key: api_key, client: client }
    }

    /// Searches giphy for the query represented by query.
    /// Returns a future to be executed on tokio
    pub fn search(&self, query: &str) -> Box<Future<Item=SearchResults, Error=hyper::error::Error>>
    {
        self.send_request::<SearchResults>("/v1/gifs/search", &[ ("q", query) ])
    }

    /// Helper function for public api functions. Sends params to path and attempts to deserialize
    /// response into T
    fn send_request<T>(&self, path: &str, params: ParameterList) -> Box<Future<Item=T, Error=hyper::error::Error>>
        where T: From<hyper::Chunk>
    {
        let url = GIPHY_URI.clone();
        let mut url = url.join(path).unwrap();

        url.query_pairs_mut().extend_pairs(params.iter());
        url.query_pairs_mut().append_pair("api_key", &self.api_key);

        let hyper_url = Uri::from_str(url.as_str()).unwrap();
        Box::new(self.client.get(hyper_url)
            .map(hyper::Response::body)
            .and_then(|body| body.concat2())
            .map(|body| T::from(body)))
    }

}

