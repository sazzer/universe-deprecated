import { Action } from "overmind";
import { User } from "./state";

/**
 * Overmind action for storing the details of a user
 */
export const storeUser: Action<User> = ({ state }, details: User) => {
  const users = state.users.users.filter(
    user => user.userId !== details.userId
  );
  users.push({
    userId: details.userId,
    username: details.username,
    displayName: details.displayName,
    email: details.email
  });
  state.users.users = users;
};
