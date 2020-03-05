import { UserContext, UserContextState } from "../users";

import { LoggedIn } from "./loggedIn";
import React from "react";
import { render } from "@testing-library/react";

describe("loggedIn", () => {
  test("When not logged in", () => {
    const userState: UserContextState = {
      hasUser: false,
      user: null,
      clearUser: () => {},
      storeUser: () => {}
    };

    const { container } = render(
      <UserContext.Provider value={userState}>
        <LoggedIn>Contents</LoggedIn>
      </UserContext.Provider>
    );
    expect(container).toMatchSnapshot();
  });

  test("When logged in", () => {
    const userState: UserContextState = {
      hasUser: true,
      user: null,
      clearUser: () => {},
      storeUser: () => {}
    };

    const { container } = render(
      <UserContext.Provider value={userState}>
        <LoggedIn>Contents</LoggedIn>
      </UserContext.Provider>
    );
    expect(container).toMatchSnapshot();
  });
});
