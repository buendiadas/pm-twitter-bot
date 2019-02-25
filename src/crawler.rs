use futures::{Future, Stream};
use hyper_tls::HttpsConnector;
use hyper::Client;
use tokio_core::reactor::Core;
use serde_json;

use std::string::String;
use std::str;
use std;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct CurrencyPrice {
    pub results: Vec<Res>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Res {
    pub contract: Contract,
    pub marginalPrices: Vec<String>,
    pub event: Event,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contract { 
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event { 
    pub oracle: Oracle,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Oracle {
    pub eventDescription: EventDescription,

}
#[derive(Serialize, Deserialize, Debug)]
pub struct EventDescription {
    pub title: String,
    pub ipfsHash: String,
}



pub struct Crawler {
}

impl Crawler {
    pub fn new() -> Crawler {
        let c = Crawler {};
        return c;
    }

    fn parse_content (&self, content : String) -> Vec<Res> {
        let s_slice: &str = &*content;
        let v: CurrencyPrice = serde_json::from_str(s_slice.clone()).unwrap();
        return v.results;
    }


    fn http_get(&self, url : &str) -> String {
        let mut core = Core::new().unwrap();
        let client = Client::configure()
                .connector(HttpsConnector::new(4, &core.handle()).unwrap())
                    .build(&core.handle());

        let uri = url.parse().unwrap();
        let work = client.get(uri).and_then (|res| {res.body().concat2()})
            .map(|chunk| std::str::from_utf8(&chunk).expect("error handling").to_string());
        let r = core.run(work).unwrap();
        return r;
    }


    pub fn get_markets (&self) -> Vec<Res> {
        let url = "https://olympia-api.helena.network/api/markets/?event_oracle_is_outcome_set=false";
        let content = self.http_get(&url[..]); 
        let values: Vec<Res>  = self.parse_content(content);
        return values;
    }
}




