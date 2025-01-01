import React from "react";

class ResultComponent extends React.Component {
  render() {
    const { type, content } = this.props;
    switch (type) {
      case "windows":
        return <CmdResponse {...content} />;
      case "api":
        switch (this.props.apiName) {
          case "Indeed Scraper":
            return <IndeedResponse {...content} />;
          default:
            return <div>API non prise en charge.</div>;
        }
      case "basic":
        return <BasicResponse {...content} />;
      default:
        return <div>Type de réponse inconnu.</div>;
    }
  }
}

export default ResultComponent;
