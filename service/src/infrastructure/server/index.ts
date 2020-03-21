import Koa from "koa";
import Router from "@koa/router";
import cors from "@koa/cors";
import koaBody from "koa-body";
import koaLogger from "koa-pino-logger";
import pino from "pino";
import responseTime from "koa-response-time";

/** The logger to use */
const LOG = pino({
  name: "universe:infrastructure:server"
});

/** Key into the loader for the router */
const ROUTER_KEY = Symbol("server:router");

/** Key into the loader for the Koa app itself */
const APP_KEY = Symbol("server:app");

/** The loader as seen from the Servers point of view */
export interface ServerLoader {
  [ROUTER_KEY]: Router;
  [APP_KEY]: Koa;
}

/**
 * Get the router out of the loader
 * @param loader the loader
 */
export function getRouter(loader: ServerLoader) {
  return loader[ROUTER_KEY];
}

/**
 * Get the web server out of the loader
 * @param loader the loader
 */
export function getServer(loader: ServerLoader) {
  return loader[APP_KEY];
}

/**
 * Populate the loader with the details of the web server
 * @param loader the loader to populate
 */
export function build(loader: ServerLoader) {
  LOG.debug("Building web server...");

  const app = new Koa();
  app.use(responseTime());
  app.use(cors());
  app.use(koaBody());

  app.use(
    koaLogger({
      logger: pino({
        name: "universe:infrastructure:server:access"
      })
    })
  );

  const router = new Router();

  app.use(router.routes()).use(router.allowedMethods());

  loader[ROUTER_KEY] = router;
  loader[APP_KEY] = app;
}
