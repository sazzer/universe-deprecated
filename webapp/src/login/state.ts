import { statemachine, Statemachine } from 'overmind';

type Mode = 'initial' | 'loading' | 'registering' | 'authenticating';

type State = {
  username: string | null,
  mode: Statemachine<Mode>,
  error: string | null,

  readonly isLoading: boolean,
  readonly isRegistering: boolean,
  readonly isAuthenticating: boolean,
};


export const state: State = {
  mode: statemachine<Mode>({
    initial: 'initial',
    states: {
      initial: ['loading'],
      loading: ['initial', 'registering', 'authenticating'],
      registering: ['initial'],
      authenticating: ['initial'],
    }
  }),
  username: null,
  error: null,

  get isRegistering() {
    return this.mode.current === 'registering';
  },

  get isAuthenticating() {
    return this.mode.current === 'authenticating';
  },

  get isLoading() {
    return this.mode.current === 'loading';
  },
};
