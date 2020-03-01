import { setAccessToken } from "../../api";

/**
 * Store the access token against the API client
 * @param token The access token to store
 */
export function storeAccessToken(token: string) {
  setAccessToken(token);
}

/**
 * Clear the access token from the API client
 */
export function clearAccessToken() {
  setAccessToken(undefined);
}

/** The API for logging in */
export const api = {
  storeAccessToken,
  clearAccessToken
};
