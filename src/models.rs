use serde::{ Serialize, Deserialize }; // Library for serializing json into code and vice versa
use reqwest::Client; // For making http requests
use chrono::NaiveDate; // Handles date and time
use tera::Tera; // Template engine

// In Rust, a "model" file defines the shape of the objects the code will handle—like blog posts
// or GitHub data—and how they should be converted to and from formats like JSON or HTML.

#[derive(Serialize, Clone)] //This is a macro. It tells the Rust compiler to automatically "write" some extra code for you so you don't have to do it manually.
pub struct BlogPost {
    pub title: String,
    pub slug: String,
    pub content: String, // HTML string
    pub date: NaiveDate,
    pub category: String,
    pub excerpt: String,
    pub status: String,
}

pub struct AppState {
    pub templates: Tera,
    pub github_service: GitHubService,
}

// 1. The data structure you want to use in your template
#[derive(Serialize, Debug, Clone)]
pub struct PortfolioRepo {
    pub name: String,
    pub html_url: String, // This will be your repo_url
    pub description: Option<String>,
    pub stargazers_count: u32,
    pub image_url: String,
    pub tech_stack: Vec<String>
}

// 2. The internal structure that matches GitHub's API response
#[derive(Deserialize, Debug)]
pub struct GitHubRawRepo {
    pub name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub stargazers_count: u32,
    pub topics: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct GitHubSearchResponse {
    pub items: Vec<GitHubRawRepo>,
}

#[derive(Debug)]
pub struct GitHubService {
    pub client: Client,
    pub username: String,
    pub base_url: String,
}