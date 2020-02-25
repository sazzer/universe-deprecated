import { Action } from "overmind";
import { User } from "./model";

/**
 * Overmind action for storing the details of a user
 */
export const storeUser: Action<User> = ({ state }, details: User) => {
  state.users.users[details.id] = {
    id: details.id,
    username: details.username,
    displayName: details.displayName,
    email: details.email
  };
};

export const fetchUser: Action<string, Promise<void>> = async (
  { effects },
  userId: string
) => {
  const user = await effects.users.api.loadUser(userId);
  console.log(user);
};
