/** The shape of a single user */
export interface User {
  /** The unqiue ID of the user */
  id: string;
  /** The unique username of the user */
  username: string;
  /** The display name of the user */
  displayName: string;
  /** The email address of the user, if we know what it is */
  email: string | null;
}

/** Error when loading a user that doesn't exist */
export class UnknownUserError extends Error {}
