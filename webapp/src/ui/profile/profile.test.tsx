import { getUserById, useUser, updateUserProfile } from "../../users";
import { render, wait, fireEvent } from "@testing-library/react";

import { UserProfileArea } from "./profile";
import React from "react";
import { ValidationErrors } from "../../api";

jest.mock("../../users");

const useUserMock = useUser as jest.Mock;
const getUserByIdMock = getUserById as jest.Mock;
const updateUserProfileMock = updateUserProfile as jest.Mock;

afterEach(() => {
  jest.resetAllMocks();
});

test("Rendering before user loaded", async () => {
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

  getUserByIdMock.mockReturnValueOnce(new Promise(() => {}));

  const { container } = render(<UserProfileArea />);

  // Waiting on getUserByIdMock
  await wait(() => {});

  expect(container).toMatchSnapshot();
  expect(storeUser).toBeCalledTimes(0);
  expect(updateUserProfileMock).toBeCalledTimes(0);
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

  getUserByIdMock.mockResolvedValueOnce({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "testuser@example.com",
    username: "testuser",
    displayName: "Test User"
  });

  const { container } = render(<UserProfileArea />);

  // Waiting on getUserByIdMock
  await wait(() => {});

  expect(container).toMatchSnapshot();

  expect(storeUser).toBeCalledTimes(1);
  // Called in response to loading the user
  expect(storeUser).toBeCalledWith({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "testuser@example.com",
    username: "testuser",
    displayName: "Test User"
  });
  expect(updateUserProfileMock).toBeCalledTimes(0);
});

test("Saving invalid email address", async () => {
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

  getUserByIdMock.mockResolvedValueOnce({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "testuser@example.com",
    username: "testuser",
    displayName: "Test User"
  });

  const { container, getByText, getByLabelText } = render(<UserProfileArea />);

  // Waiting on getUserByIdMock
  await wait(() => {});

  await wait(() => {
    fireEvent.change(getByLabelText("Email Address"), {
      target: { value: "testuser@examplecom" }
    });
    fireEvent.click(getByText("Save Changes", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();

  expect(storeUser).toBeCalledTimes(1);
  // Called in response to loading the user
  expect(storeUser).toBeCalledWith({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "testuser@example.com",
    username: "testuser",
    displayName: "Test User"
  });
  expect(updateUserProfileMock).toBeCalledTimes(0);
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

  getUserByIdMock.mockResolvedValueOnce({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "testuser@example.com",
    username: "testuser",
    displayName: "Test User"
  });

  const { container, getByText, getByLabelText } = render(<UserProfileArea />);

  // Waiting on getUserByIdMock
  await wait(() => {});

  await wait(() => {
    fireEvent.change(getByLabelText("Email Address"), {
      target: { value: "" }
    });
    fireEvent.change(getByLabelText("Display Name"), {
      target: { value: "" }
    });
    fireEvent.click(getByText("Save Changes", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();

  expect(storeUser).toBeCalledTimes(1);
  // Called in response to loading the user
  expect(storeUser).toBeCalledWith({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "testuser@example.com",
    username: "testuser",
    displayName: "Test User"
  });
  expect(updateUserProfileMock).toBeCalledTimes(0);
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

  getUserByIdMock.mockResolvedValueOnce({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "testuser@example.com",
    username: "testuser",
    displayName: "Test User"
  });

  updateUserProfileMock.mockResolvedValueOnce({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "newuser@example.com",
    username: "testuser",
    displayName: "New User"
  });

  const { container, getByText, getByLabelText } = render(<UserProfileArea />);

  // Waiting on getUserByIdMock
  await wait(() => {});

  await wait(() => {
    fireEvent.change(getByLabelText("Email Address"), {
      target: { value: "newuser@example.com" }
    });
    fireEvent.change(getByLabelText("Display Name"), {
      target: { value: "New User" }
    });
    fireEvent.click(getByText("Save Changes", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(storeUser).toBeCalledTimes(2);
  // First is in response to loading the user
  expect(storeUser).toBeCalledWith({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "testuser@example.com",
    username: "testuser",
    displayName: "Test User"
  });
  // Second is in response to saving the profile
  expect(storeUser).toBeCalledWith({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "newuser@example.com",
    username: "testuser",
    displayName: "New User"
  });
  expect(updateUserProfileMock).toBeCalledTimes(1);
  expect(updateUserProfileMock).toBeCalledWith(
    "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    "newuser@example.com",
    "New User"
  );
});

test("Saving a duplicate email address", async () => {
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

  getUserByIdMock.mockResolvedValueOnce({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "testuser@example.com",
    username: "testuser",
    displayName: "Test User"
  });

  updateUserProfileMock.mockRejectedValueOnce(
    new ValidationErrors([
      {
        field: "email",
        type: "tag:universe,2020:users/validation-errors/email/duplicate"
      }
    ])
  );

  const { container, getByText, getByLabelText } = render(<UserProfileArea />);

  // Waiting on getUserByIdMock
  await wait(() => {});

  await wait(() => {
    fireEvent.change(getByLabelText("Email Address"), {
      target: { value: "newuser@example.com" }
    });
    fireEvent.change(getByLabelText("Display Name"), {
      target: { value: "New User" }
    });
    fireEvent.click(getByText("Save Changes", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(storeUser).toBeCalledTimes(1);
  // Called in response to loading the user
  expect(storeUser).toBeCalledWith({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "testuser@example.com",
    username: "testuser",
    displayName: "Test User"
  });
  expect(updateUserProfileMock).toBeCalledTimes(1);
  expect(updateUserProfileMock).toBeCalledWith(
    "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    "newuser@example.com",
    "New User"
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

  getUserByIdMock.mockResolvedValueOnce({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "testuser@example.com",
    username: "testuser",
    displayName: "Test User"
  });

  updateUserProfileMock.mockRejectedValueOnce(new Error("Network error"));

  const { container, getByText, getByLabelText } = render(<UserProfileArea />);

  // Waiting on getUserByIdMock
  await wait(() => {});

  await wait(() => {
    fireEvent.change(getByLabelText("Email Address"), {
      target: { value: "newuser@example.com" }
    });
    fireEvent.change(getByLabelText("Display Name"), {
      target: { value: "New User" }
    });
    fireEvent.click(getByText("Save Changes", { selector: "button" }));
  });

  expect(container).toMatchSnapshot();
  expect(storeUser).toBeCalledTimes(1);
  // Called in response to loading the user
  expect(storeUser).toBeCalledWith({
    id: "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    email: "testuser@example.com",
    username: "testuser",
    displayName: "Test User"
  });
  expect(updateUserProfileMock).toBeCalledTimes(1);
  expect(updateUserProfileMock).toBeCalledWith(
    "57c33107-b43e-4b53-a967-3ff89ccaaf16",
    "newuser@example.com",
    "New User"
  );
});
