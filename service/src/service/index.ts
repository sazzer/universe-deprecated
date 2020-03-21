import Koa from "koa";
import Router from "@koa/router";
import cors from "@koa/cors";
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
  app.use(cors());

  const router = new Router();

  app.use(
    koaLogger({
      logger: pino({
        name: "universe:service:access",
        prettyPrint: process.env.NODE_ENV === "development"
      })
    })
  );

  router.get("/", async ctx => {
    ctx.body = "Hello World";
  });

  app.use(router.routes()).use(router.allowedMethods());

  return app;
}
