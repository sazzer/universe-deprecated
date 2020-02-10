import React, { useEffect } from 'react';
import { LandingPage } from '../landing';
import { StartLoginForm } from './start';
import { RegisterForm } from './register';
import { useOvermind } from '../../overmind';

/**
 * Component to represent the page that is used for logging in
 */
export const LoginPage: React.FC = () => {
  const { state, actions } = useOvermind();

  useEffect(() => {
    actions.login.resetLogin();
  }, [actions.login]);

  let body;

  if (state.login.isRegistering) {
    body = <RegisterForm />;
  } else if (state.login.isAuthenticating) {
    body = "Log in as: " + state.login.username;
  } else {
    body = <StartLoginForm />;
  }

  return (
    <LandingPage>
      {body}
    </LandingPage>
  );
}
