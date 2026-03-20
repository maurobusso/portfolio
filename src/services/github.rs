use reqwest::{ header::{USER_AGENT, ACCEPT}};
use crate::models::PortfolioRepo;
use crate::models::{GitHubService};
use crate::models::GitHubSearchResponse;

impl GitHubService {
    pub fn new(username: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            username: username.to_string(),
            base_url: "https://api.github.com".to_string(),
        }
    }

    pub async fn get_portfolio_repos(&self) -> Result<Vec<PortfolioRepo>, reqwest::Error> {
        let url = format!("{}/search/repositories", self.base_url);
        let query = format!("user:{}+topic:portfolio+fork:true", self.username);

        let response: GitHubSearchResponse = self.client
        .get(url)
        .query(&[("q", &query)]) // <--- Reqwest handles the '?' and encoding for you!
        .header(USER_AGENT, "portfolio-service")
        .header(ACCEPT, "application/vnd.github.v3+json")
        .send()
        .await?
        .error_for_status()?
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

#[cfg(test)] // This tells Rust to only compile this when running 'cargo test'
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use serde_json::json;

    #[tokio::test]
    async fn test_get_portfolio_repos_success() {
        // 1. Start a local "fake" GitHub server
        let mock_server = MockServer::start().await;

        // 2. Prepare fake data that looks like GitHub's response
        let fake_github_response = json!({
            "items": [
                {
                    "name": "my-cool-project",
                    "html_url": "https://github.com/user/my-cool-project",
                    "description": "A great project",
                    "stargazers_count": 10,
                    "topics": ["rust", "wasm"]
                }
            ]
        });

        // 3. Tell the mock server how to behave
        Mock::given(method("GET"))
            .and(path("/search/repositories"))
            // We check if our code sends the right query parameters
            .and(query_param("q", "user:testuser+topic:portfolio+fork:true"))
            .respond_with(ResponseTemplate::new(200).set_body_json(fake_github_response))
            .mount(&mock_server)
            .await;

        // 4. Create our service, pointing it to the fake server instead of the real GitHub
        // NOTE: In production, your URL is hardcoded.
        // For testing, you'd usually pass the base URL into the 'new' method.
        let service = GitHubService {
            client: reqwest::Client::new(),
            username: "testuser".to_string(),
            base_url: mock_server.uri(),
        };

        // 5. Run the function!
        // (Note: Since your URL is currently hardcoded in your function,
        // this test would normally fail unless we make the URL configurable.)
        let result = service.get_portfolio_repos().await.expect("Should work");

        // 6. Verify the results
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "my-cool-project");
        assert_eq!(result[0].tech_stack, vec!["rust", "wasm"]);
        assert!(result[0].image_url.contains("testuser"));
    }

    #[tokio::test]
    async fn test_get_portfolio_repos_server_error() {
        // 1. Start the fake server
        let mock_server = MockServer::start().await;

        // 2. Tell the mock server to return a 500 Error
        Mock::given(method("GET"))
            .and(path("/search/repositories"))
            .respond_with(ResponseTemplate::new(500)) // <--- Simulate a crash!
            .mount(&mock_server)
            .await;

        let service = GitHubService {
            client: reqwest::Client::new(),
            username: "testuser".to_string(),
            base_url: mock_server.uri(),
        };

        // 3. Run the function
        let result = service.get_portfolio_repos().await;

        // 4. Assert that it IS an error
        // We expect this to fail because GitHub "returned" a 500
        assert!(result.is_err());

        // You can even check if it's specifically a reqwest error
        let err = result.unwrap_err();
        assert!(err.is_status());
        assert_eq!(err.status().unwrap(), 500);
    }

    #[tokio::test]
    async fn test_get_portfolio_repos_not_found() {
        let mock_server = MockServer::start().await;

        // Simulate GitHub saying "This user does not exist"
        Mock::given(method("GET"))
            .and(path("/search/repositories"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let service = GitHubService {
            client: reqwest::Client::new(),
            username: "non-existent-user".to_string(),
            base_url: mock_server.uri(),
        };

        let result = service.get_portfolio_repos().await;

        // Verify it is an error and specifically a 404
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.status(), Some(reqwest::StatusCode::NOT_FOUND));
    }
}