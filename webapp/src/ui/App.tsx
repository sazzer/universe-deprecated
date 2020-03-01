import { Route, BrowserRouter as Router, Switch } from "react-router-dom";

import { HeaderBar } from "./header";
import { HomePage } from "./homePage";
import React from "react";
import { UserProvider } from "../users";

/**
 * The main entrypoint into the application
 */
export const App: React.FC = () => {
  return (
    <UserProvider>
      <Router>
        <HeaderBar />

        <Switch>
          <Route>
            <HomePage />
          </Route>
        </Switch>
      </Router>
    </UserProvider>
  );
};
