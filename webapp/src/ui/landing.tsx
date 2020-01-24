import React from 'react';

/**
 * Wrapper around landing page style pages, with an image and a sidebar
 * @param children The child elements to render in the sidebar
 */
export const LandingPage: React.FC = ({ children }) => {
  return (
    <div className="row">
      <div className="col-12 col-md-4 order-sm-1">
        {children}
      </div>
      <div className="col-12 col-md-8">
        <h3>The Continent of Khorvaire</h3>
        <img src="/landing.jpg" width="100%" alt="The Continent of Khorvaire" />
      </div>
    </div>
  );
}
