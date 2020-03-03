/**
 * Representation of the user that is currently using the application
 */
export interface User {
  /** The User ID of the user */
  userId: string;
  /** The username of the user */
  username: string;
  /** The display name of the user */
  displayName: string;
  /** The email address of the user */
  email: string;
}

/**
 * Representation of an Access Token for a user
 */
export interface AccessToken {
  token: string;
}

/**
 * Representation of a User that has just authenticated, and thus also has an access token
 */
export interface AuthenticatedUser extends User {
  accessToken: AccessToken;
}
