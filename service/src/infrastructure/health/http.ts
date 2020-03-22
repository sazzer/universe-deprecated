import { Context } from "koa";

/**
 * Actually check the health of the system
 * @param ctx the Koa context
 */
export async function checkHealth(ctx: Context) {
  ctx.status = 503;
  ctx.body = {
    status: "FAIL"
  };
}
