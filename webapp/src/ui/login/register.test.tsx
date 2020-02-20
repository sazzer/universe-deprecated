import React from "react";
import { render, wait, fireEvent } from "@testing-library/react";
import { Provider } from "overmind-react";
import { RegisterForm } from "./register";
import { createTestOvermind } from "../../overmind/test";
import { OvermindMock, Config } from "overmind";
import { ValidationErrors } from "../../api/validation";
import { MemoryRouter } from "react-router-dom";

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
        <MemoryRouter>
          <RegisterForm />
        </MemoryRouter>
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
        <MemoryRouter>
          <RegisterForm />
        </MemoryRouter>
      </Provider>
    );
    await wait(() =>
      fireEvent.click(getByText("Cancel", { selector: "button" }))
    );
    expect(overmind.mutations).toMatchSnapshot();
  });
  describe("Submitting the form", () => {
    let registerEffect: jest.Mock;
    let overmind: OvermindMock<Config>;

    beforeEach(() => {
      registerEffect = jest.fn();

      overmind = createTestOvermind(
        {
          login: {
            api: {
              registerUser: registerEffect
            }
          }
        },
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
          <MemoryRouter>
            <RegisterForm />
          </MemoryRouter>
        </Provider>
      );
      await wait(() =>
        fireEvent.click(getByText("Register", { selector: "button" }))
      );
      expect(container).toMatchSnapshot();
      expect(registerEffect).toBeCalledTimes(0);
    });
    test("Submit with whitespace for everything", async () => {
      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <MemoryRouter>
            <RegisterForm />
          </MemoryRouter>
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
      expect(registerEffect).toBeCalledTimes(0);
    });
    test("Submit with invalid email address", async () => {
      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <MemoryRouter>
            <RegisterForm />
          </MemoryRouter>
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
      expect(registerEffect).toBeCalledTimes(0);
    });
    test("Submit with mismatched passwords", async () => {
      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <MemoryRouter>
            <RegisterForm />
          </MemoryRouter>
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
      expect(registerEffect).toBeCalledTimes(0);
    });

    test("Submit successfully", async () => {
      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <MemoryRouter>
            <RegisterForm />
          </MemoryRouter>
        </Provider>
      );
      registerEffect.mockResolvedValueOnce(undefined);

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
          target: { value: "Pa55word" }
        });
        fireEvent.click(getByText("Register", { selector: "button" }));
      });
      expect(container).toMatchSnapshot();
      expect(registerEffect).toBeCalledTimes(1);
      expect(registerEffect).toBeCalledWith(
        "testuser",
        "testuser@example.com",
        "Test User",
        "Pa55word"
      );
    });

    test("Submit with network error", async () => {
      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <MemoryRouter>
            <RegisterForm />
          </MemoryRouter>
        </Provider>
      );
      registerEffect.mockRejectedValueOnce(new Error("Network Error"));

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
          target: { value: "Pa55word" }
        });
        fireEvent.click(getByText("Register", { selector: "button" }));
      });
      expect(container).toMatchSnapshot();
      expect(registerEffect).toBeCalledTimes(1);
      expect(registerEffect).toBeCalledWith(
        "testuser",
        "testuser@example.com",
        "Test User",
        "Pa55word"
      );
    });

    test("Submit with duplicate email address", async () => {
      const { container, getByText, getByLabelText } = render(
        <Provider value={overmind}>
          <MemoryRouter>
            <RegisterForm />
          </MemoryRouter>
        </Provider>
      );
      registerEffect.mockRejectedValueOnce(
        new ValidationErrors([
          {
            type: "tag:universe,2020:users/validation-errors/email/duplicate",
            field: "email"
          }
        ])
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
          target: { value: "Pa55word" }
        });
        fireEvent.click(getByText("Register", { selector: "button" }));
      });
      expect(container).toMatchSnapshot();
      expect(registerEffect).toBeCalledTimes(1);
      expect(registerEffect).toBeCalledWith(
        "testuser",
        "testuser@example.com",
        "Test User",
        "Pa55word"
      );
    });
  });
});
