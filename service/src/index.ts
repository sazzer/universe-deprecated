import "dotenv/config";

import config from "config";
import pino from "pino";

/** The logger to use */
const LOG = pino({
  name: "universe",
  prettyPrint: process.env.NODE_ENV === "development"
});

LOG.info("Config", config);
