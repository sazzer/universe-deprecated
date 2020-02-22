import React from "react";
import { render, fireEvent, wait } from "@testing-library/react";
import { MemoryRouter as Router } from "react-router-dom";
import { Header } from "./header";
import { createTestOvermind } from "../../overmind/test";
import { Provider } from "overmind-react";
import { OvermindMock, Config } from "overmind";

describe("When not logged in", () => {
  test("It renders correctly", () => {
    const overmind = createTestOvermind();
    const { container } = render(
      <Router>
        <Provider value={overmind}>
          <Header />
        </Provider>
      </Router>
    );
    expect(container).toMatchSnapshot();
  });
});

describe("When logged in", () => {
  let overmind: OvermindMock<Config>;

  beforeEach(() => {
    overmind = createTestOvermind(
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
  });

  test("It renders correctly", () => {
    const { container } = render(
      <Router>
        <Provider value={overmind}>
          <Header />
        </Provider>
      </Router>
    );
    expect(container).toMatchSnapshot();
  });
});
