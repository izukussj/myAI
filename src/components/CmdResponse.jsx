
import { invoke } from "@tauri-apps/api/core";
import PropTypes from 'prop-types';
const executeCommand = async (code) => {
  if (!code.trim()) {
    alert("Aucun code shell à exécuter.");
    return;
  }

  if (window.confirm("Êtes-vous sûr de vouloir exécuter cette commande ?")) {
    try {
      const output = await invoke("execute_command", { command: code });
      alert(`Commande exécutée avec succès :\n${output}`);
    } catch (error) {
      alert(`Erreur lors de l'exécution :\n${error}`);
    }
  }
};
const CmdResponse = ({ shellCode, description, dangerLevel, isDeveloperMode }) => {
  return (
    <div className="relative flex flex-col overflow-hidden rounded-lg shadow-lg bg-white">
      <div className="px-6 py-8 sm:px-10 sm:py-12">
        {/* Code Shell */}
        {isDeveloperMode && shellCode && (
          <div>
            <div className="flex items-center">
              <svg
                className="h-6 w-6 text-indigo-500 mr-3"
                fill="currentColor"
                viewBox="0 0 20 20"
                aria-hidden="true"
              >
                <path d="M5.05 4.293a1 1 0 011.414 0L10 7.828l3.536-3.535a1 1 0 011.414 1.414L10.828 10l3.535 3.536a1 1 0 01-1.414 1.414L10 12.172l-3.536 3.535a1 1 0 01-1.414-1.414L7.172 10 3.636 6.464a1 1 0 010-1.414z" />
              </svg>
              <h3 className="text-xl font-semibold text-gray-900">Script Shell</h3>
            </div>
            <pre className="mt-2 p-4 bg-gray-200 rounded-md text-sm font-mono">
              {shellCode}
            </pre>
          </div>
        )}

        {/* Description */}
        <div className="mt-6 flex items-center">
          <svg
            className="h-6 w-6 text-green-500 mr-3"
            fill="currentColor"
            viewBox="0 0 20 20"
            aria-hidden="true"
          >
            <path d="M13.828 9.172a4 4 0 010 5.656L10 18.656 6.172 14.828a4 4 0 015.656-5.656z" />
          </svg>
          <h3 className="text-xl font-semibold text-gray-900">Description</h3>
        </div>
        <p className="mt-2">{description}</p>

        {/* Danger Level */}
        <div className="mt-6 flex items-center">
          <svg
            className="h-6 w-6 text-yellow-500 mr-3"
            fill="currentColor"
            viewBox="0 0 20 20"
            aria-hidden="true"
          >
            <path d="M10 2a8 8 0 100 16 8 8 0 000-16zm0 11a1 1 0 110 2 1 1 0 010-2zM9 7a1 1 0 012 0v3a1 1 0 01-2 0V7z" />
          </svg>
          <h3 className="text-xl font-semibold text-gray-900">
            Niveau de Dangerosité
          </h3>
        </div>
        <span
          className={`inline-flex items-center rounded-md px-2 py-1 text-xs font-medium ring-1 ring-inset ${
            dangerLevel === 0
              ? "bg-green-50 text-green-700 ring-green-600/20"
              : dangerLevel === 1
              ? "bg-yellow-50 text-yellow-800 ring-yellow-600/20"
              : "bg-red-50 text-red-800 ring-red-600/20"
          }`}
        >
          {dangerLevel === "0"
            ? "Faible"
            : dangerLevel === "1"
            ? "Moyenne"
            : "Élevée"}
        </span> 
        <br />      
                <button
                  type="button"
                  className="mt-6 inline-flex items-center rounded-md bg-green-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-green-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-green-600"
                  onClick={executeCommand.bind(null, shellCode)}
                >
                  Exécuter la commande
                </button>
      </div>
    </div>
  );
};

CmdResponse.propTypes = {
  shellCode: PropTypes.string,
  description: PropTypes.string.isRequired,
  dangerLevel: PropTypes.number.isRequired,
  isDeveloperMode: PropTypes.bool.isRequired,
};

export default CmdResponse;
