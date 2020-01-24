import React from 'react';
import { LandingPage } from '../landing';
import { StartLoginForm } from './start';

/**
 * Component to represent the page that is used for logging in
 */
export const LoginPage: React.FC = () => {
  return (
    <LandingPage>
      <StartLoginForm />
    </LandingPage>
  );
}
