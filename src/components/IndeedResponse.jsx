
import PropTypes from "prop-types";

const IndeedResponse = ({ jobList }) => {
  return (
    <div className="relative flex flex-col overflow-hidden rounded-lg shadow-lg bg-white">
      <div className="px-6 py-8 sm:px-10 sm:py-12">
        <h3 className="text-xl font-semibold text-gray-900">Offres d&apos;emploi Indeed</h3>
        <ul className="mt-4 space-y-4">
          {jobList.map((job, index) => (
            <li key={index} className="p-4 border border-gray-200 rounded-md">
              <h4 className="text-lg font-semibold">{job.title}</h4>
              <p className="text-sm text-gray-600">{job.company}</p>
              <p className="mt-2 text-sm text-gray-800">{job.description}</p>
              <a
                href={job.url}
                className="mt-4 inline-block text-blue-600 hover:text-blue-500"
              >
                Voir l&apos;offre
              </a>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};
IndeedResponse.propTypes = {
  jobList: PropTypes.arrayOf(
    PropTypes.shape({
      title: PropTypes.string.isRequired,
      company: PropTypes.string.isRequired,
      description: PropTypes.string.isRequired,
      url: PropTypes.string.isRequired,
    })
  ).isRequired,
};

export default IndeedResponse;

