use crate::misc;
use colour::{blue_ln, dark_green_ln, green_ln, magenta_ln, yellow_ln};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Article {
    source: Source,
    author: Option<String>,
    title: String,
    description: Option<String>,
    url: String,
    urlToImage: Option<String>,
    publishedAt: String,
    content: String,
}
impl Article {
    pub fn format(&self) -> String {
        let binding = String::from("No Author");
        let author = misc::optional_value(&self.author, &binding);
        format!(
            "\nFrom: {}\nAuthor: {author}\nTitle: {}\nContent: {}\n\tURL: {}",
            self.source.name, self.title, self.content, self.url
        )
    }

    pub fn pretty_print(&self) {
        let binding = String::from("No Author");
        let author = misc::optional_value(&self.author, &binding);
        dark_green_ln!("From: {}", self.source.name);
        yellow_ln!("Author: {}", author);
        blue_ln!("Title: {}", self.title);
        green_ln!("Content: {}", self.content);
        magenta_ln!("\t\tURL: {}", self.url);
    }
}

#[derive(Debug, Deserialize)]
struct Source {
    id: Option<String>,
    name: String,
}

#[cfg(test)]
mod tests {
    use crate::structs::article::Article;
    use ureq::serde_json;

    #[test]
    fn article_parse() -> Result<(), Box<dyn std::error::Error>> {
        let input = r#"{"source":{"id":null,"name":"New Zealand Herald"},
        "author":null,"title":"Financial Times: Business trends, risks and people to watch in 2023",
        "description":"What to look for in the corporate world in 2023 - from energy to private capital and tech.",
        "url":"https://www.nzherald.co.nz/business/financial-times-business-trends-risks-and-people-to-watch-in-2023/25NZLTATHVCQ7KXBHNWL2TZFB4/",
        "urlToImage":"https://www.nzherald.co.nz/resizer/WaRt7QAvwPX9PXIZq67r0w6pG-Q=/1200x675/smart/filters:quality(70)/cloudfront-ap-southeast-2.images.arcpublishing.com/nzme/TKXC653L3BGFZLUGE37QXPOEWE.jpg",
        "publishedAt":"2023-01-03T04:00:00Z",
        "content":"Trends, people and risks to watch in 2023. Photo / Getty ImagesWritten by: FT reportersrnThis time last year, companies were wondering if there was an end in sight to the Covid-19 pandemic. Then in F… [+13852 chars]"}
        "#;
        let _result: Article = serde_json::from_str(input)?;
        Ok(())
    }
}
