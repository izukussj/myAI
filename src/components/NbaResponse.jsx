import PropTypes from "prop-types";

/**
 * @param {{ chart_url: string }} props
 */
const NbaResponse = ({ chart_url }) => {
  return (
    <div className="relative flex flex-col overflow-hidden rounded-lg shadow-lg bg-white">
      <div className="px-6 py-8 sm:px-10 sm:py-12">
        <h3 className="text-xl font-semibold text-gray-900">Graphique NBA</h3>
        {chart_url ? (
          <img
            src={chart_url}
            alt="Graphique NBA"
            className="mt-4 w-full h-auto rounded-md shadow"
          />
        ) : (
          <p className="mt-4 text-gray-700">Aucun graphique disponible.</p>
        )}
      </div>
    </div>
  );
};

NbaResponse.propTypes = {
  chart_url: PropTypes.string.isRequired,
};

export default NbaResponse;
