import { namespaced } from "overmind/config";
import { createHook } from "overmind-react";
import { IConfig } from "overmind";
import * as login from "../login";
import * as authentication from "../authentication";

export const config = namespaced({
  login,
  authentication
});

export const useOvermind = createHook<typeof config>();

declare module "overmind" {
  // tslint:disable:interface-name
  interface Config extends IConfig<typeof config> {}
}
