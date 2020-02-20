import React from "react";
import { render, wait, fireEvent } from "@testing-library/react";
import { Provider } from "overmind-react";
import { AuthenticateForm } from "./authenticate";
import { createTestOvermind } from "../../overmind/test";
import { OvermindMock, Config } from "overmind";
import { ValidationErrors } from "../../api/validation";
import { AuthenticationError } from "../../login/effects";

describe("AuthenticateForm", () => {
  test("Initial render", () => {
    const overmind = createTestOvermind(
      {},
      {
        login: {
          mode: {
            current: "authenticating"
          },
          username: "testuser"
        }
      }
    );
    const { container } = render(
      <Provider value={overmind}>
        <AuthenticateForm />
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
            current: "authenticating"
          },
          username: "testuser"
        }
      }
    );
    const { getByText } = render(
      <Provider value={overmind}>
        <AuthenticateForm />
      </Provider>
    );
    await wait(() =>
      fireEvent.click(getByText("Cancel", { selector: "button" }))
    );
    expect(overmind.mutations).toMatchSnapshot();
  });
  describe("Submitting the form", () => {
    let authenticateEffect: jest.Mock;
    let overmind: OvermindMock<Config>;

    beforeEach(() => {
      authenticateEffect = jest.fn();

      overmind = createTestOvermind(
        {
          login: {
            api: {
              authenticateUser: authenticateEffect
            }
          }
        },
        {
          login: {
            mode: {
              current: "authenticating"
            },
            username: "testuser"
          }
        }
      );
    });
    test("Submit without entering anything", async () => {
      const { container, getByText } = render(
        <Provider value={overmind}>
          <AuthenticateForm />
        </Provider>
      );
      await wait(() =>
        fireEvent.click(getByText("Login", { selector: "button" }))
      );
      expect(container).toMatchSnapshot();
      expect(authenticateEffect).toBeCalledTimes(0);
    });
    test("Submit with whitespace password", async () => {
      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <AuthenticateForm />
        </Provider>
      );
      await wait(() => {
        fireEvent.change(getByLabelText("Password"), {
          target: { value: "   " }
        });
        fireEvent.click(getByText("Login", { selector: "button" }));
      });
      expect(container).toMatchSnapshot();
      expect(authenticateEffect).toBeCalledTimes(1);
      expect(authenticateEffect).toBeCalledWith("testuser", "   ");
    });

    test("Submit successfully", async () => {
      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <AuthenticateForm />
        </Provider>
      );
      authenticateEffect.mockResolvedValueOnce(undefined);

      await wait(() => {
        fireEvent.change(getByLabelText("Password"), {
          target: { value: "Pa55word" }
        });
        fireEvent.click(getByText("Login", { selector: "button" }));
      });
      expect(container).toMatchSnapshot();
      expect(authenticateEffect).toBeCalledTimes(1);
      expect(authenticateEffect).toBeCalledWith("testuser", "Pa55word");
    });

    test("Submit with network error", async () => {
      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <AuthenticateForm />
        </Provider>
      );
      authenticateEffect.mockRejectedValueOnce(new Error("Network Error"));

      await wait(() => {
        fireEvent.change(getByLabelText("Password"), {
          target: { value: "Pa55word" }
        });
        fireEvent.click(getByText("Login", { selector: "button" }));
      });
      expect(container).toMatchSnapshot();
      expect(authenticateEffect).toBeCalledTimes(1);
      expect(authenticateEffect).toBeCalledWith("testuser", "Pa55word");
    });

    test("Submit with invalid password", async () => {
      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <AuthenticateForm />
        </Provider>
      );
      authenticateEffect.mockRejectedValueOnce(new AuthenticationError());

      await wait(() => {
        fireEvent.change(getByLabelText("Password"), {
          target: { value: "Pa55word" }
        });
        fireEvent.click(getByText("Login", { selector: "button" }));
      });
      expect(container).toMatchSnapshot();
      expect(authenticateEffect).toBeCalledTimes(1);
      expect(authenticateEffect).toBeCalledWith("testuser", "Pa55word");
    });
  });
});
