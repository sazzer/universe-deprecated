import { statemachine, Statemachine } from "overmind";

type Mode = "initial" | "registering" | "authenticating";

type State = {
  username: string | null;
  mode: Statemachine<Mode>;
  loading: boolean;
  error: string | null;
};

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
