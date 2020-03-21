import Koa from "koa";
import koaLogger from "koa-pino-logger";
import pino from "pino";

/** The logger to use */
const LOG = pino({
  name: "universe:service",
  prettyPrint: process.env.NODE_ENV === "development"
});

/**
 * Built the actual service that will do all the work
 */
export function buildService() {
  LOG.debug("Building universe...");
  const app = new Koa();
  app.use(
    koaLogger({
      logger: pino({
        name: "universe:service:access",
        prettyPrint: process.env.NODE_ENV === "development"
      })
    })
  );

  app.use(async ctx => {
    ctx.body = "Hello World";
  });

  return app;
}
