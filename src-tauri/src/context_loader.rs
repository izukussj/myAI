use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Context {
    pub system: String,
    pub instructions: String,
    pub global_rules: Vec<String>,
    pub api_config: Vec<ApiConfig>,
    pub examples: Vec<Example>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiConfig {
    pub name: String,
    pub description: String,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Endpoint {
    pub name: String,
    pub method: String,
    pub params: Vec<Param>,
    pub response_format: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Param {
    pub name: String,
    pub r#type: String,
    pub required: bool,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Example {
    pub user_query: String,
    pub response: String,
}

pub fn load_context(file_path: &str) -> Result<Context, Box<dyn Error>> {
    let yaml_content = fs::read_to_string(file_path)?;
    let context: Context = serde_yaml::from_str(&yaml_content)?;
    Ok(context)
}
