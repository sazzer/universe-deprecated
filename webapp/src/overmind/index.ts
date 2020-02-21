import { namespaced } from "overmind/config";
import { createHook } from "overmind-react";
import { IConfig } from "overmind";
import * as login from "./login";
import * as authentication from "./authentication";
import * as users from "./users";

declare module "overmind" {
  // tslint:disable:interface-name
  interface Config extends IConfig<typeof config> {}
}

/** The overmind configuration */
export const config = namespaced({
  login,
  authentication,
  users
});

/** React Hook for interacting with Overmind */
export const useOvermind = createHook<typeof config>();
