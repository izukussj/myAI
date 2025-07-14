#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use tauri::{generate_context};
use std::collections::HashMap;
mod context_loader;
use crate::context_loader::{load_context, Context};
use serde_json::Value;
use thiserror::Error;
use log::error;
use reqwest::Response;
use tauri::ipc::InvokeError;  // Pour InvokeError
#[derive(Error, Debug)]
pub enum FetchIaResponseError {
    #[error("Erreur lors du chargement du contexte : {0}")]
    ContextLoadError(String),
    #[error("Erreur lors de la requête API : {0}")]
    ApiRequestError(String),
    #[error("Erreur de parsing : {0}")]
    ParsingError(String),
    #[error("Aucune réponse valide de l'API OpenAI.")]
    InvalidApiResponse,
}

impl Into<InvokeError> for FetchIaResponseError {
  fn into(self) -> InvokeError {
      InvokeError::from(self.to_string()) // Conversion en une chaîne de caractères
  }
}
#[derive(Deserialize, Clone)]
struct ApiEndpoint {
    method: String, // "GET", "POST", etc.
    path: String,   // "/chat/completions", etc.
}

#[derive(Deserialize, Clone)]
struct ApiConfig {
    base_url: String,
    token: String,
    endpoints: HashMap<String, ApiEndpoint>, // Clé : Nom de l'endpoint (e.g., "chat", "jobs")
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatGPTResponse {
  choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
  message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
  role: String,
  content: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
enum IaResponse {
    Windows {
        content: WindowsContent,
    },
    Api {
        apiName: String,
        content: ApiContent,
    },
    Basic {
        content: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct WindowsContent {
  dangerLevel: u8,
  description: String,
  code: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct ApiContent {
  response: String,
}

#[tokio::main]
async fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![fetch_ia_response, execute_command,simulate_ia_response])
      .run(generate_context!())
      .expect("error while running Tauri application");
}
#[tauri::command]
fn simulate_ia_response(arg: u8) -> IaResponse {
    match arg {
        0 => IaResponse::Windows {
            content: WindowsContent {
                dangerLevel: 1,
                description: String::from("This command creates a file on the desktop."),
                code: String::from("echo. > C:\\Users\\%USERNAME%\\Desktop\\newFile.txt"),
            },
        },
        1 => IaResponse::Api {
            apiName: String::from("mockAPI"),
            content: ApiContent {
                response: String::from("This is a simulated API response."),
            },
        },
        2 => IaResponse::Basic {
            content: String::from("This is a basic response."),
        },
        _ => IaResponse::Basic {
            content: String::from("Invalid argument. Defaulting to basic response."),
        },
    }
}
#[tauri::command]
async fn fetch_ia_response(query: String) -> Result<IaResponse, FetchIaResponseError> {
    tokio::spawn(async move {
        let context_file = "src/contexte.yaml";
        let context: Context = load_context(context_file)
            .map_err(|e| FetchIaResponseError::ContextLoadError(e.to_string()))?;

        let complete_prompt = generate_prompt(&context);

        let params = serde_json::json!( {
            "model": "gpt-4",
            "messages": [
                { "role": "system", "content": complete_prompt },
                { "role": "user", "content": query }
            ]
        });

        let response: Response = execute_api_request("openai", "completion", Some(params))
            .await
            .map_err(|e| FetchIaResponseError::ApiRequestError(e.to_string()))?;

        let chat_response: Value = response.json()
            .await
            .map_err(|e| FetchIaResponseError::ApiRequestError(e.to_string()))?;
            println!("{:?}", chat_response);
        parse_response(chat_response)
    }).await.unwrap_or_else(|e| Err(FetchIaResponseError::ApiRequestError(e.to_string())))
}

fn generate_prompt(context: &Context) -> String {
    let system_message = &context.system;
    let instructions = &context.instructions;
    let api_config = serde_yaml::to_string(&context.api_config).unwrap_or_default();

    format!(
        "{}\n\n{}\n\nConfiguration des APIs disponibles :\n{}",
        system_message, instructions, api_config
    )
}
fn parse_response(chat_response: Value) -> Result<IaResponse, FetchIaResponseError> {
  // Extraire le champ "choices"
  if let Some(choices) = chat_response.get("choices") {
      if let Some(choice) = choices.get(0) {
          // Extraire "message.content"
          if let Some(content) = choice
              .get("message")
              .and_then(|m| m.get("content"))
              .and_then(|c| c.as_str())
          {
              // Désérialiser la chaîne JSON en Value
              let parsed_json: Value = serde_json::from_str(content).map_err(|e| {
                  log::error!("Erreur de parsing JSON pour content: {}", e);
                  FetchIaResponseError::ParsingError(e.to_string())
              })?;

              // Identifier le type et traiter le champ content séparément
              match parsed_json.get("type").and_then(|t| t.as_str()) {
                  Some("windows") => {
                      let content_obj = parsed_json.get("content").ok_or_else(|| {
                          FetchIaResponseError::InvalidApiResponse
                      })?;
                      let windows_content: WindowsContent =
                          serde_json::from_value(content_obj.clone()).map_err(|e| {
                              log::error!("Erreur de parsing WindowsContent: {}", e);
                              FetchIaResponseError::ParsingError(e.to_string())
                          })?;
                      Ok(IaResponse::Windows {
                          content: windows_content,
                      })
                  }
                  Some("api") => {
                      let api_name = parsed_json
                          .get("apiName")
                          .and_then(|n| n.as_str())
                          .ok_or_else(|| FetchIaResponseError::InvalidApiResponse)?;
                      let content_obj = parsed_json.get("content").ok_or_else(|| {
                          FetchIaResponseError::InvalidApiResponse
                      })?;
                      let api_content: ApiContent =
                          serde_json::from_value(content_obj.clone()).map_err(|e| {
                              log::error!("Erreur de parsing ApiContent: {}", e);
                              FetchIaResponseError::ParsingError(e.to_string())
                          })?;
                      Ok(IaResponse::Api {
                          apiName: api_name.to_string(),
                          content: api_content,
                      })
                  }
                  Some("basic") => {
                      let basic_content = parsed_json
                          .get("content")
                          .and_then(|c| c.as_str())
                          .ok_or_else(|| FetchIaResponseError::InvalidApiResponse)?;
                      Ok(IaResponse::Basic {
                          content: basic_content.to_string(),
                      })
                  }
                  _ => Err(FetchIaResponseError::InvalidApiResponse),
              }
          } else {
              Err(FetchIaResponseError::InvalidApiResponse)
          }
      } else {
          Err(FetchIaResponseError::InvalidApiResponse)
      }
  } else {
      Err(FetchIaResponseError::InvalidApiResponse)
  }
}




// Commande pour exécuter la commande shell (comme précédemment)
use std::process::Command;

#[tauri::command]
fn execute_command(command: String) -> Result<String, String> {
  // Utilisation de `cmd` pour exécuter la commande sous Windows
  println!("Executing command: {}", command);
  Command::new("cmd")
      .args(&["/C", &command]) // "/C" permet d'exécuter une commande unique
      .output()
      .map(|output| {
          // Convertir la sortie en chaîne de caractères
          String::from_utf8_lossy(&output.stdout).to_string()
      })
      .map_err(|err| {
          // Gérer les erreurs et les convertir en chaîne de caractères
          println!("Erreur lors de l'exécution de la commande : {}", err);
          format!("Erreur lors de l'exécution de la commande : {}", err)
      })
}

async fn execute_api_request(
  api_name: &str,
  endpoint_name: &str,
  params: Option<Value>, // Paramètres pour GET ou POST
) -> Result<Response, String> {
  let api_config = load_api_config(api_name)?;
  let endpoint = api_config
      .endpoints
      .get(endpoint_name)
      .ok_or_else(|| format!("Endpoint '{}' introuvable pour l'API '{}'", endpoint_name, api_name))?;

  let client = reqwest::Client::new();
  let url = format!("{}{}", api_config.base_url, endpoint.path);

  let mut request_builder = match endpoint.method.as_str() {
      "GET" => client.get(&url),
      "POST" => client.post(&url),
      "PUT" => client.put(&url),
      "DELETE" => client.delete(&url),
      _ => return Err(format!("Méthode HTTP inconnue : {}", endpoint.method)),
  };

  if let Some(token) = Some(&api_config.token) {
      request_builder = request_builder.header("Authorization", format!("Bearer {}", token));
  }

  if let Some(params) = params {
      match endpoint.method.as_str() {
          "GET" => request_builder = request_builder.query(&params),
          _ => request_builder = request_builder.json(&params),
      }
  }

  let response = request_builder.send().await;

  match response {
      Ok(res) if res.status().is_success() => Ok(res),
      Ok(res) => Err(format!(
          "Erreur HTTP {} : {}",
          res.status(),
          res.text().await.unwrap_or_else(|_| "Impossible de récupérer le message d'erreur.".to_string())
      )),
      Err(e) => Err(format!("Erreur lors de la requête : {}", e)),
  }
}

fn load_api_config(api_name: &str) -> Result<ApiConfig, String> {
  let file = std::fs::File::open("src/api_config.json").map_err(|_| "Impossible de lire api_config.json".to_string())?;
  let configs: HashMap<String, ApiConfig> = serde_json::from_reader(file).map_err(|_| "Format de api_config.json invalide".to_string())?;

  configs
      .get(api_name)
      .cloned()
      .ok_or_else(|| format!("Aucune configuration trouvée pour l'API : {}", api_name))
}

