import React, { useState } from 'react';
import { LandingPage } from '../landing';
import { StartLoginForm } from './start';
import { request, ProblemResponse } from '../../api';

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

  let resolveUsername = async (username: string) => {
    try {
      await request({
        url: '/usernames/{username}',
        urlParams: {
          username
        },
        method: 'GET',
      });

      setUsernameState({ username, known: true });
    } catch (e) {
      if (e instanceof ProblemResponse) {
        if (e.problem.type === 'tag:universe,2020:users/problems/unknown-user') {
          setUsernameState({ username, known: false });
        }
      }
    }
  };

  let body;
  if (usernameState === undefined) {
    body = <StartLoginForm onSubmit={resolveUsername} />;
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
