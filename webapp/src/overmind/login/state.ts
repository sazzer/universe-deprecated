import { statemachine, Statemachine } from "overmind";

/** Possible modes that the login process can be in */
type Mode = "initial" | "registering" | "authenticating";

/** The shape of this part of the state */
interface State {
  /** The username that is logging in */
  username: string | null;
  /** The current mode of the login process */
  mode: Statemachine<Mode>;
  /** whether the login process is currently loading data from the server */
  loading: boolean;
  /** The most recent error, if there was one */
  error: string | null;
}

/** The initial value for this part of the state */
export const state: State = {
  mode: statemachine<Mode>({
    initial: "initial",
    states: {
      initial: ["initial", "registering", "authenticating"],
      registering: ["initial"],
      authenticating: ["initial"]
    }
  }),
  loading: false,
  username: null,
  error: null
};
