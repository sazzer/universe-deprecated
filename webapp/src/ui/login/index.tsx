import React, { useState } from "react";

import { AuthenticateUserPage } from "./authenticate";
import { LandingPage } from "../landingPage";
import { RegisterUserPage } from "./register";
import { StartLoginPage } from "./start";

/** The possible states the page can be in */
type PageState = "AUTHENTICATE" | "REGISTER" | null;

/**
 * Page controlling login and registration of users
 */
export const LoginPage: React.FC = () => {
  const [pageState, setPageState] = useState<PageState>(null);
  const [username, setUsername] = useState<string>("");

  let page: React.ReactElement = <></>;

  switch (pageState) {
    case null:
      page = (
        <StartLoginPage
          onUsername={(username, known) => {
            setUsername(username);
            setPageState(known ? "AUTHENTICATE" : "REGISTER");
          }}
        />
      );
      break;
    case "AUTHENTICATE":
      page = (
        <AuthenticateUserPage
          username={username}
          onCancel={() => {
            setPageState(null);
          }}
        />
      );
      break;
    case "REGISTER":
      page = (
        <RegisterUserPage
          username={username}
          onCancel={() => {
            setPageState(null);
          }}
        />
      );
      break;
  }

  return <LandingPage>{page}</LandingPage>;
};
