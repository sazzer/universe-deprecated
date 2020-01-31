import React, { useState } from 'react';
import { LandingPage } from '../landing';
import { StartLoginForm } from './start';

/** The shape of the state for the username */
interface UsernameState {
  username: string,
  known: boolean,
}

/**
 * Component to represent the page that is used for logging in
 */
export const LoginPage: React.FC = () => {
  let [usernameState, setUsernameState] = useState<UsernameState | undefined>();

  let body;
  if (usernameState === undefined) {
    body = <StartLoginForm onSubmit={(username) => {
      setUsernameState({ username, known: true });
    }} />;
  } else if (usernameState.known) {
    body = "Log in as: " + usernameState.username;
  } else {
    body = "Register as: " + usernameState.username;
  }

  return (
    <LandingPage>
      {body}
    </LandingPage>
  );
}
