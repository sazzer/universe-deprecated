import { request, ProblemResponse } from "../../api";
import { ValidationErrors } from "../../api/validation";
/**
 * Check if a given username is already registered or not
 * @param username The username to check
 * @return True if the username is already registered. False if not.
 */
async function checkUsername(username: string): Promise<boolean> {
  try {
    await request({
      url: "/usernames/{username}",
      urlParams: {
        username
      },
      method: "GET"
    });

    return true;
  } catch (e) {
    if (
      e instanceof ProblemResponse &&
      e.problem.type === "tag:universe,2020:users/problems/unknown-user"
    ) {
      return false;
    } else {
      throw e;
    }
  }
}

/** The shape of an access token received from the server */
export interface AccessToken {
  /** The access token */
  token: string;
  /** The expiry date */
  expiry: string;
}

/** The shape of an authenticated user received from the server */
export interface AuthenticatedUser {
  /** The User ID */
  id: string;
  /** The email address */
  email: string;
  /** The display name */
  displayName: string;
  /** The username */
  username: string;
  /** The access token to use for this user */
  accessToken: AccessToken;
}

/**
 * Attempt to register a new user with the server
 * @param username The username to register
 * @param email The email address to register
 * @param displayName The display name to register
 * @param password The password to register
 * @return The details of the user after registration
 * @throws ValidationErrors if the details were invalid.
 */
async function registerUser(
  username: string,
  email: string,
  displayName: string,
  password: string
): Promise<AuthenticatedUser> {
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
    return user.data;
  } catch (e) {
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

/** Error class to indicate that authentication failed */
export class AuthenticationError extends Error {}

/**
 * Attempt to authenticate an existing user with the server
 * @param username The username to authenticate
 * @param password The password to authenticate
 * @return The details of the user after authentication
 * @throws AuthenticationError if the details were invalid.
 */

async function authenticateUser(
  username: string,
  password: string
): Promise<AuthenticatedUser> {
  try {
    const user = await request<AuthenticatedUser>({
      url: "/login",
      method: "POST",
      data: {
        username,
        password
      }
    });
    return user.data;
  } catch (e) {
    if (
      e instanceof ProblemResponse &&
      e.problem.type === "tag:universe,2020:users/problems/login_failure"
    ) {
      throw new AuthenticationError();
    } else {
      throw e;
    }
  }
}

/** The API for logging in */
export const api = {
  authenticateUser,
  checkUsername,
  registerUser
};
