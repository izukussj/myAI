import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Result from "./components/Result";
function App() {
  const [userInput, setUserInput] = useState("");
  const [result, setResult] = useState(null);
  const [isExecuting, setIsExecuting] = useState(false);
  const [isDeveloperMode, setIsDeveloperMode] = useState(false);

  const fetchIaResponse = async () => {
    if (!userInput.trim()) {
      alert("Veuillez entrer une commande.");
      return;
    }
  
    setIsExecuting(true);
  
    try {
      // Appel de la commande Tauri pour récupérer la réponse IA
      const result = await invoke("fetch_ia_response", { query: userInput });
      console.log("Résultat brut :", result);
      // Traitement des résultats en fonction du type de réponse
      switch (result.type) {
        case "windows":
          if (result.content) {
            setResult({
              type: "windows",
              content: {
                dangerLevel: result.content.dangerLevel ?? 0,
                description: result.content.description ?? "Aucune description disponible.",
                code: result.content.code ?? "Aucun code fourni.",
              },
            });
          } else {
            alert("Le contenu de la réponse 'windows' est manquant ou invalide.");
            console.error("Détails du résultat : ", result);
          }
          break;
  
        case "api":
          if (result.apiName && result.content) {
            setResult({
              type: "api",
              apiName: result.apiName,
              content: {
                response: result.content.response ?? "Aucune réponse API disponible.",
              },
            });
          } else {
            alert("Les données de la réponse 'api' sont incomplètes ou invalides.");
            console.error("Détails du résultat : ", result);
          }
          break;
  
        case "basic":
          if (result.content) {
            setResult({
              type: "basic",
              content: result.content ?? "Aucun contenu fourni.",
            });
          } else {
            alert("Le contenu de la réponse 'basic' est manquant ou invalide.");
            console.error("Détails du résultat : ", result);
          }
          break;
  
        default:
          alert("Type de réponse inconnu.");
          console.error("Réponse inattendue :", result);
      }
    } catch (error) {
      // Gestion des erreurs spécifiques renvoyées par le backend
      if (error instanceof Error && error.message) {
        switch (true) {
          case error.message.includes("ContextLoadError"):
            alert("Erreur lors du chargement du contexte : " + error.message);
            break;
          case error.message.includes("ApiRequestError"):
            alert("Erreur lors de la requête API : " + error.message);
            break;
          case error.message.includes("ParsingError"):
            alert("Erreur de parsing : " + error.message);
            break;
          case error.message.includes("InvalidApiResponse"):
            alert("Réponse de l'API invalide.");
            break;
          default:
            alert("Erreur inconnue : " + error.message);
            console.error("Erreur inconnue :", error);
        }
      } else {
        // Gestion d'autres erreurs inattendues
        alert("Erreur inattendue lors de la communication avec le backend.");
        console.error(error);
      }
    } finally {
      setIsExecuting(false);
    }
  };
  
  

  return (
    <div className="bg-gray-50 py-24 sm:py-32">
      <div className="mx-auto max-w-2xl px-6 lg:max-w-7xl lg:px-8">
        {/* Titre et introduction */}
        <h2 className="text-center text-base font-semibold text-indigo-600">Votre Assistant myAI</h2>
        <p className="mx-auto mt-2 max-w-lg text-4xl font-semibold text-center text-gray-950 sm:text-5xl">
          Bonjour Seydina!
        </p>

        <div className="mt-10 grid gap-8 lg:grid-cols-2">
          {/* Première carte : Entrée de commande */}
          <div className="relative flex flex-col overflow-hidden rounded-lg shadow-lg bg-white">
            <div className="px-6 py-8 sm:px-10 sm:py-12">
              <textarea
                className="w-full p-3 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                rows="4"
                placeholder="Entrez votre commande ici..."
                value={userInput}
                onChange={(e) => setUserInput(e.target.value)}
              />
              <button
                type="button"
                className="mt-6 inline-flex items-center rounded-full bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                onClick={fetchIaResponse}
                disabled={isExecuting}
              >
                {isExecuting ? "Chargement..." : "Obtenir le code shell"}
              </button>
              <div className="mt-4 flex items-center">
                <input
                  id="developerMode"
                  type="checkbox"
                  className="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                  checked={isDeveloperMode}
                  onChange={() => setIsDeveloperMode(!isDeveloperMode)}
                />
                <label
                  htmlFor="developerMode"
                  className="ml-2 block text-sm text-gray-900"
                >
                  Mode développeur
                </label>
              </div>
            </div>
          </div>

          {/* Deuxième carte : Résultats */}
          {result && <Result {...result} isDeveloperMode={isDeveloperMode} />}

        </div>
      </div>
    </div>
  );
}

export default App;
