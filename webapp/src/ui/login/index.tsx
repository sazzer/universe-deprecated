import React from 'react';
import { LandingPage } from '../landing';
import { StartLoginForm } from './start';

export const LoginPage: React.FC = () => {
  return (
    <LandingPage>
      <StartLoginForm />
    </LandingPage>
  );
}
