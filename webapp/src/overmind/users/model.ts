/** The state of the user record */
export type UserState = "LOADING" | "SAVING" | "UNKNOWN" | "LOADED";

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

/** Representation of a user as known by the state */
export interface StoredUser {
  /** The user record, if we've currently got one */
  user: User | undefined;

  /** The current state of the user */
  state: UserState;
}

/** Error when loading a user that doesn't exist */
export class UnknownUserError extends Error {}
