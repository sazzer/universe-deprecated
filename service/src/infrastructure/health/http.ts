import { Context } from "koa";

/**
 * Actually check the health of the system
 */
export function checkHealth() {
  return async function checkHealth(ctx: Context) {
    ctx.status = 503;
    ctx.body = {
      status: "FAIL"
    };
  };
}
