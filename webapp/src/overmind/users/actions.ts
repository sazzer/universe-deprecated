import { Action } from "overmind";
import { User } from "./state";

/**
 * Overmind action for storing the details of a user
 */
export const storeUser: Action<User> = ({ state }, details: User) => {
  state.users.users[details.userId] = {
    userId: details.userId,
    username: details.username,
    displayName: details.displayName,
    email: details.email
  };
};

export const fetchUser: Action<string> = ({ effects }, userId: string) => {
  console.log("Load User: ", userId);
};
