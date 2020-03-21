import "dotenv/config";

import { buildService } from "./service";
import config from "config";
import pino from "pino";

/** The logger to use */
const LOG = pino({
  name: "universe",
  prettyPrint: process.env.NODE_ENV === "development"
});

const service = buildService();

LOG.info("Starting universe...", { port: config.get("http.port") });
service.listen(config.get("http.port"));
