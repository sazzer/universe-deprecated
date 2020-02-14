import React, { useEffect } from "react";
import { LandingPage } from "../landing";
import { StartLoginForm } from "./start";
import { RegisterForm } from "./register";
import { AuthenticateForm } from "./authenticate";
import { useOvermind } from "../../overmind";

/**
 * Component to represent the page that is used for logging in
 */
export const LoginPage: React.FC = () => {
  const { state, actions } = useOvermind();

  useEffect(() => {
    actions.login.resetLogin();
  }, [actions.login]);

  let body;

  if (state.login.mode.current === "registering") {
    body = <RegisterForm />;
  } else if (state.login.mode.current === "authenticating") {
    body = <AuthenticateForm />;
  } else {
    body = <StartLoginForm />;
  }

  return <LandingPage>{body}</LandingPage>;
};
