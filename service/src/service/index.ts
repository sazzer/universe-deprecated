import Koa from "koa";
import Router from "@koa/router";
import cors from "@koa/cors";
import koaBody from "koa-body";
import koaLogger from "koa-pino-logger";
import pino from "pino";
import responseTime from "koa-response-time";

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
  app.use(responseTime());
  app.use(cors());
  app.use(koaBody());

  const router = new Router();

  app.use(
    koaLogger({
      logger: pino({
        name: "universe:service:access",
        prettyPrint: process.env.NODE_ENV === "development"
      })
    })
  );

  router.post("/", async ctx => {
    LOG.info("Request", ctx.request.query);
    ctx.body = "Hello World";
  });

  app.use(router.routes()).use(router.allowedMethods());

  return app;
}
