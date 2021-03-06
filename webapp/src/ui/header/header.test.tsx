import { LoginFailure, authenticate, useUser } from "../../users";

import { HeaderBar } from "./index";
import React from "react";
import { MemoryRouter as Router } from "react-router-dom";
import { render } from "@testing-library/react";

jest.mock("../../users");

const useUserMock = useUser as jest.Mock;

afterEach(() => {
  jest.resetAllMocks();
});

test("When not logged in", () => {
  useUserMock.mockReturnValue({
    hasUser: false,
    user: null,
    clearUser: () => {},
    storeUser: () => {}
  });

  const { container } = render(
    <Router>
      <HeaderBar />
    </Router>
  );
  expect(container).toMatchSnapshot();
});

test("When logged in", () => {
  useUserMock.mockReturnValue({
    hasUser: true,
    user: {
      id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
      email: "testuser@example.com",
      username: "testuser",
      displayName: "Test User"
    },
    clearUser: () => {},
    storeUser: () => {}
  });

  const { container } = render(
    <Router>
      <HeaderBar />
    </Router>
  );
  expect(container).toMatchSnapshot();
});
