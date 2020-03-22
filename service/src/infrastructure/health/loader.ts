import { ServerLoader, getRouter } from "../server";

import { Healthcheck } from "./check";
import { checkHealth } from "./http";

/** Key into the loader for the individual healthchecks */
const HEALTHCHECKS_KEY = Symbol("health:checks");

/** The loader as seen from the Healthchecks point of view */
export interface HealthLoader {
  [HEALTHCHECKS_KEY]: { [name: string]: Healthcheck };
}

/**
 * Add a new healthcheck to the loader
 * @param loader the loader to register the healthcheck with
 * @param name the name of the healthcheck
 * @param check the actual healthcheck
 */
export function addHealthCheck<T extends HealthLoader>(
  loader: T,
  name: string,
  check: Healthcheck
) {
  if (loader[HEALTHCHECKS_KEY] === undefined) {
    loader[HEALTHCHECKS_KEY] = {};
  }
  loader[HEALTHCHECKS_KEY][name] = check;
}

/**
 * Actually build the healthchecks
 * @param loader the loader to populate
 */
export function build<T extends HealthLoader & ServerLoader>(loader: T) {
  const router = getRouter(loader);

  router.get("/health", checkHealth());
}
