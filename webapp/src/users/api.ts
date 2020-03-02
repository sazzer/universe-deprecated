import { ProblemResponse, request } from "../api";

/**
 * Check if a given username is registered with the service or not
 * @param username The username to check
 * @return True if the username is already registered. False if it's not.
 */
export async function checkUsername(username: string): Promise<boolean> {
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
