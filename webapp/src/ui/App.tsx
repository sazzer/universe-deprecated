import { Route, BrowserRouter as Router, Switch } from "react-router-dom";

import { HeaderBar } from "./header";
import { HomePage } from "./homePage";
import React from "react";

/**
 * The main entrypoint into the application
 */
export const App: React.FC = () => {
  return (
    <Router>
      <HeaderBar />

      <Switch>
        <Route>
          <HomePage />
        </Route>
      </Switch>
    </Router>
  );
};
