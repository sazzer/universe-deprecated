import { Action } from "overmind";
import { ValidationErrors } from "../../api/validation";
import { AuthenticationError } from "./effects";

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
export const register: Action<
  RegisterValue,
  Promise<ValidationErrors | boolean>
> = async (
  { state, effects, actions },
  details: RegisterValue
): Promise<ValidationErrors | boolean> => {
  state.login.loading = true;
  state.login.error = null;
  try {
    const user = await effects.login.api.registerUser(
      details.username,
      details.email,
      details.displayName,
      details.password
    );
    actions.authentication.login({
      userId: user.id,
      accessToken: user.accessToken.token,
      expires: user.accessToken.expiry
    });
    state.login.loading = false;
    return true;
  } catch (e) {
    state.login.loading = false;
    if (e instanceof ValidationErrors) {
      return e;
    } else {
      state.login.error = e.toString();
      return false;
    }
  }
};

/**
 * Shape of the input value for authenticating as an existing user
 */
export interface AuthenticateValue {
  /** The username to use */
  username: string;
  /** The password to use */
  password: string;
}

/**
 * Attempt to authenticate as an existinguser
 */
export const authenticate: Action<
  AuthenticateValue,
  Promise<AuthenticationError | boolean>
> = async (
  { state, effects, actions },
  details: AuthenticateValue
): Promise<AuthenticationError | boolean> => {
  state.login.loading = true;
  state.login.error = null;
  try {
    const user = await effects.login.api.authenticateUser(
      details.username,
      details.password
    );
    actions.authentication.login({
      userId: user.id,
      accessToken: user.accessToken.token,
      expires: user.accessToken.expiry
    });
    state.login.loading = false;
    return true;
  } catch (e) {
    state.login.loading = false;
    if (e instanceof AuthenticationError) {
      return e;
    } else {
      state.login.error = e.toString();
      return false;
    }
  }
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
