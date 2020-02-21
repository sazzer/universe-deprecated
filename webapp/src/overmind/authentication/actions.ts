import { Action } from "overmind";

export const logout: Action = ({ state }) => {
  state.authentication.accessToken = null;
  state.authentication.userId = null;
};

export interface LoginValue {
  accessToken: string;
  expires: string;
  userId: string;
}

export const login: Action<LoginValue> = ({ state }, details: LoginValue) => {
  state.authentication.accessToken = {
    accessToken: details.accessToken,
    expires: details.expires
  };
  state.authentication.userId = details.userId;
};
