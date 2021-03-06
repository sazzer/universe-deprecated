import { AuthenticatedUser, User } from "./model";
import {
  ProblemResponse,
  ValidationErrors,
  request,
  setAccessToken
} from "../api";

import debug from "debug";

/** The logger to use */
const LOG = debug("universe:users:api");

/**
 * Check if a given username is registered with the service or not
 * @param username The username to check
 * @return True if the username is already registered. False if it's not.
 */
export async function checkUsername(username: string): Promise<boolean> {
  LOG("Checking the existance of username: %s", username);
  try {
    await request({
      url: "/usernames/{username}",
      urlParams: {
        username
      },
      method: "GET"
    });

    LOG("Existance of username %s: true", username);
    return true;
  } catch (e) {
    if (
      e instanceof ProblemResponse &&
      e.problem.type === "tag:universe,2020:users/problems/unknown-user"
    ) {
      LOG("Existance of username %s: false", username);
      return false;
    } else {
      LOG("Error checking existance of username %s: %o", username, e);
      throw e;
    }
  }
}

/**
 * Load the user record with the given ID
 * @param userId The User ID to load
 * @return The details of the user
 */
export async function getUserById(
  userId: string,
  forceReload: boolean = false
): Promise<User> {
  LOG("Loading user: %s", userId);
  try {
    const user = await request<User>({
      url: "/users/{userId}",
      urlParams: {
        userId
      },
      method: "GET",
      forceReload
    });

    LOG("User details: %o", user);
    return user.data;
  } catch (e) {
    LOG("Failed to load user with ID %s: %o", userId, e);
    throw e;
  }
}

/** Error representation for a failure to log in - e.g. incorrect password */
export class LoginFailure extends Error {}

/**
 * Attempt to authenticate with the given credentials
 * @param username the username to authenticate with
 * @param password the password to authenticate with
 */
export async function authenticate(
  username: string,
  password: string
): Promise<User> {
  LOG("Authenticating as username %s with password %s", username, password);
  try {
    const user = await request<AuthenticatedUser>({
      url: "/login",
      method: "POST",
      data: {
        username,
        password
      }
    });

    LOG("Authenticated successfully: %o", user);
    setAccessToken(user.data.accessToken.token);
    // Strip out the non-user details from the return
    return {
      id: user.data.id,
      username: user.data.username,
      displayName: user.data.displayName,
      email: user.data.email
    };
  } catch (e) {
    if (
      e instanceof ProblemResponse &&
      e.problem.type === "tag:universe,2020:users/problems/login_failure"
    ) {
      LOG("Login failure for user %s", username);
      throw new LoginFailure();
    } else {
      LOG("Failed to authenticate as username %s: %o", username, e);
      throw e;
    }
  }
}

/**
 * Attempt to register with the given user details
 * @param username the username to register with
 * @param email the email address to register with
 * @param displayName the display name to register with
 * @param password the password to register with
 * @param password the password to register with
 */
export async function register(
  username: string,
  email: string,
  displayName: string,
  password: string
): Promise<User> {
  LOG("Registering with details: %o", {
    username,
    email,
    displayName,
    password
  });
  try {
    const user = await request<AuthenticatedUser>({
      url: "/users",
      method: "POST",
      data: {
        username,
        email,
        displayName,
        password
      }
    });

    LOG("Registered successfully: %o", user);
    setAccessToken(user.data.accessToken.token);
    // Strip out the non-user details from the return
    return {
      id: user.data.id,
      username: user.data.username,
      displayName: user.data.displayName,
      email: user.data.email
    };
  } catch (e) {
    LOG("Failed to register as username %s: %o", username, e);
    if (
      e instanceof ProblemResponse &&
      e.problem.type === "tag:universe,2020:problems/validation-error"
    ) {
      throw new ValidationErrors(e.problem.errors);
    } else {
      throw e;
    }
  }
}

/**
 * Interface describing the details we can patch on a user
 */
interface UserPatch {
  email?: string;
  displayName?: string;
  password?: string;
}

/**
 * Actually perform a patch of a user record
 * @param userId The ID of the user to patch
 * @param details The details of the user to patch
 */
async function patchUser(userId: string, details: UserPatch): Promise<User> {
  LOG("Updating user %s with details: %o", userId, details);

  try {
    const user = await request<User>({
      url: "/users/{userId}",
      method: "PATCH",
      urlParams: {
        userId
      },
      headers: {
        "content-type": "application/merge-patch+json"
      },
      data: details
    });

    LOG("Updated successfully: %o", user);
    // Strip out the non-user details from the return
    return {
      id: user.data.id,
      username: user.data.username,
      displayName: user.data.displayName,
      email: user.data.email
    };
  } catch (e) {
    LOG("Failed to update user %s: %o", userId, e);
    if (
      e instanceof ProblemResponse &&
      e.problem.type === "tag:universe,2020:problems/validation-error"
    ) {
      throw new ValidationErrors(e.problem.errors);
    } else {
      throw e;
    }
  }
}

/**
 * Call to update the user profile of the given user
 * @param userId The ID of the user
 * @param email The new email address of the user
 * @param displayName The new display name of the user
 */
export async function updateUserProfile(
  userId: string,
  email: string,
  displayName: string
): Promise<User> {
  return patchUser(userId, { email, displayName });
}

/**
 * Call to update the password of the given user
 * @param userId The ID of the user
 * @param password The new password of the user
 */
export async function changePassword(
  userId: string,
  password: string
): Promise<User> {
  return patchUser(userId, { password });
}
