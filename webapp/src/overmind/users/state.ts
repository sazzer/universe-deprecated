import { User } from "./model";

/** The shape of this part of the state */
interface State {
  /** The set of all known users */
  users: { [userId: string]: User };
}

/** The initial value for this part of the state */
export const state: State = {
  users: {}
};
