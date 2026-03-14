use reqwest::{ header::{USER_AGENT, ACCEPT}};
use crate::models::PortfolioRepo;
use crate::models::{GitHubService};
use crate::models::GitHubSearchResponse;

impl GitHubService {
    pub fn new(username: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            username: username.to_string(),
        }
    }

    pub async fn get_portfolio_repos(&self) -> Result<Vec<PortfolioRepo>, reqwest::Error> {
        let url = format!(
            "https://api.github.com/search/repositories?q=user:{}+topic:portfolio+fork:true",
            self.username
        );

        // Fetch data from GitHub
        let response: GitHubSearchResponse = self.client
            .get(url)
            .header(USER_AGENT, "portfolio-service")
            .header(ACCEPT, "application/vnd.github.v3+json")
            .send()
            .await?
            .json()
            .await?;

        // Transform the raw API items into your Template-friendly format
        let items = response.items.into_iter().map(|item| {
            PortfolioRepo {
                name: item.name.clone(),
                html_url: item.html_url.clone(), // maps to repo_url
                description: item.description,
                stargazers_count: item.stargazers_count,
                image_url: format!("https://opengraph.githubassets.com/1/{}/{}", self.username, item.name),
                tech_stack: item.topics
            }
        }).collect();

        Ok(items)
    }
}
