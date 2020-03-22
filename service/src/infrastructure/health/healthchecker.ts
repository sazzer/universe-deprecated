import {
  HEALTH_FAIL,
  HEALTH_PASS,
  HealthResult,
  HealthStatus,
  Healthcheck
} from "./check";

/** The health of the overall system */
interface SystemHealth {
  status: HealthStatus;
  components: { [name: string]: HealthResult };
}

/**
 * The means to actually check the health of the system
 */
export class Healthchecker {
  /** The checks to perform */
  private checks: { [name: string]: Healthcheck };

  /**
   * Construct the healthchecker
   * @param checks The checks to perform
   */
  constructor(checks: { [name: string]: Healthcheck }) {
    this.checks = checks;
  }

  /**
   * Check the overall health of the system
   */
  async check(): Promise<SystemHealth> {
    const result: SystemHealth = {
      status: HEALTH_PASS,
      components: {}
    };

    const promises = Object.entries(this.checks).map(async ([key, value]) => {
      const health = await value();
      result.components[key] = health;
    });
    await Promise.all(promises);

    const failure = Object.values(result.components)
      .map(result => result.status)
      .some(status => status === HEALTH_FAIL);

    result.status = failure ? HEALTH_FAIL : HEALTH_PASS;

    return result;
  }
}
