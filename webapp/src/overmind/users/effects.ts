import { request, ProblemResponse } from "../../api";
import { User, UnknownUserError } from "./model";

/**
 * Retrieve a user from the API
 * @param id The ID of the user to retrieve
 * @return The details of the user
 */
async function loadUser(id: string): Promise<User> {
  try {
    const user = await request<User>({
      url: "/users/{id}",
      urlParams: {
        id
      },
      method: "GET"
    });

    return user.data;
  } catch (e) {
    throw new UnknownUserError();
  }
}

/** The API for working with users*/
export const api = {
  loadUser
};
