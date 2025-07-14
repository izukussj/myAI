import PropTypes from "prop-types";
import NbaResponse from "./NbaResponse"; // Assurez-vous d'importer correctement le composant

const ApiResponse = ({ apiName, response }) => {
  // Aiguillage en fonction de l'API
  switch (apiName) {
    case "nbapi":
      // Accédez à chart_url depuis response
      return <NbaResponse chart_url={response.chart_url} />;
    // Ajouter d'autres cas ici pour d'autres APIs
    default:
      return <p>Réponse non prise en charge pour l&apos;API : {apiName}</p>;
  }
};

ApiResponse.propTypes = {
  apiName: PropTypes.string.isRequired,
  response: PropTypes.object.isRequired,  // 'response' est un objet, pas une chaîne
};

export default ApiResponse;
