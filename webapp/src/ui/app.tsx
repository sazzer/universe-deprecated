import React from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import { Header } from "./header";
import { HomePage } from "./homePage";
import { LoginPage } from "./login";
import { ProfilePage } from "./profile";

/**
 * The main application
 */
export const App: React.FC = () => {
  return (
    <Router>
      <Header />

      <div className="container-fluid mt-3">
        <Switch>
          <Route path="/login" component={LoginPage} />
          <Route path="/profile" component={ProfilePage} />
          <Route path="/" component={HomePage} />
        </Switch>
      </div>
    </Router>
  );
};
