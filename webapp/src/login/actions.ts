import { Action } from 'overmind'

/**
 * Check if the given username is already registered or not
 * @param  username The username to check
 */
export const checkUsername: Action<string, Promise<void>> = async ({ state, effects }, username: string) => {
  return state.login.mode.loading(async () => {
    state.login.error = null;
    state.login.username = null;
    try {
      const usernameKnown = await effects.login.api.checkUsername(username);
      if (usernameKnown) {
        return state.login.mode.authenticating(() => {
          state.login.username = username;
        });
      } else {
        return state.login.mode.registering(() => {
          state.login.username = username;
        });
      }
    } catch (e) {
      return state.login.mode.initial(() => {
        state.login.error = e.toString();
      });
    }
  });
}

/**
 * Reset the login process back to the start
 */
export const resetLogin: Action = ({ state }) => {
  state.login.username = null;
  state.login.error = null;
  return state.login.mode.initial();
}

/**
 * Cancel a login process we're in the middle of performing
 */
export const cancelLogin: Action = ({ actions }) => {
  return actions.login.resetLogin();
}
