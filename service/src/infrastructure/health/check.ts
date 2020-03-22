/** Indication that the healthcheck passed */
export const HEALTH_PASS = "PASS";

/** Indication that the healthcheck failed */
export const HEALTH_FAIL = "FAIL";

/** The status of a healthcheck */
export type HealthStatus = "PASS" | "FAIL";

/** The result of a healthcheck */
export interface HealthResult {
  status: HealthStatus;
  message: string | undefined;
}

/** The actual means to check the health of something */
export type Healthcheck = () => Promise<HealthResult>;
