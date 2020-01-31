import axios, { Method, AxiosResponse } from 'axios';
import * as rfc6570 from 'rfc6570-expand';

declare global {
  interface Window {
    _UNIVERSE_CONFIG: any;
  }
}

/**
 * The shape of the request that we want to make
 */
export interface Request {
  url: string;
  urlParams?: any;
  method?: Method;
  headers?: any;
  data?: any;
}

export type Response<T> = AxiosResponse<T>;

/**
 * Shape of a Problem response from the serve
 */
export interface Problem {
  type: string,
  status: number,
}

/**
 * Response that is thrown if we get an RFC-7807 Problem back from the server
 */
export class ProblemResponse<T extends Problem> {
  readonly problem: T;
  readonly status: number;
  readonly headers: any;

  constructor(problem: T, status: number, headers: any) {
    this.problem = problem;
    this.status = status;
    this.headers = headers;
  }
}

/**
 * Build an API requester to make API calls with
 */
export async function request<T>(request: Request): Promise<Response<T>> {
  const serviceUrl = process.env.REACT_APP_SERVICE_URL || ((window || {})._UNIVERSE_CONFIG || {}).serviceUrl;

  const template = rfc6570.init(request.url);
  const expandedUri = template.expand(request.urlParams || {});
  try {
    return await axios.request({
      baseURL: serviceUrl,
      timeout: 20000,
      url: expandedUri,
      method: request.method,
      headers: request.headers,
      data: request.data,
    });
  } catch (e) {
    if (e.response) {
      const response = e.response as AxiosResponse<Problem>;
      if (response.headers['content-type'] === 'application/problem+json') {
        throw new ProblemResponse(response.data, response.status, response.headers);
      }
    }
    console.error('Unexpected error making API request', e);
    throw e;
  }
}
