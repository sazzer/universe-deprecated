import { Action } from "overmind";

/**
 * Check if the given username is already registered or not
 * @param  username The username to check
 */
export const checkUsername: Action<string, Promise<void>> = async (
  { state, effects },
  username: string
) => {
  state.login.loading = true;
  state.login.error = null;
  state.login.username = null;
  try {
    const usernameKnown = await effects.login.api.checkUsername(username);
    if (usernameKnown) {
      return state.login.mode.authenticating(() => {
        state.login.username = username;
        state.login.loading = false;
      });
    } else {
      return state.login.mode.registering(() => {
        state.login.username = username;
        state.login.loading = false;
      });
    }
  } catch (e) {
    return state.login.mode.initial(() => {
      state.login.error = e.toString();
      state.login.loading = false;
    });
  }
};

/**
 * Shape of the input value for registering a new user
 */
export interface RegisterValue {
  /** The username to register */
  username: string;
  /** The email address to register */
  email: string;
  /** The display name to register */
  displayName: string;
  /** The password to register */
  password: string;
}
/**
 * Attempt to register a new user
 */
export const register: Action<RegisterValue, Promise<void>> = async (
  { state, effects },
  details: RegisterValue
) => {
  state.login.loading = true;
  await effects.login.api.registerUser(
    details.username,
    details.email,
    details.displayName,
    details.password
  );
  state.login.loading = false;
};

/**
 * Reset the login process back to the start
 */
export const resetLogin: Action = ({ state }) => {
  state.login.username = null;
  state.login.error = null;
  state.login.loading = false;
  return state.login.mode.initial();
};

/**
 * Cancel a login process we're in the middle of performing
 */
export const cancelLogin: Action = ({ actions }) => {
  return actions.login.resetLogin();
};
