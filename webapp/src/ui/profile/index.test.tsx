import { getUserById, useUser } from "../../users";
import { render, wait } from "@testing-library/react";

import { ProfilePage } from "./index";
import React from "react";
import { StaticRouter as Router } from "react-router-dom";

jest.mock("../../users");

const useUserMock = useUser as jest.Mock;
const getUserByIdMock = getUserById as jest.Mock;

afterEach(() => {
  jest.resetAllMocks();
});

test("Rendering when not logged in", () => {
  useUserMock.mockReturnValue({
    hasUser: false,
    user: null,
    clearUser: () => {},
    storeUser: () => {}
  });

  const { container } = render(
    <Router location="/profile">
      <ProfilePage />
    </Router>
  );
  expect(container).toMatchSnapshot();
});

test("Rendering when logged in", async () => {
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

  getUserByIdMock.mockResolvedValueOnce({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "testuser@example.com",
    username: "testuser",
    displayName: "Test User"
  });

  const { container } = render(
    <Router location="/profile">
      <ProfilePage />
    </Router>
  );

  // Waiting on getUserByIdMock
  await wait(() => {});

  expect(container).toMatchSnapshot();
});
