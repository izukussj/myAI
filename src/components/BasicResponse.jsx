import PropTypes from "prop-types";

/**
 * @param {{ message: string }} props
 */
const BasicResponse = ({ message }) => {
  return (
    <div className="relative flex flex-col overflow-hidden rounded-lg shadow-lg bg-white">
      <div className="px-6 py-8 sm:px-10 sm:py-12">
        <h3 className="text-xl font-semibold text-gray-900">Réponse de base</h3>
        <p className="mt-4 text-gray-700">{message}</p>
      </div>
    </div>
  );
};
BasicResponse.propTypes = {
  message: PropTypes.string.isRequired,
};

export default BasicResponse;
