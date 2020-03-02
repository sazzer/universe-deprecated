import React, { useState } from "react";

import { LandingPage } from "../landingPage";
import { StartLogin } from "./start";

/** The possible states the page can be in */
type PageState = "AUTHENTICATE" | "REGISTER" | null;

/**
 * Page controlling login and registration of users
 */
export const LoginPage: React.FC = () => {
  const [pageState, setPageState] = useState<PageState>(null);

  let page: React.ReactElement = <></>;

  switch (pageState) {
    case null:
      page = (
        <StartLogin
          onUsername={(username, known) => {
            setPageState(known ? "AUTHENTICATE" : "REGISTER");
          }}
        />
      );
      break;
    case "AUTHENTICATE":
      page = <>Authenticate</>;
      break;
    case "REGISTER":
      page = <>Register</>;
      break;
  }

  return <LandingPage>{page}</LandingPage>;
};
