import React from "react";
import { render, wait, fireEvent } from "@testing-library/react";
import { Provider } from "overmind-react";
import { RegisterForm } from "./register";
import { createTestOvermind } from "../../overmind/test";
import { OvermindMock, Config } from "overmind";

describe("RegisterForm", () => {
  test("Initial render", () => {
    const overmind = createTestOvermind(
      {},
      {
        login: {
          mode: {
            current: "registering"
          },
          username: "testuser"
        }
      }
    );
    const { container } = render(
      <Provider value={overmind}>
        <RegisterForm />
      </Provider>
    );
    expect(container).toMatchSnapshot();
  });
  test("Cancelling Registration", async () => {
    const overmind = createTestOvermind(
      {},
      {
        login: {
          mode: {
            current: "registering"
          },
          username: "testuser"
        }
      }
    );
    const { getByText } = render(
      <Provider value={overmind}>
        <RegisterForm />
      </Provider>
    );
    await wait(() =>
      fireEvent.click(getByText("Cancel", { selector: "button" }))
    );
    expect(overmind.mutations).toMatchSnapshot();
  });
  describe("Submitting the form", () => {
    let overmind: OvermindMock<Config>;

    beforeEach(() => {
      overmind = createTestOvermind(
        {},
        {
          login: {
            mode: {
              current: "registering"
            },
            username: "testuser"
          }
        }
      );
    });
    test("Submit without entering anything", async () => {
      const { container, getByText } = render(
        <Provider value={overmind}>
          <RegisterForm />
        </Provider>
      );
      await wait(() =>
        fireEvent.click(getByText("Register", { selector: "button" }))
      );
      expect(container).toMatchSnapshot();
    });
    test("Submit with whitespace for everything", async () => {
      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <RegisterForm />
        </Provider>
      );
      await wait(() => {
        fireEvent.change(getByLabelText("Email Address"), {
          target: { value: "   " }
        });
        fireEvent.change(getByLabelText("Display Name"), {
          target: { value: "   " }
        });
        fireEvent.change(getByLabelText("Password"), {
          target: { value: "   " }
        });
        fireEvent.change(getByLabelText("Repeat Password"), {
          target: { value: "   " }
        });
        fireEvent.click(getByText("Register", { selector: "button" }));
      });
      expect(container).toMatchSnapshot();
    });
    test("Submit with invalid email address", async () => {
      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <RegisterForm />
        </Provider>
      );
      await wait(() => {
        fireEvent.change(getByLabelText("Email Address"), {
          target: { value: "testuser" }
        });
        fireEvent.change(getByLabelText("Display Name"), {
          target: { value: "Test User" }
        });
        fireEvent.change(getByLabelText("Password"), {
          target: { value: "Pa55word" }
        });
        fireEvent.change(getByLabelText("Repeat Password"), {
          target: { value: "Pa55word" }
        });
        fireEvent.click(getByText("Register", { selector: "button" }));
      });
      expect(container).toMatchSnapshot();
    });
    test("Submit with mismatched passwords", async () => {
      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <RegisterForm />
        </Provider>
      );
      await wait(() => {
        fireEvent.change(getByLabelText("Email Address"), {
          target: { value: "testuser@example.com" }
        });
        fireEvent.change(getByLabelText("Display Name"), {
          target: { value: "Test User" }
        });
        fireEvent.change(getByLabelText("Password"), {
          target: { value: "Pa55word" }
        });
        fireEvent.change(getByLabelText("Repeat Password"), {
          target: { value: "password" }
        });
        fireEvent.click(getByText("Register", { selector: "button" }));
      });
      expect(container).toMatchSnapshot();
    });
  });
});
