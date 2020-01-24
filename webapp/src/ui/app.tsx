import React from 'react';
import { BrowserRouter as Router, Switch, Route } from 'react-router-dom';
import { Header } from './header';
import { HomePage } from './homePage';

export const App: React.FC = () => {
  return (
    <Router>
      <Header />

      <div className="container-fluid mt-3">
        <Switch>
          <Route path='/login'>Login Page</Route>
          <Route path='/' component={HomePage} />
        </Switch>
      </div>
    </Router>
  );
}
