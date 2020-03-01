import { Action } from "overmind";

/**
 * Overmind action for logging out.
 */
export const logout: Action = ({ state, effects }) => {
  state.authentication.accessToken = null;
  state.authentication.userId = null;
  effects.authentication.api.clearAccessToken();
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
export const login: Action<LoginInput> = (
  { state, effects },
  details: LoginInput
) => {
  state.authentication.accessToken = {
    accessToken: details.accessToken,
    expires: details.expires
  };
  state.authentication.userId = details.userId;
  effects.authentication.api.storeAccessToken(details.accessToken);
};
