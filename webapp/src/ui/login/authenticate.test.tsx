import { LoginFailure, authenticate, useUser } from "../../users";
import { fireEvent, render, wait } from "@testing-library/react";

import { AuthenticateUserPage } from "./authenticate";
import React from "react";
import { Router } from "react-router-dom";
import { createMemoryHistory } from "history";

jest.mock("../../users");

const authenticateMock = authenticate as jest.Mock;
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
      <AuthenticateUserPage username="testuser" onCancel={onCancel} />
    </Router>
  );
  expect(container).toMatchSnapshot();
  expect(authenticateMock).toBeCalledTimes(0);
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
      <AuthenticateUserPage username="testuser" onCancel={onCancel} />
    </Router>
  );

  await wait(() => {
    fireEvent.click(getByText("Login", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(authenticateMock).toBeCalledTimes(0);
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
  authenticateMock.mockReturnValueOnce(new Promise(() => {}));

  const history = createMemoryHistory();
  const { container, getByText, getByLabelText } = render(
    <Router history={history}>
      <AuthenticateUserPage username="testuser" onCancel={onCancel} />
    </Router>
  );

  await wait(() => {
    fireEvent.change(getByLabelText("Password"), {
      target: { value: "Pa55word" }
    });
    fireEvent.click(getByText("Login", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(authenticateMock).toBeCalledTimes(1);
  expect(authenticateMock).toBeCalledWith("testuser", "Pa55word");
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
  authenticateMock.mockResolvedValueOnce({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "testuser@example.com",
    username: "testuser",
    displayName: "Test User"
  });

  const history = createMemoryHistory();
  const { container, getByText, getByLabelText } = render(
    <Router history={history}>
      <AuthenticateUserPage username="testuser" onCancel={onCancel} />
    </Router>
  );

  await wait(() => {
    fireEvent.change(getByLabelText("Password"), {
      target: { value: "Pa55word" }
    });
    fireEvent.click(getByText("Login", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(authenticateMock).toBeCalledTimes(1);
  expect(authenticateMock).toBeCalledWith("testuser", "Pa55word");
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

test("Submit form - Wrong password", async () => {
  const onCancel = jest.fn();
  const storeUser = jest.fn();

  useUserMock.mockReturnValue({
    hasUser: false,
    user: null,
    clearUser: () => {},
    storeUser
  });
  authenticateMock.mockRejectedValueOnce(new LoginFailure());

  const history = createMemoryHistory();
  const { container, getByText, getByLabelText } = render(
    <Router history={history}>
      <AuthenticateUserPage username="testuser" onCancel={onCancel} />
    </Router>
  );

  await wait(() => {
    fireEvent.change(getByLabelText("Password"), {
      target: { value: "Pa55word" }
    });
    fireEvent.click(getByText("Login", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(authenticateMock).toBeCalledTimes(1);
  expect(authenticateMock).toBeCalledWith("testuser", "Pa55word");
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
  authenticateMock.mockRejectedValueOnce(new Error("Network Error"));

  const history = createMemoryHistory();
  const { container, getByText, getByLabelText } = render(
    <Router history={history}>
      <AuthenticateUserPage username="testuser" onCancel={onCancel} />
    </Router>
  );

  await wait(() => {
    fireEvent.change(getByLabelText("Password"), {
      target: { value: "Pa55word" }
    });
    fireEvent.click(getByText("Login", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(authenticateMock).toBeCalledTimes(1);
  expect(authenticateMock).toBeCalledWith("testuser", "Pa55word");
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
      <AuthenticateUserPage username="testuser" onCancel={onCancel} />
    </Router>
  );

  await wait(() => {
    fireEvent.click(getByText("Cancel", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(authenticateMock).toBeCalledTimes(0);
  expect(onCancel).toBeCalledTimes(1);
  expect(storeUser).toBeCalledTimes(0);

  expect(history.location.pathname).toEqual("/");
});
