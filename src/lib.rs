use crate::structs::newsapi::NewsAPIResponse;
use ureq::{serde_json, Agent};

pub mod structs;

fn get_articles(
    agent: &Agent,
    api_key: String,
) -> Result<NewsAPIResponse, Box<dyn std::error::Error>> {
    let url = format!("https://newsapi.org/v2/everything?q=tesla&from=2022-12-03&sortBy=publishedAt&apiKey={api_key}");
    let articles: String = agent.get(url.as_str()).call()?.into_string()?;
    let articles: NewsAPIResponse = serde_json::from_str(articles.as_str())?;
    Ok(articles)
}

#[cfg(test)]
mod tests {
    // use super::*;

    use crate::get_articles;
    use dotenv::dotenv;
    use std::env;
    use std::num::TryFromIntError;
    use std::time::Duration;

    /// Try and convert a type that implements the `TryFrom<u64>` to a duration by method chaining.
    /// the function `to_duration` returns a `Result` which allows for ? use.
    trait ToDuration<T: TryFrom<u64> = Self> {
        fn to_duration(self) -> Result<Duration, TryFromIntError>;
    }

    impl ToDuration for i32 {
        fn to_duration(self) -> Result<Duration, TryFromIntError> {
            Ok(Duration::from_secs(u64::try_from(self)?))
        }
    }

    #[test]
    fn get_request() -> Result<(), Box<dyn std::error::Error>> {
        dotenv()?;
        // This is done at compile time, and is a &'static str
        // let x = env!("API_KEY")?;
        // This is done at runtime, and is an owned String.
        let (api_key, timeout): (String, Duration) = (
            env::var("API_KEY")?,
            env::var("TIMEOUT")?.parse::<i32>()?.to_duration()?,
        );
        let agent = ureq::AgentBuilder::new()
            .timeout_read(timeout)
            .timeout_write(timeout)
            .build();
        let articles = get_articles(&agent, api_key)?;
        for article in articles.iter() {
            // println!("{}", article.format());
            article.pretty_print();
        }
        Ok(())
    }
}
