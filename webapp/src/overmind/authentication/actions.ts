import { Action } from "overmind";

/**
 * Overmind action for logging out.
 */
export const logout: Action = ({ state }) => {
  state.authentication.accessToken = null;
  state.authentication.userId = null;
};

/**
 * Action input for logging in
 */
export interface LoginInput {
  /** The access token */
  accessToken: string;
  /** The expiry timestamp for the access token */
  expires: string;
  /** The ID of the User */
  userId: string;
}

/**
 * Overmind action for logging in
 */
export const login: Action<LoginValue> = ({ state }, details: LoginInput) => {
  state.authentication.accessToken = {
    accessToken: details.accessToken,
    expires: details.expires
  };
  state.authentication.userId = details.userId;
};
