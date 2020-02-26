import { User } from "./model";
import { Derive } from "overmind";

/** The shape of this part of the state */
interface State {
  /** The set of all known users */
  users: { [userId: string]: User };

  /** Get a user from the state by the Users ID */
  readonly userById: (id: string) => User | undefined;
}

/** The initial value for this part of the state */
export const state: State = {
  users: {},
  get userById() {
    return (id: string) => this.users[id];
  }
};
