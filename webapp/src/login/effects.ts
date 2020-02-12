import { request, ProblemResponse } from "../api";
import { ValidationErrors } from "../api/validation";

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

async function registerUser(
  username: string,
  email: string,
  displayName: string,
  password: string
) {
  try {
    await request({
      url: "/users",
      method: "POST",
      data: {
        username,
        email,
        displayName,
        password
      }
    });
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
export const api = {
  checkUsername,
  registerUser
};
