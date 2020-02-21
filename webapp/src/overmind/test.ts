import { createOvermindMock, OvermindMock, Config } from "overmind";
import { config } from "../overmind";
import { cloneDeep, merge } from "lodash";

/**
 * NestedPartial is stolen from Overmind to represent a part of the Overmind structure
 */
export declare type NestedPartial<T> = T extends Function
  ? T
  : Partial<
      {
        [P in keyof T]: NestedPartial<T[P]>;
      }
    >;

/**
 * Create a test instance of the Overmind to work with
 * @param mockEffects The mock effects to wire in
 * @param initialState The initial state to wire in
 */
export function createTestOvermind(
  mockEffects?: NestedPartial<Config["effects"]>,
  initialState?: NestedPartial<Config["state"]>
): OvermindMock<Config> {
  const mergedState = merge(cloneDeep(config), { state: initialState });
  return createOvermindMock(mergedState, mockEffects);
}
