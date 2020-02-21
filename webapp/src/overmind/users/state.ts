/** The shape of a single user */
export interface User {
  /** The unqiue ID of the user */
  userId: string;
  /** The unique username of the user */
  username: string;
  /** The display name of the user */
  displayName: string;
  /** The email address of the user, if we know what it is */
  email: string | null;
}

/** The shape of this part of the state */
interface State {
  /** The set of all known users */
  users: { [userId: string]: User };
}

/** The initial value for this part of the state */
export const state: State = {
  users: {}
};
