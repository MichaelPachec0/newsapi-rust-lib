use crate::structs::newsapi::NewsAPIResponse;
use std::num::TryFromIntError;
use std::time::Duration;

use ureq::{serde_json, Agent};
use crate::misc::ToDuration;

pub mod misc;
pub mod structs;

pub struct NewsApiLib<'a> {
    agent: Agent,
    url: String,
    api_key: &'a str,
    timeout: Duration,
}

impl<'a> NewsApiLib<'a> {
    pub fn new(
        api_key: &'a str,
        timeout: Option<i32>,
        initial_query: &str,
        date: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let timeout = timeout.unwrap_or(5).to_duration()?;
        let agent = ureq::AgentBuilder::new().timeout_read(timeout).build();
        let date = "2022-12-03";
        let url = format!("https://newsapi.org/v2/everything?q={initial_query}&from={date}&sortBy=publishedAt&apiKey={api_key}");
        Ok(Self {
            agent,
            url,
            api_key,
            timeout,
        })
    }
    pub fn get_articles(&self) -> Result<NewsAPIResponse, Box<dyn std::error::Error>> {
        let articles = self.agent.get(self.url.as_str()).call()?.into_string()?;
        let articles = serde_json::from_str::<NewsAPIResponse>(articles.as_str())?;
        Ok(articles)
    }
    pub fn change_topic(&mut self, topic: &str) {}
    pub fn change_api_key(&mut self, api_key: &'a str) {}
}

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::NewsApiLib;
    use dotenv::dotenv;
    use std::env;

    #[test]
    fn get_request() -> Result<(), Box<dyn std::error::Error>> {
        dotenv()?;
        // This is done at compile time, and is a &'static str
        // let x = env!("API_KEY")?;
        // This is done at runtime, and is an owned String.
        let (api_key, timeout): (String, i32) =
            (env::var("API_KEY")?, env::var("TIMEOUT")?.parse::<i32>()?);
        let query = "Hacker";
        let news_lib = NewsApiLib::new(api_key.as_str(), Some(timeout), query, None)?;
        let articles = news_lib.get_articles()?;

        for article in articles.iter() {
            // println!("{}", article.format());
            article.pretty_print();
        }
        Ok(())
    }
}
