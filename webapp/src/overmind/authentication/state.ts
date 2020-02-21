/** The shape of the access token for this session */
export interface AccessToken {
  /** The actual access token */
  accessToken: string;
  /** When the token expires, as an ISO-8601 String */
  expires: string;
}

/** The shape of this part of the state */
interface State {
  /** The User ID that we are authenticated as */
  userId: string | null;
  /** The access token that we are authenticated with */
  accessToken: AccessToken | null;

  /** Whether the user is currently logged in */
  isLoggedIn: boolean;
}

/**
 * The initial value for this part of the state
 */
export const state: State = {
  userId: null,
  accessToken: null,

  get isLoggedIn() {
    return this.userId !== null;
  }
};
