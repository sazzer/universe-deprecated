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
  delete state.users.userStates[details.id];
};

export const fetchUser: Action<string, Promise<void>> = async (
  { state, actions, effects },
  userId: string
) => {
  state.users.userStates[userId] = "LOADING";
  const user = await effects.users.api.loadUser(userId);
  actions.users.storeUser(user);
};
