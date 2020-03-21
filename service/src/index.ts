import "dotenv/config";

import { build } from "./loaders";
import config from "config";
import pino from "pino";

/** The logger to use */
const LOG = pino({
  name: "universe"
});

const service = build();

LOG.info("Starting universe...", { port: config.get("http.port") });
service.listen(config.get("http.port"));
