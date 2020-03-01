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
