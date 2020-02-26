import { User, UserState, StoredUser } from "./model";

/** The shape of this part of the state */
interface State {
  /** The set of all known users */
  users: { [userId: string]: User };

  /** The state of the user */
  userStates: { [userId: string]: UserState };

  /** Get a user from the state by the Users ID */
  readonly userById: (id: string) => StoredUser;
}

/** The initial value for this part of the state */
export const state: State = {
  users: {},
  userStates: {},
  get userById() {
    return (id: string) => {
      let state = this.userStates[id] || "LOADED";
      const user = this.users[id];

      if (user === undefined) {
        state = "UNKNOWN";
      }

      return {
        state,
        user
      };
    };
  }
};
