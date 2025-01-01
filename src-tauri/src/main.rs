#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::{generate_context, Builder};

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
pub struct ShellCodeResult {
  pub script: String,
  pub description: String,
  pub danger_level: u8,
}

#[tokio::main]
async fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![fetch_shell_code, execute_command])
      .run(generate_context!())
      .expect("error while running Tauri application");
}

// Commande Tauri pour appeler l'API ChatGPT et récupérer le script shell
#[tauri::command]
async fn fetch_shell_code(query: String) -> Result<ShellCodeResult, String> {
  let  api_key="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";
  // Remplacez par votre propre clé API OpenAI
  let client = Client::new();
  
  // Requête POST à l'API OpenAI
  let response = client
      .post("https://api.openai.com/v1/chat/completions")
      .header("Authorization", format!("Bearer {}", api_key))
      .json(&serde_json::json!({
          "model": "gpt-4", // Utilisez le modèle que vous préférez
          "messages": [
              { "role": "system", "content": "Vous êtes un assistant qui traduit des requêtes en scripts cmd et fournit des informations utiles dans un format structuré. Je peut etre dans un directory au hasard reviens toujours a la racine" },
              { "role": "user", "content": format!(
                  "Transcris cette action en script cmd dans un format JSON avec trois champs :\n\
                  - \"script\" : le script cmd correspondant.\n\
                  - \"description\" : une brève description de ce que fait le script.\n\
                  - \"danger_level\" : le niveau de dangerosité sous forme d'entier (0: faible, 1: moyenne, 2: élevée).\n\
                  Action : \"{}\"",
                  query
              ) }
          ]
      }))
      .send()
      .await;

  match response {
      Ok(res) => {
          if res.status().is_success() {
              let chat_response: ChatGPTResponse = res.json().await.map_err(|e| e.to_string())?;
              if let Some(choice) = chat_response.choices.get(0) {
                  let content = &choice.message.content;
                  // Tente de parser la réponse en JSON
                  let result: Result<ShellCodeResult, serde_json::Error> = serde_json::from_str(content);
                  match result {
                      Ok(parsed) => Ok(parsed),
                      Err(_) => Err("Réponse de l'API mal formatée.".to_string()),
                  }
              } else {
                  Err("Aucune réponse de l'API OpenAI.".to_string())
              }
          } else {
              Err("Échec de la réponse de l'API OpenAI.".to_string())
          }
      }
      Err(e) => Err(format!("Erreur lors de la requête à l'API : {}", e)),
  }
}

// Commande pour exécuter la commande shell (comme précédemment)
use std::process::Command;

#[tauri::command]
fn execute_command(command: String) -> Result<String, String> {
  // Utilisation de `cmd` pour exécuter la commande sous Windows
  Command::new("cmd")
      .args(&["/C", &command]) // "/C" permet d'exécuter une commande unique
      .output()
      .map(|output| {
          // Convertir la sortie en chaîne de caractères
          String::from_utf8_lossy(&output.stdout).to_string()
      })
      .map_err(|err| {
          // Gérer les erreurs et les convertir en chaîne de caractères
          format!("Erreur lors de l'exécution de la commande : {}", err)
      })
}