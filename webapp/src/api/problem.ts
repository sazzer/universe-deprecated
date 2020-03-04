/**
 * Shape of a Problem response from the serve
 */
export interface Problem {
  type: string;
  title: string;
  status: number;
}

/**
 * Response that is thrown if we get an RFC-7807 Problem back from the server
 */
export class ProblemResponse<T extends Problem> extends Error {
  readonly problem: T;
  readonly status: number;
  readonly headers: any;

  /**
   * Construct the problem response
   * @param problem The actual problem
   * @param status The status code of the problem
   * @param headers Any headers from the response
   */
  constructor(problem: T, status: number, headers: any) {
    super(problem.title);
    this.problem = problem;
    this.status = status;
    this.headers = headers;
  }
}
