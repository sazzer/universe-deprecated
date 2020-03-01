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

export interface SaveUserValue {
  userId: string;
  displayName: string;
  email: string;
}

export const saveUser: Action<SaveUserValue, Promise<void>> = async (
  { state, actions, effects },
  { userId, displayName, email }: SaveUserValue
) => {
  state.users.userStates[userId] = "SAVING";
  const user = await effects.users.api.saveUser(userId, displayName, email);
  actions.users.storeUser(user);
};
