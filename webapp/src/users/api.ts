import { ProblemResponse, request } from "../api";

import { AuthenticatedUser } from "./model";
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
 * Attempt to authenticate with the given credentials
 * @param username the username to authenticate with
 * @param password the password to authenticate with
 */
export async function authenticate(
  username: string,
  password: string
): Promise<AuthenticatedUser> {
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
    return user.data;
  } catch (e) {
    LOG("Failed to authenticate as username %s: %o", username, e);
    throw e;
  }
}
