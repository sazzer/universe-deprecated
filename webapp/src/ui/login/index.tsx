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
      page = <StartLogin />;
      break;
    case "AUTHENTICATE":
    case "REGISTER":
      page = <></>;
  }

  return <LandingPage>{page}</LandingPage>;
};
