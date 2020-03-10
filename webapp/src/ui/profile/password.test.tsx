import { changePassword, useUser } from "../../users";
import { fireEvent, render, wait } from "@testing-library/react";

import { ChangePasswordArea } from "./password";
import React from "react";

jest.mock("../../users");

const useUserMock = useUser as jest.Mock;
const changePasswordMock = changePassword as jest.Mock;

afterEach(() => {
  jest.resetAllMocks();
});

test("Rendering after user loaded", async () => {
  const storeUser = jest.fn();

  useUserMock.mockReturnValue({
    hasUser: true,
    user: {
      id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
      email: "testuser@example.com",
      username: "testuser",
      displayName: "Test User"
    },
    clearUser: () => {},
    storeUser
  });

  const { container } = render(<ChangePasswordArea />);

  // Waiting on getUserByIdMock
  await wait(() => {});

  expect(container).toMatchSnapshot();

  expect(storeUser).toBeCalledTimes(0);
  expect(changePasswordMock).toBeCalledTimes(0);
});

test("Saving blank fields", async () => {
  const storeUser = jest.fn();

  useUserMock.mockReturnValue({
    hasUser: true,
    user: {
      id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
      email: "testuser@example.com",
      username: "testuser",
      displayName: "Test User"
    },
    clearUser: () => {},
    storeUser
  });

  const { container, getByText, getByLabelText } = render(
    <ChangePasswordArea />
  );

  // Waiting on getUserByIdMock
  await wait(() => {});

  await wait(() => {
    fireEvent.change(getByLabelText("New Password"), {
      target: { value: "" }
    });
    fireEvent.change(getByLabelText("Repeat Password"), {
      target: { value: "" }
    });
    fireEvent.click(getByText("Change Password", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(storeUser).toBeCalledTimes(0);
  expect(changePasswordMock).toBeCalledTimes(0);
});

test("Saving mismatched passwords", async () => {
  const storeUser = jest.fn();

  useUserMock.mockReturnValue({
    hasUser: true,
    user: {
      id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
      email: "testuser@example.com",
      username: "testuser",
      displayName: "Test User"
    },
    clearUser: () => {},
    storeUser
  });

  const { container, getByText, getByLabelText } = render(
    <ChangePasswordArea />
  );

  // Waiting on getUserByIdMock
  await wait(() => {});

  await wait(() => {
    fireEvent.change(getByLabelText("New Password"), {
      target: { value: "Pa55word" }
    });
    fireEvent.change(getByLabelText("Repeat Password"), {
      target: { value: "password" }
    });
    fireEvent.click(getByText("Change Password", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(storeUser).toBeCalledTimes(0);
  expect(changePasswordMock).toBeCalledTimes(0);
});
test("Saving successfully", async () => {
  const storeUser = jest.fn();

  useUserMock.mockReturnValue({
    hasUser: true,
    user: {
      id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
      email: "testuser@example.com",
      username: "testuser",
      displayName: "Test User"
    },
    clearUser: () => {},
    storeUser
  });

  changePasswordMock.mockResolvedValueOnce({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "newuser@example.com",
    username: "testuser",
    displayName: "New User"
  });

  const { container, getByText, getByLabelText } = render(
    <ChangePasswordArea />
  );

  // Waiting on getUserByIdMock
  await wait(() => {});

  await wait(() => {
    fireEvent.change(getByLabelText("New Password"), {
      target: { value: "Pa55word" }
    });
    fireEvent.change(getByLabelText("Repeat Password"), {
      target: { value: "Pa55word" }
    });
    fireEvent.click(getByText("Change Password", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(storeUser).toBeCalledTimes(0);
  expect(changePasswordMock).toBeCalledTimes(1);
  expect(changePasswordMock).toBeCalledWith(
    "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    "Pa55word"
  );
});

test("Saving with an unexpected error", async () => {
  const storeUser = jest.fn();

  useUserMock.mockReturnValue({
    hasUser: true,
    user: {
      id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
      email: "testuser@example.com",
      username: "testuser",
      displayName: "Test User"
    },
    clearUser: () => {},
    storeUser
  });

  changePasswordMock.mockRejectedValueOnce(new Error("Network error"));

  const { container, getByText, getByLabelText } = render(
    <ChangePasswordArea />
  );

  // Waiting on getUserByIdMock
  await wait(() => {});

  await wait(() => {
    fireEvent.change(getByLabelText("New Password"), {
      target: { value: "Pa55word" }
    });
    fireEvent.change(getByLabelText("Repeat Password"), {
      target: { value: "Pa55word" }
    });
    fireEvent.click(getByText("Change Password", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(storeUser).toBeCalledTimes(0);
  expect(changePasswordMock).toBeCalledTimes(1);
  expect(changePasswordMock).toBeCalledWith(
    "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    "Pa55word"
  );
});
