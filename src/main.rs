extern crate serde_json;
extern crate pbr;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate twitter_api;
extern crate oauth_client as oauth;
extern crate job_scheduler;

use std::time::Duration;
use std::env;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use job_scheduler::{JobScheduler, Job};

mod config;
mod crawler;
mod twitter;


const CONF_FILENAME: &'static str = ".crypto-bot.conf";


#[derive(Serialize, Deserialize, Debug)]
pub struct Tweet {
    pub title: String,
    pub percentage: String,
    pub url: String,
}


fn get_home_dir() -> PathBuf {
    match env::home_dir() {
        Some(p) => p,
        None => {
            panic!("Impossible to get your home dir!");
        }
    }
}

pub fn build_message (markets : &mut HashMap<String, &crawler::EventDescription>) -> String {
    let mut s = String::from("");
    for (cur, value) in markets.iter() {
        let t = format!("#{}: {:?} \n", cur, value.title);
        s = s + &t[..];
    }

    return s;
}

pub fn build_message_from_tweet (tweet : &Tweet) -> String {
    let mut s = String::from("");
    let pct_float = tweet.percentage.parse::<f64>().unwrap();
    let bar_yes = create_bar(pct_float);
    let bar_no = create_bar(1.00 - pct_float);
    let pct_yes = pct_float * 100.00;
    let pct_no = 100.00 - pct_yes; 
    println!("{}", bar_yes);
    let t = format!("{:?} \n Outcome probabilities📊: \n \n Yes:  {} {}% \n \n No:   {} {}% \n \n Check it out at: \n  \n 👇\n https://pm.helena.network/markets/{} \n", tweet.title, bar_yes, pct_yes, bar_no, pct_no, tweet.url);
    s = s + &t[..];
    return s;
}

pub fn create_bar(pct: f64) -> String {
    let mut s = String::from("");
    let sym_yes = "⬛";
    let sym_no = "⬜";
    let total = 15;
    let num_bars_yes = (pct * 15.00) as i32;
    let num_bars_no = 15 - &num_bars_yes;
    
    for number in (1..15) {
        if(number <= num_bars_yes){
            s = s + &sym_yes[..];
        }
        else {
            s = s + &sym_no[..];
        }
    }
    s
}

pub fn build_tweet(res: &crawler::Res) -> Tweet { 
    let result = &res;
    let title = &result.event.oracle.eventDescription.title;
    let percentage = &result.marginalPrices[0];
    let url = &result.contract.address;
    Tweet {
        title: title.to_string(),
        percentage: percentage.to_string(),
        url: url.to_string(),
    }
}

fn main() {
    let mut conf_file_path: PathBuf = get_home_dir();
    conf_file_path.push(Path::new(CONF_FILENAME));


    let config = match config::Config::read(&conf_file_path) {
        Some(v) => v,
        None => panic!("Cannot find config file"),
    };

    let crawler = crawler::Crawler::new();
    let twitter = twitter::Twitter::new(config.consumer_key, config.consumer_secret,
                                        config.access_key, config.access_secret);
    let res: Vec<crawler::Res> = crawler.get_markets();
    let mut sched = JobScheduler::new();

    sched.add(Job::new(config.cron_expression.parse().unwrap(), || {
        for events in &res {
            if(&events.event.type_name == "CATEGORICAL"){
                let tweet: Tweet = build_tweet(&events);
                let msg_tw: String = build_message_from_tweet(&tweet);
                //twitter.tweet(msg_tw);
                println!("{}", msg_tw);
            }
        }
    }));
    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
}
