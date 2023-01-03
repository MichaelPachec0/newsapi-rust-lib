use crate::structs::article::Article;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewsAPIResponse {
    status: String,
    totalResults: i32,
    articles: Vec<Article>,
}

impl NewsAPIResponse {
    pub fn iter(&self) -> impl Iterator<Item = &Article> {
        self.articles.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::newsapi::NewsAPIResponse;
    use ureq::serde_json;

    #[test]
    fn news_api_parse() -> Result<(), Box<dyn std::error::Error>> {
        let input = r#"{"status":"ok","totalResults":18032,"articles":[{"source":{"id":null,"name":"New Zealand Herald"},"author":null,"title":"Financial Times: Business trends, risks and people to watch in 2023","description":"What to look for in the corporate world in 2023 - from energy to private capital and tech.","url":"https://www.nzherald.co.nz/business/financial-times-business-trends-risks-and-people-to-watch-in-2023/25NZLTATHVCQ7KXBHNWL2TZFB4/","urlToImage":"https://www.nzherald.co.nz/resizer/WaRt7QAvwPX9PXIZq67r0w6pG-Q=/1200x675/smart/filters:quality(70)/cloudfront-ap-southeast-2.images.arcpublishing.com/nzme/TKXC653L3BGFZLUGE37QXPOEWE.jpg","publishedAt":"2023-01-03T04:00:00Z","content":"Trends, people and risks to watch in 2023. Photo / Getty ImagesWritten by: FT reportersrnThis time last year, companies were wondering if there was an end in sight to the Covid-19 pandemic. Then in F… [+13852 chars]"}]}"#;
        let newsapi_test: NewsAPIResponse = serde_json::from_str(input)?;
        let articles_len = newsapi_test.articles.len();
        assert_eq!(articles_len, 1, "ERROR: newsapi: api response was not parsed correctly, expected 1 article, got {articles_len}");
        Ok(())
    }
}
