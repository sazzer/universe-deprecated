import * as health from "./infrastructure/health/loader";
import * as server from "./infrastructure/server";

import pino from "pino";

/** The logger to use */
const LOG = pino({
  name: "universe:infrastructure:server"
});

/** The ordered list of loaders to use to construct the application */
const loaders = [server.build, health.build];

/**
 * Actually build the application
 */
export function build() {
  LOG.info("Building Universe...");
  const loader: any = {};
  loaders.forEach(builder => builder(loader));

  return server.getServer(loader);
}
