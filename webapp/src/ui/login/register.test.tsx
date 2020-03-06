import { LoginFailure, register, useUser } from "../../users";
import { fireEvent, render, wait } from "@testing-library/react";

import React from "react";
import { RegisterUserPage } from "./register";
import { Router } from "react-router-dom";
import { ValidationErrors } from "../../api";
import { createMemoryHistory } from "history";

jest.mock("../../users");

const registerMock = register as jest.Mock;
const useUserMock = useUser as jest.Mock;

afterEach(() => {
  jest.resetAllMocks();
});

test("Initial rendering", () => {
  const onCancel = jest.fn();
  const storeUser = jest.fn();

  useUserMock.mockReturnValue({
    hasUser: false,
    user: null,
    clearUser: () => {},
    storeUser
  });

  const history = createMemoryHistory();
  const { container } = render(
    <Router history={history}>
      <RegisterUserPage username="testuser" onCancel={onCancel} />
    </Router>
  );
  expect(container).toMatchSnapshot();
  expect(registerMock).toBeCalledTimes(0);
  expect(onCancel).toBeCalledTimes(0);
  expect(storeUser).toBeCalledTimes(0);

  expect(history.location.pathname).toEqual("/");
});

test("Submit empty form", async () => {
  const onCancel = jest.fn();
  const storeUser = jest.fn();

  useUserMock.mockReturnValue({
    hasUser: false,
    user: null,
    clearUser: () => {},
    storeUser
  });

  const history = createMemoryHistory();
  const { container, getByText } = render(
    <Router history={history}>
      <RegisterUserPage username="testuser" onCancel={onCancel} />
    </Router>
  );

  await wait(() => {
    fireEvent.click(getByText("Register", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(registerMock).toBeCalledTimes(0);
  expect(onCancel).toBeCalledTimes(0);
  expect(storeUser).toBeCalledTimes(0);

  expect(history.location.pathname).toEqual("/");
});

test("Submit form - all whitespace", async () => {
  const onCancel = jest.fn();
  const storeUser = jest.fn();

  useUserMock.mockReturnValue({
    hasUser: false,
    user: null,
    clearUser: () => {},
    storeUser
  });

  const history = createMemoryHistory();
  const { container, getByText, getByLabelText } = render(
    <Router history={history}>
      <RegisterUserPage username="testuser" onCancel={onCancel} />
    </Router>
  );

  await wait(() => {
    fireEvent.change(getByLabelText("Email Address"), {
      target: { value: "    " }
    });
    fireEvent.change(getByLabelText("Display Name"), {
      target: { value: "    " }
    });
    fireEvent.change(getByLabelText("Password"), {
      target: { value: "    " }
    });
    fireEvent.change(getByLabelText("Repeat Password"), {
      target: { value: "    " }
    });
    fireEvent.click(getByText("Register", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(registerMock).toBeCalledTimes(0);
  expect(onCancel).toBeCalledTimes(0);
  expect(storeUser).toBeCalledTimes(0);

  expect(history.location.pathname).toEqual("/");
});

test("Submit form - different passwords", async () => {
  const onCancel = jest.fn();
  const storeUser = jest.fn();

  useUserMock.mockReturnValue({
    hasUser: false,
    user: null,
    clearUser: () => {},
    storeUser
  });

  const history = createMemoryHistory();
  const { container, getByText, getByLabelText } = render(
    <Router history={history}>
      <RegisterUserPage username="testuser" onCancel={onCancel} />
    </Router>
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
      target: { value: "Pa5word" }
    });
    fireEvent.click(getByText("Register", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(registerMock).toBeCalledTimes(0);
  expect(onCancel).toBeCalledTimes(0);
  expect(storeUser).toBeCalledTimes(0);

  expect(history.location.pathname).toEqual("/");
});

test("Submit form - malformed email address", async () => {
  const onCancel = jest.fn();
  const storeUser = jest.fn();

  useUserMock.mockReturnValue({
    hasUser: false,
    user: null,
    clearUser: () => {},
    storeUser
  });

  const history = createMemoryHistory();
  const { container, getByText, getByLabelText } = render(
    <Router history={history}>
      <RegisterUserPage username="testuser" onCancel={onCancel} />
    </Router>
  );

  await wait(() => {
    fireEvent.change(getByLabelText("Email Address"), {
      target: { value: "testuser@examplecom" }
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
  expect(registerMock).toBeCalledTimes(0);
  expect(onCancel).toBeCalledTimes(0);
  expect(storeUser).toBeCalledTimes(0);

  expect(history.location.pathname).toEqual("/");
});

test("Submit form - pending", async () => {
  const onCancel = jest.fn();
  const storeUser = jest.fn();

  useUserMock.mockReturnValue({
    hasUser: false,
    user: null,
    clearUser: () => {},
    storeUser
  });
  registerMock.mockReturnValueOnce(new Promise(() => {}));

  const history = createMemoryHistory();
  const { container, getByText, getByLabelText } = render(
    <Router history={history}>
      <RegisterUserPage username="testuser" onCancel={onCancel} />
    </Router>
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
  expect(registerMock).toBeCalledTimes(1);
  expect(registerMock).toBeCalledWith(
    "testuser",
    "testuser@example.com",
    "Test User",
    "Pa55word"
  );
  expect(onCancel).toBeCalledTimes(0);
  expect(storeUser).toBeCalledTimes(0);

  expect(history.location.pathname).toEqual("/");
});

test("Submit form - Success", async () => {
  const onCancel = jest.fn();
  const storeUser = jest.fn();

  useUserMock.mockReturnValue({
    hasUser: false,
    user: null,
    clearUser: () => {},
    storeUser
  });
  registerMock.mockResolvedValueOnce({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "testuser@example.com",
    username: "testuser",
    displayName: "Test User"
  });

  const history = createMemoryHistory();
  const { container, getByText, getByLabelText } = render(
    <Router history={history}>
      <RegisterUserPage username="testuser" onCancel={onCancel} />
    </Router>
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
  expect(registerMock).toBeCalledTimes(1);
  expect(registerMock).toBeCalledWith(
    "testuser",
    "testuser@example.com",
    "Test User",
    "Pa55word"
  );
  expect(onCancel).toBeCalledTimes(0);
  expect(storeUser).toBeCalledTimes(1);
  expect(storeUser).toBeCalledWith({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "testuser@example.com",
    username: "testuser",
    displayName: "Test User"
  });

  expect(history.location.pathname).toEqual("/profile");
});

test("Submit form - Duplicate Email Address", async () => {
  const onCancel = jest.fn();
  const storeUser = jest.fn();

  useUserMock.mockReturnValue({
    hasUser: false,
    user: null,
    clearUser: () => {},
    storeUser
  });
  registerMock.mockRejectedValueOnce(
    new ValidationErrors([
      {
        field: "email",
        type: "tag:universe,2020:users/validation-errors/email/duplicate"
      }
    ])
  );

  const history = createMemoryHistory();
  const { container, getByText, getByLabelText } = render(
    <Router history={history}>
      <RegisterUserPage username="testuser" onCancel={onCancel} />
    </Router>
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
  expect(registerMock).toBeCalledTimes(1);
  expect(registerMock).toBeCalledWith(
    "testuser",
    "testuser@example.com",
    "Test User",
    "Pa55word"
  );
  expect(onCancel).toBeCalledTimes(0);
  expect(storeUser).toBeCalledTimes(0);

  expect(history.location.pathname).toEqual("/");
});

test("Submit form - Unexpected error", async () => {
  const onCancel = jest.fn();
  const storeUser = jest.fn();

  useUserMock.mockReturnValue({
    hasUser: false,
    user: null,
    clearUser: () => {},
    storeUser
  });
  registerMock.mockRejectedValueOnce(new Error("Network Error"));

  const history = createMemoryHistory();
  const { container, getByText, getByLabelText } = render(
    <Router history={history}>
      <RegisterUserPage username="testuser" onCancel={onCancel} />
    </Router>
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
  expect(registerMock).toBeCalledTimes(1);
  expect(registerMock).toBeCalledWith(
    "testuser",
    "testuser@example.com",
    "Test User",
    "Pa55word"
  );
  expect(onCancel).toBeCalledTimes(0);
  expect(storeUser).toBeCalledTimes(0);

  expect(history.location.pathname).toEqual("/");
});

test("Cancel", async () => {
  const onCancel = jest.fn();
  const storeUser = jest.fn();

  useUserMock.mockReturnValue({
    hasUser: false,
    user: null,
    clearUser: () => {},
    storeUser
  });

  const history = createMemoryHistory();
  const { container, getByText } = render(
    <Router history={history}>
      <RegisterUserPage username="testuser" onCancel={onCancel} />
    </Router>
  );

  await wait(() => {
    fireEvent.click(getByText("Cancel", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(registerMock).toBeCalledTimes(0);
  expect(onCancel).toBeCalledTimes(1);
  expect(storeUser).toBeCalledTimes(0);

  expect(history.location.pathname).toEqual("/");
});
