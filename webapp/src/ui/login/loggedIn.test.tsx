import React from "react";
import { render } from "@testing-library/react";
import { LoggedIn } from "./loggedIn";
import { createTestOvermind } from "../../overmind/test";
import { Provider } from "overmind-react";
import { OvermindMock, Config } from "overmind";

describe("When not logged in", () => {
  test("It renders correctly", () => {
    const overmind = createTestOvermind();
    const { container } = render(
      <Provider value={overmind}>
        <LoggedIn>Hello</LoggedIn>
      </Provider>
    );
    expect(container).toMatchSnapshot();
  });
});

describe("When logged in", () => {
  test("It renders correctly", () => {
    const overmind = createTestOvermind(
      {},
      {
        users: {
          users: {
            someUserId: {
              userId: "someUserId",
              username: "username",
              email: "test@example.com",
              displayName: "Test User"
            }
          }
        },
        authentication: {
          userId: "someUserId",
          accessToken: {
            accessToken: "accessToken",
            expires: new Date().toISOString()
          },
          isLoggedIn: true
        }
      }
    );
    const { container } = render(
      <Provider value={overmind}>
        <LoggedIn>Hello</LoggedIn>
      </Provider>
    );
    expect(container).toMatchSnapshot();
  });
});
