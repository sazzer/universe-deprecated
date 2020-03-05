import { fireEvent, render, wait } from "@testing-library/react";

import React from "react";
import { StartLoginPage } from "./start";
import { checkUsername } from "../../users";

jest.mock("../../users");

const checkUsernameMock = checkUsername as jest.Mock;

afterEach(() => {
  jest.resetAllMocks();
});

test("Initial rendering", () => {
  const onUsername = jest.fn();
  const { container } = render(<StartLoginPage onUsername={onUsername} />);

  expect(container).toMatchSnapshot();
  expect(checkUsernameMock).toBeCalledTimes(0);
  expect(onUsername).toBeCalledTimes(0);
});

test("Submit empty form", async () => {
  const onUsername = jest.fn();
  const { container, getByText } = render(
    <StartLoginPage onUsername={onUsername} />
  );

  await wait(() => {
    fireEvent.click(getByText("Login / Register", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(checkUsernameMock).toBeCalledTimes(0);
  expect(onUsername).toBeCalledTimes(0);
});

test("Submit whitespace username", async () => {
  const onUsername = jest.fn();
  const { container, getByText, getByLabelText } = render(
    <StartLoginPage onUsername={onUsername} />
  );

  await wait(() => {
    fireEvent.change(getByLabelText("Username"), {
      target: { value: "    " }
    });
    fireEvent.click(getByText("Login / Register", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(checkUsernameMock).toBeCalledTimes(0);
  expect(onUsername).toBeCalledTimes(0);
});

test("Submit valid username - user known", async () => {
  checkUsernameMock.mockResolvedValueOnce(true);

  const onUsername = jest.fn();

  const { container, getByText, getByLabelText } = render(
    <StartLoginPage onUsername={onUsername} />
  );

  await wait(() => {
    fireEvent.change(getByLabelText("Username"), {
      target: { value: "testuser" }
    });
    fireEvent.click(getByText("Login / Register", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(checkUsernameMock).toBeCalledTimes(1);
  expect(checkUsernameMock).toBeCalledWith("testuser");
  expect(onUsername).toBeCalledTimes(1);
  expect(onUsername).toBeCalledWith("testuser", true);
});

test("Submit valid username - user unknown", async () => {
  checkUsernameMock.mockResolvedValueOnce(false);

  const onUsername = jest.fn();

  const { container, getByText, getByLabelText } = render(
    <StartLoginPage onUsername={onUsername} />
  );

  await wait(() => {
    fireEvent.change(getByLabelText("Username"), {
      target: { value: "testuser" }
    });
    fireEvent.click(getByText("Login / Register", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(checkUsernameMock).toBeCalledTimes(1);
  expect(checkUsernameMock).toBeCalledWith("testuser");
  expect(onUsername).toBeCalledTimes(1);
  expect(onUsername).toBeCalledWith("testuser", false);
});

test("Submit valid username - unexpected error", async () => {
  checkUsernameMock.mockRejectedValueOnce(new Error("Network Error"));

  const onUsername = jest.fn();

  const { container, getByText, getByLabelText } = render(
    <StartLoginPage onUsername={onUsername} />
  );

  await wait(() => {
    fireEvent.change(getByLabelText("Username"), {
      target: { value: "testuser" }
    });
    fireEvent.click(getByText("Login / Register", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(checkUsernameMock).toBeCalledTimes(1);
  expect(checkUsernameMock).toBeCalledWith("testuser");
  expect(onUsername).toBeCalledTimes(0);
});

test("Submit valid username - pending", async () => {
  checkUsernameMock.mockReturnValue(new Promise(() => {}));

  const onUsername = jest.fn();

  const { container, getByText, getByLabelText } = render(
    <StartLoginPage onUsername={onUsername} />
  );

  await wait(() => {
    fireEvent.change(getByLabelText("Username"), {
      target: { value: "testuser" }
    });
    fireEvent.click(getByText("Login / Register", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(checkUsernameMock).toBeCalledTimes(1);
  expect(checkUsernameMock).toBeCalledWith("testuser");
  expect(onUsername).toBeCalledTimes(0);
});
