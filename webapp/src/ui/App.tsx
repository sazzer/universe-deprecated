import { HeaderBar } from "./header";
import React from "react";
import { BrowserRouter as Router } from "react-router-dom";

/**
 * The main entrypoint into the application
 */
export const App: React.FC = () => {
  return (
    <Router>
      <HeaderBar />
    </Router>
  );
};
