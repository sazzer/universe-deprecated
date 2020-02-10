import { namespaced } from 'overmind/config';
import { createHook } from 'overmind-react'
import { IConfig } from 'overmind'
import * as login from '../login';

export const config = namespaced({
  login,
});

export const useOvermind = createHook<typeof config>();

declare module 'overmind' {
  // tslint:disable:interface-name
  interface Config extends IConfig<typeof config> { }
}
