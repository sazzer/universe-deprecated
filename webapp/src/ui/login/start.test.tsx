import React from "react";
import { render, wait, fireEvent } from "@testing-library/react";
import { Provider } from "overmind-react";
import { StartLoginForm } from "./start";
import { createTestOvermind } from "../../overmind/test";
import { OvermindMock, Config } from "overmind";

describe("StartLoginForm", () => {
  test("Initial render", () => {
    const overmind = createTestOvermind();
    const { container } = render(
      <Provider value={overmind}>
        <StartLoginForm />
      </Provider>
    );
    expect(container).toMatchSnapshot();
  });

  describe("Submitting the form", () => {
    let checkUsernameEffect: jest.Mock;
    let overmind: OvermindMock<Config>;

    beforeEach(() => {
      checkUsernameEffect = jest.fn();

      overmind = createTestOvermind({
        login: {
          api: {
            checkUsername: checkUsernameEffect
          }
        }
      });
    });
    test("Submit without entering anything", async () => {
      const { container, getByText } = render(
        <Provider value={overmind}>
          <StartLoginForm />
        </Provider>
      );
      await wait(() =>
        fireEvent.click(getByText("Login / Register", { selector: "button" }))
      );
      expect(container).toMatchSnapshot();
      expect(checkUsernameEffect).toBeCalledTimes(0);
    });

    test("Submit with a pure whitespace username", async () => {
      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <StartLoginForm />
        </Provider>
      );
      await wait(() => {
        fireEvent.change(getByLabelText("Username"), {
          target: { value: "   " }
        });
        fireEvent.click(getByText("Login / Register", { selector: "button" }));
      });
      expect(container).toMatchSnapshot();
      expect(checkUsernameEffect).toBeCalledTimes(0);
    });
    test("Submit with a valid username - not known", async () => {
      checkUsernameEffect.mockResolvedValueOnce(false);

      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <StartLoginForm />
        </Provider>
      );
      await wait(() => {
        fireEvent.change(getByLabelText("Username"), {
          target: { value: "testuser" }
        });
        fireEvent.click(getByText("Login / Register", { selector: "button" }));
      });
      expect(container).toMatchSnapshot();
      expect(checkUsernameEffect).toBeCalledTimes(1);
      expect(checkUsernameEffect).toBeCalledWith("testuser");
      expect(overmind.mutations).toMatchSnapshot();
    });
    test("Submit with a valid username - is known", async () => {
      checkUsernameEffect.mockResolvedValueOnce(true);

      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <StartLoginForm />
        </Provider>
      );
      await wait(() => {
        fireEvent.change(getByLabelText("Username"), {
          target: { value: "testuser" }
        });
        fireEvent.click(getByText("Login / Register", { selector: "button" }));
      });
      expect(container).toMatchSnapshot();
      expect(checkUsernameEffect).toBeCalledTimes(1);
      expect(checkUsernameEffect).toBeCalledWith("testuser");
      expect(overmind.mutations).toMatchSnapshot();
    });
    test("Submit with a valid username - network error", async () => {
      checkUsernameEffect.mockRejectedValueOnce(new Error("Network Error"));

      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <StartLoginForm />
        </Provider>
      );
      await wait(() => {
        fireEvent.change(getByLabelText("Username"), {
          target: { value: "testuser" }
        });
        fireEvent.click(getByText("Login / Register", { selector: "button" }));
      });
      expect(container).toMatchSnapshot();
      expect(checkUsernameEffect).toBeCalledTimes(1);
      expect(checkUsernameEffect).toBeCalledWith("testuser");
      expect(overmind.mutations).toMatchSnapshot();
    });
  });
});
