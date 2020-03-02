import { Route, BrowserRouter as Router, Switch } from "react-router-dom";

import { HeaderBar } from "./header";
import { HomePage } from "./homePage";
import { LoginPage } from "./login";
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

        <div className="container-fluid mt-3">
          <Switch>
            <Route path="/login">
              <LoginPage />
            </Route>
            <Route>
              <HomePage />
            </Route>
          </Switch>
        </div>
      </Router>
    </UserProvider>
  );
};
