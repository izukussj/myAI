import PropTypes from "prop-types";
import CmdResponse from "./CmdResponse";
import ApiResponse from "./ApiResponse";
import BasicResponse from "./BasicResponse";

const Result = ({ type, content, apiName, isDeveloperMode }) => {
  // Gestion des types de réponses
  switch (type) {
    case "windows":
      return (
        <CmdResponse
          dangerLevel={content.dangerLevel}
          description={content.description}
          shellCode={content.code}
          isDeveloperMode={isDeveloperMode}
        />
      );

    case "api":
      return (
        <ApiResponse
          apiName={apiName}
          response={content.response}
        />
      );

    case "basic":
      return <BasicResponse content={content} />;

    default:
      return <p>Type de réponse inconnu.</p>;
  }
};
Result.propTypes = {
  type: PropTypes.string.isRequired,
  content: PropTypes.shape({
    dangerLevel: PropTypes.number,
    description: PropTypes.string,
    code: PropTypes.string,
    response: PropTypes.string,
  }).isRequired,
  apiName: PropTypes.string,
  isDeveloperMode: PropTypes.bool,
};

export default Result;

